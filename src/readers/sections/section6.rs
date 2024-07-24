/// 第6節: ビットマップ節
#[derive(Debug, Clone, Copy)]
pub struct Section6 {
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// ビットマップ指示符
    bitmap_indicator: u8,
}

impl Section6 {
    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }
    /// ビットマップ指示符を返す。
    pub fn bitmap_indicator(&self) -> u8 {
        self.bitmap_indicator
    }
}
