/// 第８節: 終端節
#[derive(Debug, Clone)]
pub struct Section8 {
    /// 終端のマーカー
    end_marker: [u8; 4],
}

impl Section8 {
    /// 終端のマーカーを返す。
    pub fn end_marker(&self) -> &[u8; 4] {
        &self.end_marker
    }
}
