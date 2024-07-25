use std::io::{BufReader, Read, Seek};

mod section0;
mod section1;
mod section2;
mod section3;
mod section4;
mod section5;
mod section6;
mod section7;
mod section8;

use crate::Grib2Result;
pub use section0::Section0;
pub use section1::Section1;
pub use section2::Section2;
pub use section3::{Section3, Section3_0};
pub use section4::{Section4, Section4_50000, Section4_50008};
pub use section5::{Section5, Section5_200i16, Section5_200u16};
pub use section6::Section6;
pub use section7::{Section7, Section7_200};
pub use section8::Section8;

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
