use std::io::{BufReader, Read};

use crate::Grib2Result;

/// 第2節:地域使用節（不使用）
pub struct Section2;

impl Section2 {
    /// GRIB2ファイルから第2節:地域使用節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第2節:地域使用節
    pub(crate) fn from_reader<R: Read>(_reader: &mut BufReader<R>) -> Grib2Result<Self> {
        Ok(Self)
    }
}
