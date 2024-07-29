use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

use anyhow::anyhow;

use grib2_2::readers::{Grib2RecordIter, PswReader, PswTank};

/// 土壌雨量指数ファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20170807170000_SRF_GPV_Ggis1km_Psw_Aper10min_ANAL_grib2.bin";
#[rustfmt::skip]
const DST_ALL_PATH: &str = "resources/dst/Z__C_RJTD_20170807170000_SRF_GPV_Ggis1km_Psw_Aper10min_ANAL_grib2_all.csv";
#[rustfmt::skip]
const DST_FIRST_PATH: &str = "resources/dst/Z__C_RJTD_20170807170000_SRF_GPV_Ggis1km_Psw_Aper10min_ANAL_grib2_first.csv";
#[rustfmt::skip]
const DST_SECOND_PATH: &str = "resources/dst/Z__C_RJTD_20170807170000_SRF_GPV_Ggis1km_Psw_Aper10min_ANAL_grib2_second.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let mut reader = PswReader::new(SRC_PATH)?;

    let dst_paths = [DST_ALL_PATH, DST_FIRST_PATH, DST_SECOND_PATH];
    for (tank, dst_path) in (0..=3u8).zip(dst_paths) {
        let tank = PswTank::try_from(tank)?;
        let iter = reader.record_iter(tank).unwrap();
        output_tank(iter, dst_path)?;
    }

    Ok(())
}

fn output_tank(iter: Grib2RecordIter<File, u16>, dst_path: &str) -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"lon,lat,index\n")?;
    for record in iter.flatten() {
        let lon = record.lon as f64 / 1e6;
        let lat = record.lat as f64 / 1e6;
        if let Some(value) = record.value {
            writer.write_fmt(format_args!("{lon:.6},{lat:.6},{value}\n"))?;
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}
