use std::io::{BufReader, Read};

use crate::readers::utils::{read_u8, validate_u32, validate_u8};
use crate::Grib2Result;

/// 第6節:節の長さ（バイト）
const SECTION6_BYTES: u32 = 6;

/// 第6節:ビットマップ節
#[derive(Debug, Clone, Copy)]
pub struct Section6 {
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// ビットマップ指示符
    bitmap_indicator: u8,
}

impl Section6 {
    /// 第6節:ビットマップ節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * 第6節:ビットマップ節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = validate_u32(reader, SECTION6_BYTES, "第6節:節の長さ")? as usize;
        // 節番号: 1バイト
        validate_u8(reader, 6, "第6節:節番号")?;
        // ビットマップ指示符: 1バイト
        let bitmap_indicator = read_u8(reader, "第6節:ビットマップ指示符")?;

        Ok(Self {
            section_bytes,
            bitmap_indicator,
        })
    }

    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }
    /// ビットマップ指示符を返す。
    pub fn bitmap_indicator(&self) -> u8 {
        self.bitmap_indicator
    }
}
