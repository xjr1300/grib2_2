use std::io::{Read, Seek, Write};

use anyhow::anyhow;

use grib2_2::readers::{Grib2RecordIter, LwjmHour, LwjmReader};
use helpers::{buf_writer, should_write_record};

/// 実況と1時間から3時間までの予想を記録した土砂災害警戒判定メッシュファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20180706095000_MET_INF_Jdosha_Ggis1km_FH00-03_grib2.bin";
#[rustfmt::skip]
const DST_LIVE_PATH: &str = "resources/dst/Z__C_RJTD_20180706095000_MET_INF_Jdosha_Ggis1km_FH00-03_grib2_live.csv";
#[rustfmt::skip]
const DST_HOUR1_PATH: &str = "resources/dst/Z__C_RJTD_20180706095000_MET_INF_Jdosha_Ggis1km_FH00-03_grib2_hour1.csv";
#[rustfmt::skip]
const DST_HOUR2_PATH: &str = "resources/dst/Z__C_RJTD_20180706095000_MET_INF_Jdosha_Ggis1km_FH00-03_grib2_hour2.csv";
#[rustfmt::skip]
const DST_HOUR3_PATH: &str = "resources/dst/Z__C_RJTD_20180706095000_MET_INF_Jdosha_Ggis1km_FH00-03_grib2_hour3.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let mut reader = LwjmReader::new(SRC_PATH, true)?;
    let dst_paths = [
        DST_LIVE_PATH,
        DST_HOUR1_PATH,
        DST_HOUR2_PATH,
        DST_HOUR3_PATH,
    ];
    for (hour, dst_path) in (0..=3u8).zip(dst_paths) {
        let hour = LwjmHour::try_from(hour)?;
        let iter = reader.record_iter(hour).unwrap();
        output_judgement(iter, dst_path)?;
    }

    Ok(())
}

fn output_judgement<R: Read + Seek>(
    iter: Grib2RecordIter<R, i16>,
    dst_path: &str,
) -> anyhow::Result<()> {
    let mut writer = buf_writer(dst_path)?;
    writer.write_all(b"lon,lat,value\n")?;
    for record in iter.flatten() {
        if should_write_record(&record) {
            let lon = record.lon as f64 / 1e6;
            let lat = record.lat as f64 / 1e6;
            if let Some(value) = record.value {
                writer.write_fmt(format_args!("{lon:.6},{lat:.6},{value}\n"))?;
            }
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}
