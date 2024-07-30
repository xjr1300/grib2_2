use std::io::Write;

use anyhow::anyhow;

use grib2_2::readers::{FPswIndex, FPswReader, ForecastRange, PswTank};
use helpers::{buf_writer, format_optional_value};

/// 土壌水分量予想値ファイル
/// cspell: disable
#[rustfmt::skip]
const SRC_PATH: &str = "resources/Z__C_RJTD_20170807152000_SRF_GPV_Ggis1km_Psw_Fper10min_FH01-06_grib2.bin";
#[rustfmt::skip]
const DST_ALL_PATH: &str = "resources/dst/Z__C_RJTD_20170807001000_SRF_GPV_Ggis1km_Prr60lv_Fper10min_FH01-06_grib2_all.csv";
#[rustfmt::skip]
const DST_TANK1_PATH: &str = "resources/dst/Z__C_RJTD_20170807001000_SRF_GPV_Ggis1km_Prr60lv_Fper10min_FH01-06_grib2_tank1.csv";
#[rustfmt::skip]
const DST_TANK2_PATH: &str = "resources/dst/Z__C_RJTD_20170807001000_SRF_GPV_Ggis1km_Prr60lv_Fper10min_FH01-06_grib2_tank2.csv";
/// cspell: enable

fn main() -> anyhow::Result<()> {
    let mut reader = FPswReader::new(SRC_PATH, ForecastRange::Hours6)?;
    write_file(&mut reader, PswTank::All, DST_ALL_PATH)?;
    write_file(&mut reader, PswTank::Tank1, DST_TANK1_PATH)?;
    write_file(&mut reader, PswTank::Tank2, DST_TANK2_PATH)?;

    Ok(())
}

fn write_file(reader: &mut FPswReader, tank: PswTank, dst_path: &str) -> anyhow::Result<()> {
    let mut writer = buf_writer(dst_path)?;
    writer.write_all(b"lon,lat,hour1,hour2,hour3,hour4,hour5,hour6\n")?;
    for record in reader.value_iter(tank) {
        if should_write(&record) {
            let lon = record.lon as f64 / 1e6;
            let lat = record.lat as f64 / 1e6;
            writer.write_fmt(format_args!(
                "{lon:.6},{lat:.6},{},{},{},{},{},{}\n",
                format_optional_value(record.hour1),
                format_optional_value(record.hour2),
                format_optional_value(record.hour3),
                format_optional_value(record.hour4),
                format_optional_value(record.hour5),
                format_optional_value(record.hour6)
            ))?;
        }
    }
    writer.flush().map_err(|e| anyhow!(e))
}

fn should_write(record: &FPswIndex) -> bool {
    record.hour1.is_some()
        || record.hour2.is_some()
        || record.hour3.is_some()
        || record.hour4.is_some()
        || record.hour5.is_some()
        || record.hour6.is_some()
}
