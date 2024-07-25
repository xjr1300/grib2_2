use std::io::{BufReader, Read, Seek};

use crate::Grib2Result;

pub mod section0;
pub mod section1;
pub mod section2;
pub mod section3;
pub mod section4;
pub mod section5;
pub mod section6;
pub mod section7;
pub mod section8;

/// GRIB2のテンプレートに実装するトレイト
pub trait TemplateReader {
    /// GRIB2のテンプレートを読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * GRIB2テンプレート
    fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self>
    where
        Self: Sized;
}

/// GRIB2を読み込むときに、節全体のバイト数が必要なテンプレートに実装するトレイト
pub trait TemplateReaderWithBytes {
    /// GRIB2のテンプレートを読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    /// * `section_bytes` - 読み込むテンプレートが属する節全体のバイト数
    ///
    /// # 戻り値
    ///
    /// * GRIB2テンプレート
    fn from_reader<R: Read + Seek>(
        reader: &mut BufReader<R>,
        section_bytes: usize,
    ) -> Grib2Result<Self>
    where
        Self: Sized;
}
