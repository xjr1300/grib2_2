use std::io::Write;

use anyhow::anyhow;

use grib2_2::readers::PrrReader;
use helpers::{buf_writer, should_write_record};

/// 解析雨量ファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20161121010000_SRF_GPV_Ggis1km_Prr60lv_Aper10min_ANAL_grib2.bin";
#[rustfmt::skip]
const DST_PATH: &str = "resources/dst/Z__C_RJTD_20161121010000_SRF_GPV_Ggis1km_Prr60lv_Aper10min_ANAL_grib2.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let mut reader = PrrReader::new(SRC_PATH)?;
    let iter = reader.record_iter()?;
    let mut writer = buf_writer(DST_PATH)?;
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
