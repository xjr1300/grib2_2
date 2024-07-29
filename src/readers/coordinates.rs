use std::cmp::Ordering;

use super::Grib2Record;

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
