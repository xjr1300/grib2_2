use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::readers::coordinates::Coordinate;
use crate::readers::records::{Grib2RecordIter, Grib2RecordIterBuilder};
use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_50009, Section5_200u16, Section6,
    Section7_200, Section8,
};
use crate::{Grib2Error, Grib2Result};

/// 降水短時間予報ファイルリーダー
pub struct FPrrReader {
    /// ファイルパス
    pub path: PathBuf,
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第3節:格子系定義節
    section3: Section3_0,
    /// 第4節:プロダクト定義節から第7節:資料節
    forecasts: [FPrrForecast; 6],
    /// 第8節:終端節
    section8: Section8,
    /// 予想降水量の座標
    coordinates: Vec<Coordinate>,
    /// 予想降水量
    precipitations: [HashMap<Coordinate, Option<u16>>; 6],
}

pub struct FPrrForecast {
    /// 第4節:プロダクト定義節
    pub section4: Section4_50009,
    /// 第5節:資料表現節
    pub section5: Section5_200u16,
    /// 第6節:ビットマップ節
    pub section6: Section6,
    /// 第7節:資料節
    pub section7: Section7_200,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FPrrHour {
    /// 1時間予想
    Hour1 = 1,
    /// 2時間予想
    Hour2 = 2,
    /// 3時間予想
    Hour3 = 3,
    /// 4時間予想
    Hour4 = 4,
    /// 5時間予想
    Hour5 = 5,
    /// 6時間予想
    Hour6 = 6,
}

impl FPrrReader {
    /// 降水短時間予報ファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 降水短時間予報ファイルのパス
    ///
    /// # 戻り値
    ///
    /// * 降水短時間予報ファイルリーダー
    pub fn new<P: AsRef<Path>>(path: P) -> Grib2Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Grib2Error::FileDoesNotExist);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;
        let mut reader = BufReader::new(file);
        let section0 = Section0::from_reader(&mut reader)?;
        let section1 = Section1::from_reader(&mut reader)?;
        let section2 = Section2;
        let section3 = Section3_0::from_reader(&mut reader)?;
        let forecasts = [
            FPrrForecast::from_reader(&mut reader)?,
            FPrrForecast::from_reader(&mut reader)?,
            FPrrForecast::from_reader(&mut reader)?,
            FPrrForecast::from_reader(&mut reader)?,
            FPrrForecast::from_reader(&mut reader)?,
            FPrrForecast::from_reader(&mut reader)?,
        ];
        let section8 = Section8::from_reader(&mut reader)?;

        // 予想降水量を読み込み
        let precipitations = [
            read_precipitation(path, &section3, &forecasts[0])?,
            read_precipitation(path, &section3, &forecasts[1])?,
            read_precipitation(path, &section3, &forecasts[2])?,
            read_precipitation(path, &section3, &forecasts[3])?,
            read_precipitation(path, &section3, &forecasts[4])?,
            read_precipitation(path, &section3, &forecasts[5])?,
        ];
        // 予想降水量を記録している座標
        let mut coordinates = precipitations[0]
            .keys()
            .map(|k| k.to_owned())
            .collect::<Vec<Coordinate>>();
        coordinates.sort();

        Ok(Self {
            path: path.to_path_buf(),
            section0,
            section1,
            section2,
            section3,
            forecasts,
            section8,
            coordinates,
            precipitations,
        })
    }

    /// 開いている土砂災害警戒判定メッシュファイルのパスを返す。
    ///
    /// # 戻り値
    ///
    /// * 開いている土砂災害警戒判定メッシュファイルのパス
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 第0節:指示節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第0節:指示節
    pub fn section0(&self) -> &Section0 {
        &self.section0
    }

    /// 第1節:識別節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第1節:識別節
    pub fn section1(&self) -> &Section1 {
        &self.section1
    }

    /// 第2節:地域使用節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第2節:地域使用節
    pub fn section2(&self) -> &Section2 {
        &self.section2
    }

    /// 第3節:格子系定義節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第3節:格子系定義節
    pub fn section3(&self) -> &Section3_0 {
        &self.section3
    }

    /// 第4節:プロダクト定義節から第7節:資料節までを返す。
    ///
    /// # 引数
    ///
    /// * `hour` - 予報時間
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    pub fn forecast(&self, hour: FPrrHour) -> &FPrrForecast {
        &self.forecasts[hour as u8 as usize - 1]
    }

    /// 第8節:終端節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub fn section8(&self) -> &Section8 {
        &self.section8
    }

    /// 指定された予報時間のレコードを反復処理するイテレーターを返す。
    ///
    /// # 引数
    ///
    /// * `hour` - レコードを取得する予報時間
    ///
    /// # 戻り値
    ///
    /// * 指定された予報時間のレコードを反復処理するイテレーター
    pub fn record_iter(&mut self, hour: FPrrHour) -> Grib2Result<Grib2RecordIter<'_, File, u16>> {
        // 降水短時間予報ファイルを開く
        if !self.path.is_file() {
            return Err(Grib2Error::FileDoesNotExist);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;
        let mut reader = BufReader::new(file);

        // ランレングス符号の開始位置にファイルポインターを移動
        let forecast = self.forecast(hour);
        reader
            .seek(SeekFrom::Start(
                forecast.section7.run_length_position() as u64
            ))
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;

        // イテレーターを構築
        Grib2RecordIterBuilder::new()
            .reader(reader)
            .total_bytes(forecast.section7.run_length_bytes())
            .number_of_points(self.section3.number_of_data_points())
            .lat_max(self.section3.lat_of_first_grid_point())
            .lon_min(self.section3.lon_of_first_grid_point())
            .lon_max(self.section3.lon_of_last_grid_point())
            .lat_inc(self.section3.j_direction_increment())
            .lon_inc(self.section3.i_direction_increment())
            .nbit(forecast.section5.bits_per_value() as u16)
            .maxv(forecast.section5.max_level_value())
            .level_values(forecast.section5.level_values())
            .build()
    }

    /// 予想降水量を反復操作するイテレーターを返す。
    ///
    /// # 引数
    ///
    /// * `hour` - 予想降水量の時間
    ///
    /// # 戻り値
    ///
    /// * 予想降水量を反復操作するイテレーター
    pub fn prep_iter(&self) -> FPrrPrepIterator {
        FPrrPrepIterator {
            index: 0,
            coordinates: &self.coordinates,
            precipitations: &self.precipitations,
        }
    }
}

pub struct FPrrPrep {
    /// 緯度
    pub lat: u32,
    /// 経度
    pub lon: u32,
    /// 1時間予報降水量
    pub hour1: Option<u16>,
    /// 2時間予報降水量
    pub hour2: Option<u16>,
    /// 3時間予報降水量
    pub hour3: Option<u16>,
    /// 4時間予報降水量
    pub hour4: Option<u16>,
    /// 5時間予報降水量
    pub hour5: Option<u16>,
    /// 6時間予報降水量
    pub hour6: Option<u16>,
}

pub struct FPrrPrepIterator<'a> {
    /// 次に返す座標のインデックス
    index: usize,
    /// 座標を格納したスライスへの参照
    coordinates: &'a [Coordinate],
    /// キーと値に座標と予想降水量を格納したハッシュマップを格納したスライスへの参照
    precipitations: &'a [HashMap<Coordinate, Option<u16>>],
}

impl<'a> Iterator for FPrrPrepIterator<'a> {
    type Item = FPrrPrep;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index < self.coordinates.len() {
            true => {
                let coordinate = self.coordinates[self.index];
                let hour1 = self.precipitations[0].get(&coordinate).unwrap();
                let hour2 = self.precipitations[1].get(&coordinate).unwrap();
                let hour3 = self.precipitations[2].get(&coordinate).unwrap();
                let hour4 = self.precipitations[3].get(&coordinate).unwrap();
                let hour5 = self.precipitations[4].get(&coordinate).unwrap();
                let hour6 = self.precipitations[4].get(&coordinate).unwrap();
                let prep = FPrrPrep {
                    lat: coordinate.lat,
                    lon: coordinate.lon,
                    hour1: *hour1,
                    hour2: *hour2,
                    hour3: *hour3,
                    hour4: *hour4,
                    hour5: *hour5,
                    hour6: *hour6,
                };
                self.index += 1;
                Some(prep)
            }
            _ => None,
        }
    }
}

/// 予想降水量を読み込む。
///
/// # 引数
///
/// * `path` - 降水短時間保養ファイルのパス
/// * `forecasts` - 第4節:プロダクト定義節から第7節:資料節
///
/// # 戻り値
///
/// * キーと値に緯度と経度と予想降水量を持つハッシュマップ
fn read_precipitation<P: AsRef<Path>>(
    path: P,
    section3: &Section3_0,
    forecast: &FPrrForecast,
) -> Grib2Result<HashMap<Coordinate, Option<u16>>> {
    // ランレングス符号の開始位置にファイルポインターを移動
    let file = OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(|e| Grib2Error::Unexpected(e.into()))?;
    let mut reader = BufReader::new(file);
    reader
        .seek(SeekFrom::Start(
            forecast.section7.run_length_position() as u64
        ))
        .map_err(|e| Grib2Error::Unexpected(e.into()))?;

    // イテレーターを構築
    let iter = Grib2RecordIterBuilder::new()
        .reader(reader)
        .total_bytes(forecast.section7.run_length_bytes())
        .number_of_points(section3.number_of_data_points())
        .lat_max(section3.lat_of_first_grid_point())
        .lon_min(section3.lon_of_first_grid_point())
        .lon_max(section3.lon_of_last_grid_point())
        .lat_inc(section3.j_direction_increment())
        .lon_inc(section3.i_direction_increment())
        .nbit(forecast.section5.bits_per_value() as u16)
        .maxv(forecast.section5.max_level_value())
        .level_values(forecast.section5.level_values())
        .build()?;

    // 指定された予報時間の降水量を読み込み
    let mut precipitations = HashMap::new();
    for record in iter.flatten() {
        let coordinate: Coordinate = record.into();
        precipitations.insert(coordinate, record.value);
    }

    Ok(precipitations)
}

impl FPrrForecast {
    /// 第4節:プロダクト定義節から第7節:資料節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - 土砂災害警戒判定メッシュファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        let section4 = Section4_50009::from_reader(reader)?;
        let section5 = Section5_200u16::from_reader(reader)?;
        let section6 = Section6::from_reader(reader)?;
        let section7 = Section7_200::from_reader(reader)?;

        Ok(Self {
            section4,
            section5,
            section6,
            section7,
        })
    }
}
impl TryFrom<u8> for FPrrHour {
    type Error = Grib2Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hour1),
            2 => Ok(Self::Hour2),
            3 => Ok(Self::Hour3),
            4 => Ok(Self::Hour4),
            5 => Ok(Self::Hour5),
            6 => Ok(Self::Hour6),
            _ => Err(Grib2Error::ConvertError(
                format!("`{value}`を`FPrrHour`型に変換できません。").into(),
            )),
        }
    }
}
