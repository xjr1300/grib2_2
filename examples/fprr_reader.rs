use std::borrow::Cow;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use anyhow::anyhow;

use grib2_2::readers::{FPrrReader, FPrrValue};

/// 降水短時間予報ファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20170807001000_SRF_GPV_Ggis1km_Prr60lv_Fper10min_FH01-06_grib2.bin";
#[rustfmt::skip]
const DST_PATH: &str = "resources/dst/Z__C_RJTD_20170807001000_SRF_GPV_Ggis1km_Prr60lv_Fper10min_FH01-06_grib2.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let reader = FPrrReader::new(SRC_PATH)?;
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(DST_PATH)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(b"lon,lat,hour1,hour2,hour3,hour4,hour5,hour6\n")?;
    for record in reader.value_iter() {
        if should_write(&record) {
            let lon = record.lon as f64 / 1e6;
            let lat = record.lat as f64 / 1e6;
            writer.write_fmt(format_args!(
                "{lon:.6},{lat:.6},{},{},{},{},{},{}\n",
                format_u16(record.hour1),
                format_u16(record.hour2),
                format_u16(record.hour3),
                format_u16(record.hour4),
                format_u16(record.hour5),
                format_u16(record.hour6)
            ))?;
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}

fn should_write(record: &FPrrValue) -> bool {
    record.hour1.is_some()
        || record.hour2.is_some()
        || record.hour3.is_some()
        || record.hour4.is_some()
        || record.hour5.is_some()
        || record.hour6.is_some()
}

fn format_u16(value: Option<u16>) -> Cow<'static, str> {
    match value {
        Some(value) => value.to_string().into(),
        None => "".into(),
    }
}
