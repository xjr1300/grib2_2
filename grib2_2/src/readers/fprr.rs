use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use crate::readers::records::Grib2RecordIterBuilder;
use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_50009, Section5_200u16, Section6,
    Section7_200, Section8,
};
use crate::readers::ForecastHour;
use crate::{Grib2Error, Grib2Result};

/// 降水短時間予報ファイルリーダー
pub struct FPrrReader {
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第3節:格子系定義節
    section3: Section3_0,
    /// 第4節:プロダクト定義節から第7節:資料節
    fprr_sections: [FPrrSections; 6],
    /// 第8節:終端節
    section8: Section8,
    /// 予想降水量
    preps: [Vec<Option<u16>>; 6],
}

pub struct FPrrSections {
    /// 第4節:プロダクト定義節
    pub section4: Section4_50009,
    /// 第5節:資料表現節
    pub section5: Section5_200u16,
    /// 第6節:ビットマップ節
    pub section6: Section6,
    /// 第7節:資料節
    pub section7: Section7_200,
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
        let fprr_sections = [
            FPrrSections::from_reader(&mut reader)?,
            FPrrSections::from_reader(&mut reader)?,
            FPrrSections::from_reader(&mut reader)?,
            FPrrSections::from_reader(&mut reader)?,
            FPrrSections::from_reader(&mut reader)?,
            FPrrSections::from_reader(&mut reader)?,
        ];
        let section8 = Section8::from_reader(&mut reader)?;

        // 予想降水量を読み込み
        let preps = [
            read_preps(&mut reader, &section3, &fprr_sections[0])?,
            read_preps(&mut reader, &section3, &fprr_sections[1])?,
            read_preps(&mut reader, &section3, &fprr_sections[2])?,
            read_preps(&mut reader, &section3, &fprr_sections[3])?,
            read_preps(&mut reader, &section3, &fprr_sections[4])?,
            read_preps(&mut reader, &section3, &fprr_sections[5])?,
        ];

        Ok(Self {
            section0,
            section1,
            section2,
            section3,
            fprr_sections,
            section8,
            preps,
        })
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
    pub fn fprr_sections(&self, hour: ForecastHour) -> &FPrrSections {
        &self.fprr_sections[hour as u8 as usize - 1]
    }

    /// 第8節:終端節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub fn section8(&self) -> &Section8 {
        &self.section8
    }

    /// 予想降水量を反復操作するイテレーターを返す。
    ///
    /// # 引数
    ///
    /// # 戻り値
    ///
    /// * 予想降水量を反復操作するイテレーター
    pub fn value_iter(&self) -> FPrrValueIterator {
        FPrrValueIterator::new(
            self.section3.lat_of_first_grid_point(),
            self.section3.lon_of_first_grid_point(),
            self.section3.lon_of_last_grid_point(),
            self.section3.j_direction_increment(),
            self.section3.i_direction_increment(),
            &self.preps,
        )
    }
}

pub struct FPrrValue {
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

pub struct FPrrValueIterator<'a> {
    /// 格子点の緯度
    lat: u32,
    /// 格子点の経度
    lon: u32,
    /// 最初の格子点の経度
    lon_min: u32,
    /// 最後の格子点の経度
    lon_max: u32,
    /// 緯度方向の増分
    lat_inc: u32,
    /// 緯度方向の増分
    lon_inc: u32,
    /// 次に返す予想降水量のインデックス
    index: usize,
    /// 予想降水量を予想時間でインデックス化した配列
    preps: &'a [Vec<Option<u16>>; 6],
}

impl<'a> FPrrValueIterator<'a> {
    /// コンストラクタ
    ///
    /// # 引数
    ///
    /// * `lat` - 最初の格子点の緯度
    /// * `lon` - 最初の格子点の経度
    /// * `lon_max` - 最後の格子点の経度
    /// * `lat_inc` - 緯度方向の増分
    /// * `lon_inc` - 経度方向の増分
    /// * `preps` - 予想降水量を予想時間でインデックス化した配列
    fn new(
        lat: u32,
        lon: u32,
        lon_max: u32,
        lat_inc: u32,
        lon_inc: u32,
        preps: &'a [Vec<Option<u16>>; 6],
    ) -> Self {
        Self {
            lat,
            lon,
            lon_min: lon,
            lon_max,
            lat_inc,
            lon_inc,
            index: 0,
            preps,
        }
    }
}

impl<'a> Iterator for FPrrValueIterator<'a> {
    type Item = FPrrValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.preps[0].len() <= self.index {
            return None;
        }
        let result = FPrrValue {
            lat: self.lat,
            lon: self.lon,
            hour1: self.preps[0][self.index],
            hour2: self.preps[1][self.index],
            hour3: self.preps[2][self.index],
            hour4: self.preps[3][self.index],
            hour5: self.preps[4][self.index],
            hour6: self.preps[5][self.index],
        };
        self.index += 1;
        self.lon += self.lon_inc;
        if self.lon_max < self.lon {
            self.lat -= self.lat_inc;
            self.lon = self.lon_min;
        }

        Some(result)
    }
}

/// 予想降水量を読み込む。
///
/// # 引数
///
/// * `reader` - 降水短時間予報リーダー
/// * `section3` - 第3節:格子系定義節
/// * `fprr_sections` - 第4節:プロダクト定義節から第7節:資料節
///
/// # 戻り値
///
/// * 座標値と予想降水量を格納したタプル
fn read_preps(
    reader: &mut BufReader<File>,
    section3: &Section3_0,
    fprr_sections: &FPrrSections,
) -> Grib2Result<Vec<Option<u16>>> {
    // 第7節のランレングス圧縮オクテット列の開始位置にファイルポインターを移動
    reader
        .seek(SeekFrom::Start(
            fprr_sections.section7.run_length_position() as u64,
        ))
        .map_err(|e| Grib2Error::Unexpected(e.into()))?;

    // イテレーターを構築
    let iter = Grib2RecordIterBuilder::new()
        .reader(reader)
        .total_bytes(fprr_sections.section7.run_length_bytes())
        .number_of_points(section3.number_of_data_points())
        .lat_max(section3.lat_of_first_grid_point())
        .lon_min(section3.lon_of_first_grid_point())
        .lon_max(section3.lon_of_last_grid_point())
        .lat_inc(section3.j_direction_increment())
        .lon_inc(section3.i_direction_increment())
        .nbit(fprr_sections.section5.bits_per_value() as u16)
        .maxv(fprr_sections.section5.max_level_value())
        .level_values(fprr_sections.section5.level_values())
        .build()?;
    // 予想降水量を読み込み
    let mut precipitations = vec![];
    for record in iter.flatten() {
        precipitations.push(record.value);
    }

    Ok(precipitations)
}

impl FPrrSections {
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
