use std::borrow::Cow;

pub mod grib2;
pub mod readers;

/// GRIB2結果
type Grib2Result<T> = Result<T, Grib2Error>;

/// GRIB2エラー
#[derive(Debug, thiserror::Error)]
pub enum Grib2Error {
    #[error("ファイルが存在しません。")]
    FileDoesNotExist,

    /// 読み込みエラー
    #[error("{0}")]
    ReadError(Cow<'static, str>),

    /// ランタイムエラー
    #[error("{0}")]
    RuntimeError(Cow<'static, str>),

    /// 変換エラー
    #[error("{0}")]
    ConvertError(Cow<'static, str>),

    /// 未実装エラー
    #[error("{0}")]
    NotImplemented(Cow<'static, str>),

    /// 予期しないエラー
    #[error("予期していないエラーが発生しました。{0}")]
    Unexpected(Box<dyn std::error::Error + Send + Sync + 'static>),
}
