use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use anyhow::anyhow;

use grib2_2::readers::prr::PrrReader;

/// 実況のみを記録した土砂災害警戒判定メッシュファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20161121000000_SRF_GPV_Ggis1km_Prr60lv_Aper10min_ANAL_grib2.bin";
#[rustfmt::skip]
const DST_PATH: &str = "resources/dst/Z__C_RJTD_20161121000000_SRF_GPV_Ggis1km_Prr60lv_Aper10min_ANAL_grib2.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let mut reader = PrrReader::new(SRC_PATH)?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DST_PATH)?;
    let iter = reader.record_iter().unwrap();
    let mut writer = BufWriter::new(file);
    writer.write_all(b"lon,lat,precipitation\n")?;
    for record in iter.flatten() {
        let lon = record.lon as f64 / 1e6;
        let lat = record.lat as f64 / 1e6;
        if let Some(value) = record.value {
            writer.write_fmt(format_args!("{lon:.6},{lat:.6},{value}\n"))?;
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}
