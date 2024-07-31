use std::io::{BufReader, Read};

use crate::readers::utils::{read_u32, read_u8, validate_u8};
use crate::Grib2Result;

/// 第6節:ビットマップ節
pub struct Section6 {
    /// 節の長さ（バイト数）
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// ビットマップ指示符
    pub bitmap_indicator: u8,
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
        let section_bytes = read_u32(reader, "第6節:節の長さ")? as usize;
        // 節番号: 1バイト
        let section_number = validate_u8(reader, 6, "第6節:節番号")?;
        // ビットマップ指示符: 1バイト
        let bitmap_indicator = read_u8(reader, "第6節:ビットマップ指示符")?;

        Ok(Self {
            section_bytes,
            section_number,
            bitmap_indicator,
        })
    }
}
