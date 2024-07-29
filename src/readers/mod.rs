mod fprr;
mod fpsw;
mod lwjm;
mod prr;
mod psw;
mod records;
pub mod sections;
mod utils;

use std::cmp::Ordering;

use crate::Grib2Error;
pub use fprr::FPrrReader;
pub use lwjm::{LwjmHour, LwjmReader, LwjmSections};
pub use prr::PrrReader;
pub use psw::{PswReader, PswSections, PswTank};
pub use records::{Grib2Record, Grib2RecordIter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ForecastHour {
    /// 1時間予想
    Hour1 = 1,
    /// 2時間予想
    Hour2 = 2,
    /// 3時間予想
    Hour3 = 3,
    /// 4時間予想
    Hour4 = 4,
    /// 5時間予想
    Hour5 = 5,
    /// 6時間予想
    Hour6 = 6,
}

impl TryFrom<u8> for ForecastHour {
    type Error = Grib2Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Hour1),
            2 => Ok(Self::Hour2),
            3 => Ok(Self::Hour3),
            4 => Ok(Self::Hour4),
            5 => Ok(Self::Hour5),
            6 => Ok(Self::Hour6),
            _ => Err(Grib2Error::ConvertError(
                format!("`{value}`を`FPrrHour`型に変換できません。").into(),
            )),
        }
    }
}

/// 座標
///
/// 緯度と経度は1e-6度単位で管理するため、実際の緯度と経度にするためには1e-6を乗じる。￥
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Coordinate {
    /// 1e-6度単位の緯度
    pub(crate) lat: u32,
    /// 1e-6度単位の経度
    pub(crate) lon: u32,
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.lat.cmp(&other.lat);
        if result != Ordering::Equal {
            return result;
        }

        self.lon.cmp(&other.lon)
    }
}

impl<T> From<Grib2Record<T>> for Coordinate
where
    T: Clone + Copy,
{
    fn from(value: Grib2Record<T>) -> Self {
        Self {
            lat: value.lat,
            lon: value.lon,
        }
    }
}

/// 予想時間範囲
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForecastRange {
    /// 1時間から6時間
    Hours6 = 6,
    /// 1時間から3時間
    Hours3 = 3,
}

impl ForecastRange {
    /// 予想時間が含まれるか確認する。
    ///
    /// # 引数
    ///
    /// * `hour` - 予想時間
    ///
    /// # 戻り値
    ///
    /// * 予想時間が含まれる場合は`true`
    /// * 予想時間が含まれない場合は`false`
    pub(crate) fn contains(&self, hour: ForecastHour) -> bool {
        match self {
            Self::Hours6 => true,
            Self::Hours3 => match hour {
                ForecastHour::Hour1 | ForecastHour::Hour2 | ForecastHour::Hour3 => true,
                _ => false,
            },
        }
    }
}
