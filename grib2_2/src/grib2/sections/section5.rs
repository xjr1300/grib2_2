use std::io::{BufReader, Read, Seek};

use crate::readers::utils::{read_u16, read_u32, read_u8, validate_u8};
use crate::{Grib2Error, Grib2Result};

/// 第5節:資料表現節
pub enum Section5 {
    /// テンプレート5.200
    Template5_200(Section5_200),
}

impl Section5 {
    // GRIB2ファイルから第5節:資料表現節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第5節:資料表現節
    pub(crate) fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第5節:節の長さ")? as usize;
        // 節番号: 1バイト
        let section_number = validate_u8(reader, 5, "第5節:節番号")?;
        // 全資料点の数: 4バイト
        let number_of_points = read_u32(reader, "第5節:全資料点の数")?;
        // 資料表現テンプレート番号: 2バイト
        let data_representation_template_number =
            read_u16(reader, "第5節:資料表現テンプレート番号")?;
        match data_representation_template_number {
            200 => read_section5_200(reader, section_bytes, section_number, number_of_points, data_representation_template_number),
            _ => Err(Grib2Error::NotImplemented(format!("第5節の資料表現テンプレート番号`{data_representation_template_number}`は未実装です。").into())),
        }
    }

    /// 1データのビット数を返す。
    ///
    /// # 戻り値
    ///
    /// * 1データのビット数
    pub fn bit_per_value(&self) -> Grib2Result<u8> {
        match self {
            Self::Template5_200(s) => Ok(s.bits_per_value),
            // _ => Err(Grib2Error::RuntimeError(
            //     format!("{self}は1データのビット数を記録していません。").into(),
            // )),
        }
    }

    /// 今回の圧縮に用いたレベルの最大値を返す。
    ///
    /// # 戻り値
    ///
    /// * 今回の圧縮に用いたレベルの最大値
    pub fn max_level_value(&self) -> Grib2Result<u16> {
        match self {
            Self::Template5_200(s) => Ok(s.max_level_value),
            // _ => Err(Grib2Error::RuntimeError(
            //     format!("{self}は今回の圧縮に用いたレベルの最大値を記録していません。").into(),
            // )),
        }
    }

    /// レベル別物理値を返す。
    ///
    /// # 戻り値
    ///
    /// * レベル別物理値
    pub fn level_values(&self) -> Grib2Result<&[[u8; 2]]> {
        match self {
            Self::Template5_200(s) => Ok(&s.level_values),
            // _ => Err(Grib2Error::RuntimeError(
            //     format!("{self}は今回の圧縮に用いたレベルの最大値を記録していません。").into(),
            // )),
        }
    }
}

impl std::fmt::Display for Section5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Template5_200(_) => write!(f, "第5節テンプレート5.200"),
        }
    }
}
pub struct Section5_200 {
    /// 節の長さ（バイト数）
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// 全資料点の数
    pub number_of_points: u32,
    /// 資料表現テンプレート番号
    pub data_representation_template_number: u16,
    /// 1データのビット数
    pub bits_per_value: u8,
    /// 今回の圧縮に用いたレベルの最大値
    pub max_level_value: u16,
    /// データの取り得るレベルの最大値
    pub number_of_level_values: u16,
    /// データ代表値の尺度因子
    pub decimal_scale_factor: u8,
    /// レベル値と物理値(mm/h)の対応を格納するコレクション
    pub level_values: Vec<[u8; 2]>,
}

fn read_section5_200<R: Read + Seek>(
    reader: &mut BufReader<R>,
    section_bytes: usize,
    section_number: u8,
    number_of_points: u32,
    data_representation_template_number: u16,
) -> Grib2Result<Section5> {
    // 1データのビット数: 1バイト
    let bits_per_value = read_u8(reader, "第5節:1データのビット数")?;
    // 今回の圧縮に用いたレベルの最大値: 2バイト
    let max_level_value = read_u16(reader, "第5節:今回の圧縮に用いたレベルの最大値")?;
    // データの取り得るレベルの最大値: 2バイト
    let number_of_level_values = read_u16(reader, "第5節:レベルの最大値")?;
    // データ代表値の尺度因子: 1バイト
    let decimal_scale_factor = read_u8(reader, "第5節:データ代表値の尺度因子")?;
    // テンプレート5.200のバイト数を計算
    // 4byte: 節の長さ
    // 1byte: 節番号
    // 4byte: 全資料点の数
    // 1byte: 資料表現テンプレート番号
    // よって、テンプレート5.200のバイト数は、section_bytes - 4 - 1 - 4 - 1 = section_bytes - 10
    let template_bytes = section_bytes - 10;
    // レベルmに対応するデータ代表値の数を計算
    // 1byte: 1データのビット数
    // 2byte: 今回の圧縮に用いたレベルの最大値
    // 2byte: レベルの最大値
    // 1byte: データ代表値の尺度因子
    // よって、レベルmに対応するデータ代表値の数は、(template_bytes - 1 - 2 - 2 - 1) / 2 = (template_bytes - 6) / 2
    let number_of_levels = (template_bytes - 6) / 2;
    // レベルmに対応するデータ代表値
    let mut level_values = Vec::with_capacity(number_of_levels);
    let mut level_value = [0u8; 2];
    for _ in 0..number_of_levels {
        reader.read_exact(&mut level_value).map_err(|e| {
            Grib2Error::ReadError(format!("レベル値の読み込みに失敗しました。{e}").into())
        })?;
        level_values.push(level_value);
    }

    Ok(Section5::Template5_200(Section5_200 {
        section_bytes,
        section_number,
        number_of_points,
        data_representation_template_number,
        bits_per_value,
        max_level_value,
        number_of_level_values,
        decimal_scale_factor,
        level_values,
    }))
}
