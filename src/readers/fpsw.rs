use std::fs::OpenOptions;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

use crate::readers::records::Grib2RecordIterBuilder;
use crate::readers::sections::{Section0, Section1, Section2, Section3_0, Section8};
use crate::readers::{ForecastHour, ForecastRange};
use crate::readers::{PswSections, PswTank};
use crate::{Grib2Error, Grib2Result};

/// 土壌雨量指数予想値ファイルリーダー
pub struct FPswReader {
    /// 予想時間範囲
    forecast_range: ForecastRange,
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第３節:格子系定義節
    section3: Section3_0,
    /// 第4節:プロダクト定義節から第7節:資料節を1時間から6時間予想別タンク別に格納したベクター
    /// 1時間予想
    ///     インデックス0: 全タンク
    ///     インデックス1: 第一タンク
    ///     インデックス2: 第二タンク
    /// 2時間予想
    ///     ...
    /// 3時間予想
    ///     ...
    /// 4時間予想
    ///     ...
    /// 5時間予想
    ///     ...
    /// 6時間予想
    ///     インデックス0: 全タンク
    ///     インデックス1: 第一タンク
    ///     インデックス2: 第二タンク
    fpsw_sections: Vec<[PswSections; 3]>,
    /// 第８節:終端節
    section8: Section8,
    /// 1時間から6時間までの土壌雨量指数予想値をタンク別に格納したベクター
    tank_values: Vec<TankValue>,
}

/// タンク別土壌雨量指数予想値
struct TankValue {
    /// 1時間土壌雨量指数予想値
    hour1: Vec<Option<u16>>,
    /// 2時間土壌雨量指数予想値
    hour2: Vec<Option<u16>>,
    /// 3時間土壌雨量指数予想値
    hour3: Vec<Option<u16>>,
    /// 4時間土壌雨量指数予想値
    hour4: Option<Vec<Option<u16>>>,
    /// 5時間土壌雨量指数予想値
    hour5: Option<Vec<Option<u16>>>,
    /// 6時間土壌雨量指数予想値
    hour6: Option<Vec<Option<u16>>>,
}

impl FPswReader {
    /// 土壌雨量指数ファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 土壌雨量指数ファイルのパス
    ///
    /// # 戻り値
    ///
    /// * 土壌雨量指数リーダー
    pub fn new<P: AsRef<Path>>(path: P, forecast_range: ForecastRange) -> Grib2Result<Self> {
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
        let mut fpsw_sections = vec![];
        for _ in 0..(forecast_range as u8) {
            fpsw_sections.push([
                PswSections::from_reader(&mut reader)?,
                PswSections::from_reader(&mut reader)?,
                PswSections::from_reader(&mut reader)?,
            ]);
        }
        let section8 = Section8::from_reader(&mut reader)?;

        let mut tank_values = vec![];
        for tank in [PswTank::All, PswTank::Tank1, PswTank::Tank2] {
            tank_values.push(TankValue::from_reader(
                &mut reader,
                tank,
                &section3,
                &fpsw_sections,
            )?);
        }

        Ok(Self {
            forecast_range,
            section0,
            section1,
            section2,
            section3,
            fpsw_sections,
            section8,
            tank_values,
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

    /// 指定された予想時間とタンクの第4節:プロダクト定義節から第7節:資料節を返す。
    ///
    /// # 引数
    ///
    /// * `hour` - 予想時間
    /// * `tank` - タンク
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    pub fn fpsw_sections(&self, hour: ForecastHour, tank: PswTank) -> Grib2Result<&PswSections> {
        if !self.forecast_range.contains(hour) {
            return Err(Grib2Error::RuntimeError(
                format!(
                    "土壌雨量指数予想値ファイルは{}時間の予想を記録していません。",
                    hour as u8
                )
                .into(),
            ));
        }

        Ok(&self.fpsw_sections[hour as u8 as usize - 1][tank as u8 as usize])
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
    /// * `tank` - タンク
    ///
    /// # 戻り値
    ///
    /// * 予想降水量を反復操作するイテレーター
    pub fn value_iter(&self, tank: PswTank) -> FPswIndexIterator {
        FPswIndexIterator::new(
            self.section3.lat_of_first_grid_point(),
            self.section3.lon_of_first_grid_point(),
            self.section3.lon_of_last_grid_point(),
            self.section3.j_direction_increment(),
            self.section3.i_direction_increment(),
            &self.tank_values[tank as u8 as usize],
        )
    }
}

/// タンク土壌雨量指数予想値
pub struct FPswIndex {
    /// 緯度
    pub lat: u32,
    /// 経度
    pub lon: u32,
    /// 1時間土壌雨量指数予想値
    pub hour1: Option<u16>,
    /// 2時間土壌雨量指数予想値
    pub hour2: Option<u16>,
    /// 3時間土壌雨量指数予想値
    pub hour3: Option<u16>,
    /// 4時間土壌雨量指数予想値
    pub hour4: Option<u16>,
    /// 5時間土壌雨量指数予想値
    pub hour5: Option<u16>,
    /// 6時間土壌雨量指数予想値
    pub hour6: Option<u16>,
}

/// タンク土壌雨量指数予想値を反復処理するイテレーター
pub struct FPswIndexIterator<'a> {
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
    /// 次に返す土壌雨量指数予想値のインデックス
    index: usize,
    /// 土壌雨量指数予想値
    tank_values: &'a TankValue,
}

impl<'a> FPswIndexIterator<'a> {
    /// コンストラクタ
    ///
    /// # 引数
    ///
    /// * `lat` - 最初の格子点の緯度
    /// * `lon` - 最初の格子点の経度
    /// * `lon_max` - 最後の格子点の経度
    /// * `lat_inc` - 緯度方向の増分
    /// * `lon_inc` - 経度方向の増分
    /// * `tank_values` - タンクの土壌雨量指数予想値
    fn new(
        lat: u32,
        lon: u32,
        lon_max: u32,
        lat_inc: u32,
        lon_inc: u32,
        tank_values: &'a TankValue,
    ) -> Self {
        Self {
            lat,
            lon,
            lon_min: lon,
            lon_max,
            lat_inc,
            lon_inc,
            index: 0,
            tank_values,
        }
    }
}

impl<'a> Iterator for FPswIndexIterator<'a> {
    type Item = FPswIndex;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tank_values.hour1.len() <= self.index {
            return None;
        }
        let (hour4, hour5, hour6) = if self.tank_values.hour4.is_some() {
            (
                self.tank_values.hour4.as_ref().unwrap()[self.index],
                self.tank_values.hour5.as_ref().unwrap()[self.index],
                self.tank_values.hour6.as_ref().unwrap()[self.index],
            )
        } else {
            (None, None, None)
        };
        let result = FPswIndex {
            lat: self.lat,
            lon: self.lon,
            hour1: self.tank_values.hour1[self.index],
            hour2: self.tank_values.hour2[self.index],
            hour3: self.tank_values.hour3[self.index],
            hour4,
            hour5,
            hour6,
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

/// タンクの土壌雨量指数予想値を読み込む。
///
/// # 引数
///
/// * `reader` - 土壌雨量指数予想値ファイルリーダー
/// * `section3` - 第3節:格子系定義節
/// * `fpsw_sections` - 読み込む予想時間とタンクの第4節:プロダクト定義節から第7節:資料節
///
/// # 戻り値
///
/// * タンクの特定の時間の土壌雨量指数
fn read_tank_indexes<R>(
    reader: &mut BufReader<R>,
    section3: &Section3_0,
    fpsw_sections: &PswSections,
) -> Grib2Result<Vec<Option<u16>>>
where
    R: Read + Seek,
{
    // 第7節のランレングス圧縮オクテット列の開始位置にファイルポインターを移動
    reader
        .seek(SeekFrom::Start(
            fpsw_sections.section7.run_length_position() as u64,
        ))
        .map_err(|e| Grib2Error::Unexpected(e.into()))?;
    // イテレーターを構築
    let iter = Grib2RecordIterBuilder::new()
        .reader(reader)
        .total_bytes(fpsw_sections.section7.run_length_bytes())
        .number_of_points(section3.number_of_data_points())
        .lat_max(section3.lat_of_first_grid_point())
        .lon_min(section3.lon_of_first_grid_point())
        .lon_max(section3.lon_of_last_grid_point())
        .lat_inc(section3.j_direction_increment())
        .lon_inc(section3.i_direction_increment())
        .nbit(fpsw_sections.section5.bits_per_value() as u16)
        .maxv(fpsw_sections.section5.max_level_value())
        .level_values(fpsw_sections.section5.level_values())
        .build()?;
    // 土壌雨量指数予想値を読み込み
    let mut soil_water_indexes = vec![];
    for record in iter.flatten() {
        soil_water_indexes.push(record.value);
    }

    Ok(soil_water_indexes)
}

impl TankValue {
    /// タンクの土壌雨量指数予想値を読み込み。
    fn from_reader<R: Read + Seek>(
        reader: &mut BufReader<R>,
        tank: PswTank,
        section3: &Section3_0,
        fpsw_sections: &[[PswSections; 3]],
    ) -> Grib2Result<Self> {
        let hour1 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour1 as u8 as usize - 1][tank as u8 as usize],
        )?;
        let hour2 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour2 as u8 as usize - 1][tank as u8 as usize],
        )?;
        let hour3 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour3 as u8 as usize - 1][tank as u8 as usize],
        )?;
        if fpsw_sections.len() == 3 {
            return Ok(Self {
                hour1,
                hour2,
                hour3,
                hour4: None,
                hour5: None,
                hour6: None,
            });
        }
        let hour4 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour4 as u8 as usize - 1][tank as u8 as usize],
        )?;
        let hour5 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour5 as u8 as usize - 1][tank as u8 as usize],
        )?;
        let hour6 = read_tank_indexes(
            reader,
            section3,
            &fpsw_sections[ForecastHour::Hour6 as u8 as usize - 1][tank as u8 as usize],
        )?;

        Ok(Self {
            hour1,
            hour2,
            hour3,
            hour4: Some(hour4),
            hour5: Some(hour5),
            hour6: Some(hour6),
        })
    }
}
