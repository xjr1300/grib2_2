use std::fs::{File, OpenOptions};
use std::io::{BufReader, Seek, SeekFrom};
use std::path::Path;

use crate::readers::records::{Grib2RecordIter, Grib2RecordIterBuilder};
use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_50008, Section5_200u16, Section6,
    Section7_200, Section8,
};
use crate::{Grib2Error, Grib2Result};

/// 解析雨量ファイルリーダー
pub struct PrrReader {
    /// ファイルリーダー
    reader: BufReader<File>,
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第３節:格子系定義節
    section3: Section3_0,
    /// 第４節:プロダクト定義節
    section4: Section4_50008,
    /// 第５節:資料表現節
    section5: Section5_200u16,
    /// 第６節:ビットマップ節
    section6: Section6,
    /// 第７節:資料節
    section7: Section7_200,
    /// 第８節:終端節
    section8: Section8,
}

impl PrrReader {
    /// 解析雨量ファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 解析雨量フィルのパス
    ///
    /// # 戻り値
    ///
    /// * 解析雨量リーダー
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
        let section4 = Section4_50008::from_reader(&mut reader)?;
        let section5 = Section5_200u16::from_reader(&mut reader)?;
        let section6 = Section6::from_reader(&mut reader)?;
        let section7 = Section7_200::from_reader(&mut reader)?;
        let section8 = Section8::from_reader(&mut reader)?;

        Ok(Self {
            reader,
            section0,
            section1,
            section2,
            section3,
            section4,
            section5,
            section6,
            section7,
            section8,
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

    /// 第4節:プロダクト定義節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節
    pub fn section4(&self) -> &Section4_50008 {
        &self.section4
    }

    /// 第5節:資料表現節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第5節:資料表現節
    pub fn section5(&self) -> &Section5_200u16 {
        &self.section5
    }

    /// 第6節:ビットマップ節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第6節:ビットマップ節
    pub fn section6(&self) -> &Section6 {
        &self.section6
    }

    /// 第7節:資料節を返す。
    ///
    /// # 戻り値
    ///
    /// 第7節:資料節
    pub fn section7(&self) -> &Section7_200 {
        &self.section7
    }

    /// 第8節:終端節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub fn section8(&self) -> &Section8 {
        &self.section8
    }

    /// レコードを反復処理するイテレーターを返す。
    ///
    /// # 戻り値
    ///
    /// * レコードを反復処理するイテレーター
    pub fn record_iter(&mut self) -> Grib2Result<Grib2RecordIter<'_, File, u16>> {
        // ランレングス符号の開始位置にファイルポインターを移動
        self.reader
            .seek(SeekFrom::Start(self.section7.run_length_position() as u64))
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;

        // イテレーターを構築
        Grib2RecordIterBuilder::new()
            .reader(&mut self.reader)
            .total_bytes(self.section7.run_length_bytes())
            .number_of_points(self.section3.number_of_data_points())
            .lat_max(self.section3.lat_of_first_grid_point())
            .lon_min(self.section3.lon_of_first_grid_point())
            .lon_max(self.section3.lon_of_last_grid_point())
            .lat_inc(self.section3.j_direction_increment())
            .lon_inc(self.section3.i_direction_increment())
            .nbit(self.section5.bits_per_value() as u16)
            .maxv(self.section5.max_level_value())
            .level_values(self.section5.level_values())
            .build()
    }
}
