/// 第7節: 資料節
#[derive(Debug, Clone, Copy)]
pub struct Section7<T> {
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// テンプレート7
    template7: T,
}

impl<T> Section7<T> {
    /// 節の長さ（バイト数）
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }
}

/// テンプレート7.200
#[derive(Debug, Clone, Copy)]
pub struct Template7_200 {
    /// ランレングス圧縮符号列の開始位置
    run_length_position: usize,
    /// ランレングス圧縮符号のバイト数
    run_length_bytes: usize,
}

pub type Section7_200 = Section7<Template7_200>;

impl Section7_200 {
    /// ランレングス圧縮符号列の開始位置を返す。
    pub fn run_length_position(&self) -> usize {
        self.template7.run_length_position
    }

    /// ランレングス圧縮符号のバイト数を返す。
    pub fn run_length_bytes(&self) -> usize {
        self.template7.run_length_bytes
    }
}
