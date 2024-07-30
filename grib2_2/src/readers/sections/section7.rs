use std::io::{BufReader, Read, Seek};

use crate::readers::sections::TemplateReaderWithBytes;
use crate::readers::utils::{read_u32, validate_u8};
use crate::{Grib2Error, Grib2Result};

/// 第7節:資料節
#[derive(Debug, Clone, Copy)]
pub struct Section7<T>
where
    T: TemplateReaderWithBytes,
{
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// テンプレート7
    template7: T,
}

impl<T> Section7<T>
where
    T: TemplateReaderWithBytes,
{
    /// 第7節:資料節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * 第7節:資料節
    pub(crate) fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第7節:節の長さ")? as usize;
        // 節番号: 1バイト
        validate_u8(reader, 7, "第7節:節番号")?;
        // テンプレート7
        // let template_bytes = section_bytes - (4 + 1);
        let template7 = T::from_reader(reader, section_bytes)?;

        Ok(Self {
            section_bytes,
            template7,
        })
    }

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

impl TemplateReaderWithBytes for Template7_200 {
    /// テンプレート7.200を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * テンプレート7.200
    fn from_reader<R: Read + Seek>(
        reader: &mut BufReader<R>,
        section_bytes: usize,
    ) -> Grib2Result<Self>
    where
        Self: Sized,
    {
        // ランレングス圧縮符号列の開始位置を記憶
        let run_length_position = reader.stream_position().map_err(|_| {
            Grib2Error::ReadError(
                "第7節:ランレングス圧縮符号列の開始位置の記憶に失敗しました。".into(),
            )
        })? as usize;
        // テンプレート7.200のバイト数を計算
        // 4byte: 節の長さ
        // 1byte: 節番号
        let run_length_bytes = section_bytes - 5;
        // ランレングス圧縮符号列をスキップ
        reader.seek_relative(run_length_bytes as i64).map_err(|_| {
            Grib2Error::ReadError(
                "第7節:ランレングス圧縮オクテット列の読み飛ばしに失敗しました。".into(),
            )
        })?;

        Ok(Self {
            run_length_position,
            run_length_bytes,
        })
    }
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
