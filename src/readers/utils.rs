use std::io::{BufReader, Read};

use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

use crate::{Grib2Error, Grib2Result};

/// バイト列を読み込む。
///
/// # 引数
///
/// * `reader` - リーダー
/// * `name` - 読み込むデータの名前
/// * `length` - 読み込むバイト数
pub(crate) fn read_bytes<R>(
    reader: &mut BufReader<R>,
    name: &str,
    length: usize,
) -> Grib2Result<Vec<u8>>
where
    R: Read,
{
    let mut buf = vec![0_u8; length];
    reader.read_exact(&mut buf).map_err(|e| {
        Grib2Error::ReadError(format!("{name}の読み込みに失敗しました。{e}").into())
    })?;

    Ok(buf)
}

/// 符号なし整数を読み込む関数を生成するマクロ
///
/// * `$fname` - 関数名
/// * `$type` - 読み込む符号なし整数の型
macro_rules! impl_read_uint {
    ($fname:ident, $type:ty) => {
        /// 符号なし整数を読み込む。
        ///
        /// # 引数
        ///
        /// * `reader` - リーダー
        /// * `name` - 読み込むデータの名前
        ///
        /// # 戻り値
        ///
        /// * 符号なし整数の値
        pub(crate) fn $fname<R>(reader: &mut BufReader<R>, name: &str) -> Grib2Result<$type>
        where
            R: Read,
        {
            let expected_bytes = std::mem::size_of::<$type>();
            let mut buf = vec![0_u8; expected_bytes];
            reader.read_exact(&mut buf).map_err(|e| {
                Grib2Error::ReadError(format!("{name}の読み込みに失敗しました。{e}").into())
            })?;

            Ok(<$type>::from_be_bytes(buf.try_into().unwrap()))
        }
    };
}

impl_read_uint!(read_u8, u8);
impl_read_uint!(read_u16, u16);
impl_read_uint!(read_u32, u32);
impl_read_uint!(read_u64, u64);

/// 符号なし整数を読み込み検証する関数を生成するマクロ
macro_rules! validate_uint {
    ($fname:ident, $read_fn:ident, $type:ty) => {
        /// 符号なし整数を読み込んで検証する。
        ///
        /// # 引数
        ///
        /// * `reader` - リーダー
        /// * `expected` - 期待する値
        /// * `name` - 読み込むデータの名前
        ///
        /// # 戻り値
        ///
        /// * 符号なし整数の値
        pub(crate) fn $fname<R>(
            reader: &mut BufReader<R>,
            expected: $type,
            name: &str,
        ) -> Grib2Result<$type>
        where
            R: Read,
        {
            let value = $read_fn(reader, name).map_err(|_| {
                Grib2Error::ReadError(format!("{}の読み込みに失敗しました。", name).into())
            })?;
            if value != expected {
                return Err(Grib2Error::Unexpected(
                    format!(
                        "{}の値は{}でしたが、{}でなければなりません。",
                        name, value, expected
                    )
                    .into(),
                ));
            }

            Ok(value)
        }
    };
}

validate_uint!(validate_u8, read_u8, u8);
//validate_uint!(validate_u16, read_u16, u16);
validate_uint!(validate_u32, read_u32, u32);
//validate_uint!(validate_u64, read_u64, u64);

/// 符号付き整数を読み込む関数を生成するマクロ
///
/// * `$fname` - 関数名
/// * `$type` - 読み込む符号付き整数の型
macro_rules! impl_read_int {
    ($fname:ident, $type:ty) => {
        /// 符号付き整数を読み込む。
        ///
        /// # 引数
        ///
        /// * `reader` - リーダー
        /// * `name` - 読み込むデータの名前
        ///
        /// # 戻り値
        ///
        /// * 符号付き整数の値
        pub(crate) fn $fname<R: Read>(reader: &mut BufReader<R>, name: &str) -> Grib2Result<$type> {
            let expected_bytes = std::mem::size_of::<$type>();
            let mut buf = vec![0_u8; expected_bytes];
            reader.read_exact(&mut buf).map_err(|_| {
                Grib2Error::ReadError(format!("{}の読み込みに失敗しました。", name).into())
            })?;
            let sign = if buf[0] & 0x80 == 0 { 1 } else { -1 };
            buf[0] &= 0x7F;

            Ok(<$type>::from_be_bytes(buf.try_into().unwrap()) * sign)
        }
    };
}

//impl_read_int!(read_i8, i8);
impl_read_int!(read_i16, i16);
impl_read_int!(read_i32, i32);
//impl_read_int!(read_i64, i64);

pub(crate) fn read_date_time<R>(
    reader: &mut BufReader<R>,
    name: &str,
) -> Grib2Result<OffsetDateTime>
where
    R: Read,
{
    let year = read_u16(reader, name)?;
    let mut parts = Vec::new();
    for _ in 0..5 {
        parts.push(read_u8(reader, name)?);
    }
    // 日付と時刻を構築
    let month = Month::try_from(parts[0]).map_err(|_| {
        Grib2Error::Unexpected(
            format!(
                "{}:月の値は{}でしたが、1から12の範囲でなければなりません。",
                name, parts[0]
            )
            .into(),
        )
    })?;
    let date = Date::from_calendar_date(year as i32, month, parts[1]).map_err(|_| {
        Grib2Error::Unexpected(
            format!(
                "{}:{}年{}月{}日を日付に変換できませんでした。",
                name, year, month as u8, parts[1]
            )
            .into(),
        )
    })?;
    let time = Time::from_hms(parts[2], parts[3], parts[4]).map_err(|_| {
        Grib2Error::Unexpected(
            format!(
                "{}:{}時{}分{}秒を時刻に変換できませんでした。",
                name, parts[2], parts[3], parts[4]
            )
            .into(),
        )
    })?;

    Ok(PrimitiveDateTime::new(date, time).assume_utc())
}
