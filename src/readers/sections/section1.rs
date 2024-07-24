use time::OffsetDateTime;

/// 第1節: 識別節
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
