use std::io::{BufReader, Read, Seek};

use crate::readers::utils::{read_u32, validate_u8};
use crate::{Grib2Error, Grib2Result};

/// 第7節:資料節
pub enum Section7 {
    /// テンプレート7.200
    Template7_200(Section7_200),
}

impl Section7 {
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
        let section_number = validate_u8(reader, 7, "第7節:節番号")?;
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

        Ok(Self::Template7_200(Section7_200 {
            section_bytes,
            section_number,
            run_length_position,
            run_length_bytes,
        }))
    }

    /// ランレングス圧縮符号列の開始位置を返す。
    ///
    /// # 戻り値
    ///
    /// * ランレングス圧縮符号列の開始位置
    pub fn run_length_position(&self) -> Grib2Result<usize> {
        match self {
            Self::Template7_200(s) => Ok(s.run_length_position),
            // _ => Err(Grib2Error::RuntimeError(
            //     format!("{}はランレングス圧縮符号列の開始位置を返せません。", self).into(),
            // )),
        }
    }

    /// ランレングス圧縮符号列の全体バイト数を返す。
    ///
    /// # 戻り値
    ///
    /// * ランレングス圧縮符号列の全体バイト数
    pub fn run_length_bytes(&self) -> Grib2Result<usize> {
        match self {
            Self::Template7_200(s) => Ok(s.run_length_bytes),
            // _ => Err(Grib2Error::RuntimeError(
            //     format!(
            //         "{}はランレングス圧縮符号列の全体バイト数を返せません。",
            //         self
            //     )
            //     .into(),
            // )),
        }
    }
}

impl std::fmt::Display for Section7 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Template7_200(_) => write!(f, "第7節テンプレート7.200"),
        }
    }
}

pub struct Section7_200 {
    /// 節の長さ（バイト数）
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// ランレングス圧縮符号列の開始位置
    pub run_length_position: usize,
    /// ランレングス圧縮符号のバイト数
    pub run_length_bytes: usize,
}
