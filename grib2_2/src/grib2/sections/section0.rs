use std::io::{BufReader, Read};

use crate::readers::utils::{read_bytes, read_u64, read_u8};
use crate::{Grib2Error, Grib2Result};

/// 第0節:指示節
pub struct Section0 {
    /// GRIB
    pub grib: [u8; 4],
    /// 保留
    pub reserved: [u8; 2],
    /// 資料分野
    pub field: u8,
    /// GRIB版番号
    pub editions: u8,
    /// GRIB報全体のバイト数
    pub total_bytes: usize,
}

impl Section0 {
    /// 第0節:指示節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第0節:指示節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // GRIB: 4バイト
        let grib = read_bytes(reader, "第0節:GRIB", 4)?;
        if grib != b"GRIB" {
            return Err(Grib2Error::ReadError(
                "第0節:GRIBは、ASCIIバイト表現で`GRIB`を記録していなければなりません。".into(),
            ));
        }
        // 保留: 2バイト
        let reserved = read_bytes(reader, "第0節:保留", 2)?;
        // 資料分野: 1バイト
        let field = read_u8(reader, "第0節:資料分野")?;
        // GRIB版番号: 1バイト
        let editions = read_u8(reader, "第0節:GRIB版番号")?;
        // GRIB報全体の長さ: 8バイト
        let total_bytes = read_u64(reader, "第0節:GRIB報全体の長さ")? as usize;

        Ok(Self {
            grib: grib.try_into().unwrap(),
            reserved: reserved.try_into().unwrap(),
            field,
            editions,
            total_bytes,
        })
    }
}
