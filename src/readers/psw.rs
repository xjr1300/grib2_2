use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::readers::records::{Grib2RecordIter, Grib2RecordIterBuilder};
use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_0, Section5_200u16, Section6, Section7_200,
    Section8,
};
use crate::{Grib2Error, Grib2Result};

/// 土壌雨量指数値リーダー
pub struct PswReader {
    /// ファイルのパス
    pub path: PathBuf,
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第３節:格子系定義節
    section3: Section3_0,
    /// インデックス0: 全タンク
    /// インデックス1: 第一タンク
    /// インデックス2: 第二タンク
    /// タンク別に第4節:プロダクト定義節から第7節:資料節を格納した配列
    tank_sections: [PswTankSections; 3],
    /// 第８節:終端節
    section8: Section8,
}

impl PswReader {
    /// 土壌雨量指数ファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 土壌雨量指数ファイルのパス
    ///
    /// # 戻り値
    ///
    /// * 土壌雨量指数リーダー
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
        let tank_sections = [
            PswTankSections::from_reader(&mut reader)?,
            PswTankSections::from_reader(&mut reader)?,
            PswTankSections::from_reader(&mut reader)?,
        ];
        let section8 = Section8::from_reader(&mut reader)?;

        Ok(Self {
            path: path.to_path_buf(),
            section0,
            section1,
            section2,
            section3,
            tank_sections,
            section8,
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

    /// 指定されたタンクの第4節:プロダクト定義節から第7節:資料節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    pub fn tank_sections(&self, tank: PswTank) -> &PswTankSections {
        &self.tank_sections[tank as u8 as usize]
    }

    /// 第8節:終端節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub fn section8(&self) -> &Section8 {
        &self.section8
    }

    /// 指定されたタンクのレコードを反復処理するイテレーターを返す。
    ///
    /// # 引数
    ///
    /// * `tank` - レコードを取得するタンク
    ///
    /// # 戻り値
    ///
    /// * 指定された土砂災害警戒判定時間のレコードを反復処理するイテレーター
    pub fn record_iter(&mut self, tank: PswTank) -> Grib2Result<Grib2RecordIter<'_, File, u16>> {
        let tank_section = &self.tank_sections[tank as u8 as usize];

        // 土壌雨量指数ファイルを開く
        if !self.path.is_file() {
            return Err(Grib2Error::FileDoesNotExist);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;
        let mut reader = BufReader::new(file);

        // ランレングス符号の開始位置にファイルポインターを移動
        reader
            .seek(SeekFrom::Start(
                tank_section.section7.run_length_position() as u64
            ))
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;

        // イテレーターを構築
        Grib2RecordIterBuilder::new()
            .reader(reader)
            .total_bytes(tank_section.section7.run_length_bytes())
            .number_of_points(self.section3.number_of_data_points())
            .lat_max(self.section3.lat_of_first_grid_point())
            .lon_min(self.section3.lon_of_first_grid_point())
            .lon_max(self.section3.lon_of_last_grid_point())
            .lat_inc(self.section3.j_direction_increment())
            .lon_inc(self.section3.i_direction_increment())
            .nbit(tank_section.section5.bits_per_value() as u16)
            .maxv(tank_section.section5.max_level_value())
            .level_values(tank_section.section5.level_values())
            .build()
    }
}

/// 土壌雨量指数の第4節プロダクト定義節から第7節:資料節
pub struct PswTankSections {
    /// 第4節:プロダクト定義節
    pub section4: Section4_0,
    /// 第5節:資料表現節
    pub section5: Section5_200u16,
    /// 第6節:ビットマップ節
    pub section6: Section6,
    /// 第7節:資料節
    pub section7: Section7_200,
}

impl PswTankSections {
    fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        let section4 = Section4_0::from_reader(reader)?;
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

/// 土壌雨量指数タンク
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PswTank {
    /// 全タンク（土壌雨量指数）
    All = 0,
    /// 第1タンク
    First = 1,
    /// 第2タンク
    Second = 2,
}

impl TryFrom<u8> for PswTank {
    type Error = Grib2Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::All),
            1 => Ok(Self::First),
            2 => Ok(Self::Second),
            _ => Err(Grib2Error::ConvertError(
                format!("`{value}`を`PswTank`型に変換できません。").into(),
            )),
        }
    }
}
