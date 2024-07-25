use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

use anyhow::anyhow;

use grib2_2::readers::lwjm::{LwjmHour, LwjmReader};
use grib2_2::readers::records::Grib2RecordIter;

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

fn output_judgement(iter: Grib2RecordIter<File, i16>, dst_path: &str) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"lon,lat,judgment\n")?;
    for record in iter.flatten() {
        let lon = record.lon as f64 / 1e6;
        let lat = record.lat as f64 / 1e6;
        if let Some(value) = record.value {
            writer.write_fmt(format_args!("{lon:.6},{lat:.6},{value}\n"))?;
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}
