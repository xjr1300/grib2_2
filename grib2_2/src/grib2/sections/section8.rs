use std::io::{BufReader, Read};

use crate::readers::utils::read_bytes;
use crate::{Grib2Error, Grib2Result};

/// 第8節:終端節
pub struct Section8 {
    /// 終端マーカー
    pub marker: [u8; 4],
}

/// 第8節:終端のマーカー
const SECTION8_MARKER: &str = "7777";

impl Section8 {
    /// 第8節:終端節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 第8節:終端マーカー
        let marker = read_bytes(reader, "第8節:終端マーカー", 4)?;
        if marker != SECTION8_MARKER.as_bytes() {
            return Err(Grib2Error::Unexpected(
                format!(
                    "第8節の終了が不正です。ファイルを正確に読み込めなかった可能性があります。expected: {:x?}, actual: {:x?}",
                    SECTION8_MARKER.as_bytes(), marker
                )
                .into(),
            ));
        }

        Ok(Self {
            marker: marker.try_into().unwrap(),
        })
    }
}
