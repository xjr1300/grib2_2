use std::io::{BufReader, Read};

use time::OffsetDateTime;

use crate::readers::utils::{read_date_time, read_u16, read_u8, validate_u32, validate_u8};
use crate::Grib2Result;

/// 第1節:節の長さ（バイト）
const SECTION1_BYTES: u32 = 21;

/// 第1節:識別節
#[derive(Debug, Clone, Copy)]
pub struct Section1 {
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// 作成中枢の識別
    center: u16,
    /// 作成副中枢
    sub_center: u16,
    /// GRIBマスター表バージョン番号
    table_version: u8,
    /// GRIB地域表バージョン番号
    local_table_version: u8,
    /// 参照時刻の意味
    significance_of_reference_time: u8,
    /// 資料の参照時刻（世界標準時）
    referenced_at: OffsetDateTime,
    /// 作成ステータス
    production_status_of_processed_data: u8,
    /// 資料の種類
    type_of_processed_data: u8,
}

impl Section1 {
    /// 第1節:識別節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第1節:識別節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4bytes
        let section_bytes = validate_u32(reader, SECTION1_BYTES, "第1節:節の長さ")? as usize;
        // 節番号
        validate_u8(reader, 1, "第1節:節番号")?;
        // 作成中枢の識別: 2bytes
        let center = read_u16(reader, "第1節:作成中枢")?;
        // 作成副中枢: 2bytes
        let sub_center = read_u16(reader, "第1節:作成副中枢")?;
        // GRIBマスター表バージョン番号: 1byte
        let table_version = read_u8(reader, "第1節:GRIBマスター表バージョン番号")?;
        // GRIB地域表バージョン番号: 1byte
        let local_table_version = read_u8(reader, "第1節:GRIB地域表バージョン番号")?;
        // 参照時刻の意味: 1byte
        let significance_of_reference_time = read_u8(reader, "第1節:参照時刻の意味")?;
        // 資料の参照時刻（日時）
        let referenced_at = read_date_time(reader, "第1節:資料の参照時刻")?;
        // 作成ステータス
        let production_status_of_processed_data = read_u8(reader, "第1節:作成ステータス")?;
        // 資料の種類
        let type_of_processed_data = read_u8(reader, "第1節:資料の種類")?;

        Ok(Self {
            section_bytes,
            center,
            sub_center,
            table_version,
            local_table_version,
            significance_of_reference_time,
            referenced_at,
            production_status_of_processed_data,
            type_of_processed_data,
        })
    }

    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }

    /// 作成中枢の識別を返す。
    pub fn center(&self) -> u16 {
        self.center
    }

    /// 作成副中枢を返す。
    pub fn sub_center(&self) -> u16 {
        self.sub_center
    }

    /// GRIBマスター表バージョン番号を返す。
    pub fn table_version(&self) -> u8 {
        self.table_version
    }

    /// GRIB地域表バージョン番号を返す。
    pub fn local_table_version(&self) -> u8 {
        self.local_table_version
    }

    /// 参照時刻の意味を返す。
    pub fn significance_of_reference_time(&self) -> u8 {
        self.significance_of_reference_time
    }

    /// 資料の参照時刻（世界標準時）を返す。
    pub fn referenced_at(&self) -> OffsetDateTime {
        self.referenced_at
    }

    /// 作成ステータスを返す。
    pub fn production_status_of_processed_data(&self) -> u8 {
        self.production_status_of_processed_data
    }

    /// 資料の種類を返す。
    pub fn type_of_processed_data(&self) -> u8 {
        self.type_of_processed_data
    }
}
