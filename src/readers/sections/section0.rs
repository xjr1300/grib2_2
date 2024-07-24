/// 第0節: 指示節
#[derive(Debug, Clone, Copy)]
pub struct Section0 {
    /// GRIB
    grib: [u8; 4],
    /// 保留
    reserved: [u8; 2],
    /// 資料分野
    field: u8,
    /// GRIB版番号
    editions: u8,
    /// GRIB報全体のバイト数
    total_bytes: usize,
}

impl Section0 {
    /// GRIBを返す。
    pub fn grib(&self) -> &[u8; 4] {
        &self.grib
    }

    /// 保留を返す。
    pub fn reserved(&self) -> &[u8; 2] {
        &self.reserved
    }

    /// 資料分野を返す。
    pub fn field(&self) -> u8 {
        self.field
    }

    /// GRIB版番号を返す。
    pub fn editions(&self) -> u8 {
        self.editions
    }

    /// GRIB報全体のバイト数を返す。
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}
