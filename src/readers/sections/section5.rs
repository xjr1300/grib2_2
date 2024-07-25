use std::io::{BufReader, Read, Seek};

use crate::readers::sections::TemplateReaderWithBytes;
use crate::readers::utils::{read_i16, read_u16, read_u32, read_u8, validate_u8};
use crate::Grib2Result;

/// 第5節:資料表現節
#[derive(Debug, Clone)]
pub struct Section5<T>
where
    T: TemplateReaderWithBytes,
{
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// 全資料点の数
    number_of_values: u32,
    /// 資料表現テンプレート番号
    data_representation_template_number: u16,
    /// テンプレート5
    template5: T,
}

impl<T> Section5<T>
where
    T: TemplateReaderWithBytes,
{
    /// 第5節:資料表現節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * 第5節:資料表現節
    pub(crate) fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第5節:節の長さ")? as usize;
        // 節番号: 1バイト
        validate_u8(reader, 5, "第5節:節番号")?;
        // 全資料点の数: 4バイト
        let number_of_values = read_u32(reader, "第5節:全資料点の数")?;
        // 資料表現テンプレート番号: 2バイト
        let data_representation_template_number =
            read_u16(reader, "第5節:資料表現テンプレート番号")?;
        // テンプレート5
        let template5 = T::from_reader(reader, section_bytes)?;

        Ok(Self {
            section_bytes,
            number_of_values,
            data_representation_template_number,
            template5,
        })
    }

    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }

    /// 全資料点の数を返す。
    pub fn number_of_values(&self) -> u32 {
        self.number_of_values
    }

    /// 資料表現テンプレート番号を返す。
    pub fn data_representation_template_number(&self) -> u16 {
        self.data_representation_template_number
    }
}

/// テンプレート5.200
#[derive(Debug, Clone)]
pub struct Template5_200<V>
where
    V: Clone + Copy,
{
    /// 1データのビット数
    bits_per_value: u8,
    /// 今回の圧縮に用いたレベルの最大値
    max_level_value: u16,
    /// データの取り得るレベルの最大値
    number_of_level_values: u16,
    /// データ代表値の尺度因子
    decimal_scale_factor: u8,
    /// レベル値と物理値(mm/h)の対応を格納するコレクション
    level_values: Vec<V>,
}

macro_rules! template5_200 {
    ($template_name:ident, $type:ty, $read_func:ident) => {
        pub type $template_name = Template5_200<$type>;

        impl TemplateReaderWithBytes for $template_name {
            /// `i16`型の値を記録したテンプレート5.200を読み込む。
            ///
            /// # 引数
            ///
            /// * `reader` - GRIB2リーダー
            /// * `section_bytes` - 第5節全体のバイト数
            ///
            /// # 戻り値
            ///
            /// * テンプレート5.200
            fn from_reader<R: Read>(
                reader: &mut BufReader<R>,
                section_bytes: usize,
            ) -> Grib2Result<Self>
            where
                Self: Sized,
            {
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
                for _ in 0..number_of_levels {
                    level_values.push($read_func(reader, "第5節:レベルmに対応するデータ代表値")?);
                }

                Ok(Self {
                    bits_per_value,
                    max_level_value,
                    number_of_level_values,
                    decimal_scale_factor,
                    level_values,
                })
            }
        }
    };
}

macro_rules! section5_200 {
    ($struct_name:ident, $template_name:ident, $type:ty) => {
        pub type $struct_name = Section5<$template_name>;

        impl $struct_name {
            /// 1データのビット数を返す。
            pub fn bits_per_value(&self) -> u8 {
                self.template5.bits_per_value
            }

            /// 今回の圧縮に用いたレベルの最大値を返す。
            pub fn max_level_value(&self) -> u16 {
                self.template5.max_level_value
            }

            /// データの取り得るレベルの最大値を返す。
            pub fn number_of_level_values(&self) -> u16 {
                self.template5.number_of_level_values
            }

            /// データ代表値の尺度因子を返す。
            pub fn decimal_scale_factor(&self) -> u8 {
                self.template5.decimal_scale_factor
            }

            /// レベルmに対応するデータ代表値を返す。
            pub fn level_values(&self) -> &[$type] {
                &self.template5.level_values
            }
        }
    };
}

template5_200!(Template5_200i16, i16, read_i16);
section5_200!(Section5_200i16, Template5_200i16, i16);

template5_200!(Template5_200u16, u16, read_u16);
section5_200!(Section5_200u16, Template5_200u16, u16);
