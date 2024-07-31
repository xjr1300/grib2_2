use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;

use grib2_2::readers::Grib2Record;

/// ファイルライターを構築する。
///
/// # 引数
///
/// * `path` - 出力するファイルのパス
///
/// # 戻り値
///
/// * ファイルライター
pub fn buf_writer<P: AsRef<Path>>(path: P) -> anyhow::Result<BufWriter<File>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path.as_ref())?;

    Ok(BufWriter::new(file))
}

/// レコードを出力するか確認する。
///
/// # 引数
///
/// * `record` - レコード
///
/// # 戻り値
///
/// * 出力する場合は`true`
/// * 出力しない場合は`false`
pub fn should_write_record<T>(record: &Grib2Record<T>) -> bool
where
    T: Clone + Copy,
{
    record.value.is_some()
}

/// オプショナルな値を書式化する。
///
/// # 引数
///
/// * `value` - オプショナルな値
///
/// # 戻り値
///
/// * オプショナルな値を初期化した文字列。
pub fn format_optional_value<T: Clone + Copy + ToString>(value: Option<T>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => "".into(),
    }
}

pub mod grib2 {
    use grib2_2::grib2::reader::Grib2Record;

    /// レコードを出力するか確認する。
    ///
    /// # 引数
    ///
    /// * `record` - レコード
    ///
    /// # 戻り値
    ///
    /// * 出力する場合は`true`
    /// * 出力しない場合は`false`
    pub fn should_write_record(record: &Grib2Record) -> bool {
        record.value.is_some()
    }
}
