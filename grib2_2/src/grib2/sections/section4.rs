use std::io::{BufReader, Read};

use time::OffsetDateTime;

use crate::readers::utils::{
    read_date_time, read_i32, read_u16, read_u32, read_u64, read_u8, validate_u8,
};
use crate::{Grib2Error, Grib2Result};

/// 第4節:プロダクト定義節
pub enum Section4 {
    /// テンプレート4.0
    Template4_0(Section4_0),
    /// テンプレート4.50008
    Template4_50008(Section4_50008),
}

impl Section4 {
    /// GRIB2ファイルから第4節:プロダクト定義節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第4節:節の長さ")? as usize;
        // 節番号: 1バイト
        let section_number = validate_u8(reader, 4, "第4節:節番号")?;
        // テンプレート直後の座標値の数: 2バイト
        let number_of_after_template_points =
            read_u16(reader, "第4節:テンプレート直後の座標値の数")?;
        // プロダクト定義テンプレート番号: 2バイト
        let product_definition_template_number =
            read_u16(reader, "第4節:プロダクト定義テンプレート番号")?;
        match product_definition_template_number {
            0 => read_section4_0(reader, section_bytes, section_number, number_of_after_template_points, product_definition_template_number),
            50008 => read_section4_50008(reader, section_bytes, section_number, number_of_after_template_points, product_definition_template_number),
            _ => Err(Grib2Error::NotImplemented(format!("第4節のプロダクト定義テンプレート番号`{product_definition_template_number}`は未実装です。").into())),
        }
    }
}

pub struct Section4_0 {
    /// 節の長さ（バイト数）
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// テンプレート直後の座標値の数
    pub number_of_after_template_points: u16,
    /// プロダクト定義テンプレート番号
    pub product_definition_template_number: u16,
    /// パラメータカテゴリー
    pub parameter_category: u8,
    /// パラメータ番号
    pub parameter_number: u8,
    /// 作成処理の種類
    pub type_of_generating_process: u8,
    /// 背景作成処理識別符
    pub background_process: u8,
    /// 予報の作成処理識別符
    pub generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    pub hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    pub minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    pub indicator_of_unit_of_time_range: u8,
    /// 予報時間
    pub forecast_time: i32,
    /// 第一固定面の種類
    pub type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    pub scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    pub scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    pub type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    pub scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    pub scaled_value_of_second_fixed_surface: u32,
}

/// GRIB2ファイルから第4節:プロダクト定義節（テンプレート4.0）を読み込む。
///
/// # 引数
///
/// * `reader` - ファイルリーダー
/// * `section_bytes` - 節の長さ
/// * `section_number` - 節番号
/// * `number_of_after_template_points` - テンプレート直後の座標値の数
/// * `product_definition_template_number` - プロダクト定義テンプレート番号
///
/// # 戻り値
///
/// * 第4節:プロダクト定義節
fn read_section4_0<R: Read>(
    reader: &mut BufReader<R>,
    section_bytes: usize,
    section_number: u8,
    number_of_after_template_points: u16,
    product_definition_template_number: u16,
) -> Grib2Result<Section4> {
    // パラメータカテゴリー: 1バイト
    let parameter_category = read_u8(reader, "第4節:パラメータカテゴリー")?;
    // パラメータ番号: 1バイト
    let parameter_number = read_u8(reader, "第4節:パラメータ番号")?;
    // 作成処理の種類: 1バイト
    let type_of_generating_process = read_u8(reader, "第4節:作成処理の種類")?;
    // 背景作成処理識別符: 1バイト
    let background_process = read_u8(reader, "第4節:背景作成処理識別符")?;
    // 予報の作成処理識別符: 1バイト
    let generating_process_identifier = read_u8(reader, "第4節:予報の作成処理識別符")?;
    // 観測資料の参照時刻からの締切時間（時）: 2バイト
    let hours_after_data_cutoff = read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
    // 観測資料の参照時刻からの締切時間（分）: 1バイト
    let minutes_after_data_cutoff =
        read_u8(reader, "第4節:観測資料の参照時刻からの締切時間（分）")?;
    // 期間の単位の指示符: 1バイト
    let indicator_of_unit_of_time_range = read_u8(reader, "第4節:期間の単位の指示符")?;
    // 予報時間: 4バイト
    let forecast_time = read_i32(reader, "第4節:予報時間")?;
    // 第一固定面の種類: 1バイト
    let type_of_first_fixed_surface = read_u8(reader, "第4節:第一固定面の種類")?;
    // 第一固定面の尺度因子: 1バイト
    let scale_factor_of_first_fixed_surface = read_u8(reader, "第4節:第一固定面の尺度因子")?;
    // 第一固定面の尺度付きの値: 4バイト
    let scaled_value_of_first_fixed_surface = read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
    // 第二固定面の種類: 1バイト
    let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
    // 第二固定面の尺度因子: 1バイト
    let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
    // 第二固定面の尺度付きの値: 4バイト
    let scaled_value_of_second_fixed_surface = read_u32(reader, "第4節:第二固定面の尺度付きの値")?;

    Ok(Section4::Template4_0(Section4_0 {
        section_bytes,
        section_number,
        number_of_after_template_points,
        product_definition_template_number,
        parameter_category,
        parameter_number,
        type_of_generating_process,
        background_process,
        generating_process_identifier,
        hours_after_data_cutoff,
        minutes_after_data_cutoff,
        indicator_of_unit_of_time_range,
        forecast_time,
        type_of_first_fixed_surface,
        scale_factor_of_first_fixed_surface,
        scaled_value_of_first_fixed_surface,
        type_of_second_fixed_surface,
        scale_factor_of_second_fixed_surface,
        scaled_value_of_second_fixed_surface,
    }))
}

pub struct Section4_50008 {
    /// 節の長さ（バイト数）
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// テンプレート直後の座標値の数
    pub number_of_after_template_points: u16,
    /// プロダクト定義テンプレート番号
    pub product_definition_template_number: u16,
    /// パラメータカテゴリー
    pub parameter_category: u8,
    /// パラメータ番号
    pub parameter_number: u8,
    /// 作成処理の種類
    pub type_of_generating_process: u8,
    /// 背景作成処理識別符
    pub background_process: u8,
    /// 予報の作成処理識別符
    pub generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    pub hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    pub minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    pub indicator_of_unit_of_time_range: u8,
    /// 予報時間
    pub forecast_time: i32,
    /// 第一固定面の種類
    pub type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    pub scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    pub scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    pub type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    pub scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    pub scaled_value_of_second_fixed_surface: u32,
    /// 全時間間隔の終了時(UTC)
    pub end_of_all_time_intervals: OffsetDateTime,
    /// 統計を算出するために使用した時間間隔を記述する期間の仕様の数
    pub number_of_time_range_specs: u8,
    /// 統計処理における欠測資料の総数
    pub number_of_missing_values: u32,
    /// 統計処理の種類
    pub type_of_stat_proc: u8,
    /// 統計処理の時間増分の種類
    pub type_of_stat_proc_time_increment: u8,
    /// 統計処理の時間の単位の指示符
    pub stat_proc_time_unit: u8,
    /// 統計処理した時間の長さ
    pub stat_proc_time_length: u32,
    /// 連続的な資料場間の増分に関する時間の単位の指示符
    pub successive_time_unit: u8,
    /// 連続的な資料場間の時間の増分
    pub successive_time_increment: u32,
    /// レーダー等運用情報その1
    pub radar_info1: u64,
    /// レーダー等運用情報その2
    pub radar_info2: u64,
    /// 雨量計運用情報
    pub rain_gauge_info: u64,
}

/// GRIB2ファイルから第4節:プロダクト定義節（テンプレート4.50008）を読み込む。
///
/// # 引数
///
/// * `reader` - ファイルリーダー
/// * `section_bytes` - 節の長さ
/// * `section_number` - 節番号
/// * `number_of_after_template_points` - テンプレート直後の座標値の数
/// * `product_definition_template_number` - プロダクト定義テンプレート番号
///
/// # 戻り値
///
/// * 第4節:プロダクト定義節
fn read_section4_50008<R: Read>(
    reader: &mut BufReader<R>,
    section_bytes: usize,
    section_number: u8,
    number_of_after_template_points: u16,
    product_definition_template_number: u16,
) -> Grib2Result<Section4> {
    // パラメータカテゴリー: 1バイト
    let parameter_category = read_u8(reader, "第4節:パラメータカテゴリー")?;
    // パラメータ番号: 1バイト
    let parameter_number = read_u8(reader, "第4節:パラメータ番号")?;
    // 作成処理の種類: 1バイト
    let type_of_generating_process = read_u8(reader, "第4節:作成処理の種類")?;
    // 背景作成処理識別符: 1バイト
    let background_process = read_u8(reader, "第4節:背景作成処理識別符")?;
    // 予報の作成処理識別符: 1バイト
    let generating_process_identifier = read_u8(reader, "第4節:予報の作成処理識別符")?;
    // 観測資料の参照時刻からの締切時間（時）: 2バイト
    let hours_after_data_cutoff = read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
    // 観測資料の参照時刻からの締切時間（分）: 1バイト
    let minutes_after_data_cutoff =
        read_u8(reader, "第4節:観測資料の参照時刻からの締切時間（分）")?;
    // 期間の単位の指示符: 1バイト
    let indicator_of_unit_of_time_range = read_u8(reader, "第4節:期間の単位の指示符")?;
    // 予報時間: 4バイト
    let forecast_time = read_i32(reader, "第4節:予報時間")?;
    // 第一固定面の種類: 1バイト
    let type_of_first_fixed_surface = read_u8(reader, "第4節:第一固定面の種類")?;
    // 第一固定面の尺度因子: 1バイト
    let scale_factor_of_first_fixed_surface = read_u8(reader, "第4節:第一固定面の尺度因子")?;
    // 第一固定面の尺度付きの値: 4バイト
    let scaled_value_of_first_fixed_surface = read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
    // 第二固定面の種類: 1バイト
    let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
    // 第二固定面の尺度因子: 1バイト
    let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
    // 第二固定面の尺度付きの値: 4バイト
    let scaled_value_of_second_fixed_surface = read_u32(reader, "第4節:第二固定面の尺度付きの値")?;
    // 全時間間隔の終了時: 7バイト
    let end_of_all_time_intervals = read_date_time(reader, "第4節:全時間間隔の終了時")?;
    // 統計を算出するために使用した時間間隔を記述する期間の仕様の数: 1バイト
    let number_of_time_range_specs = read_u8(
        reader,
        "第4節:統計を算出するために使用した時間間隔を記述する期間の仕様の数",
    )?;
    // 統計処理における欠測資料の総数: 4バイト
    let number_of_missing_values = read_u32(reader, "第4節:統計処理における欠測資料の総数")?;
    // 統計処理の種類: 1バイト
    let type_of_stat_proc = read_u8(reader, "第4節:統計処理の種類")?;
    // 統計処理の時間増分の種類: 1バイト
    let type_of_stat_proc_time_increment = read_u8(reader, "第4節:統計処理の時間増分の種類")?;
    // 統計処理の時間の単位の指示符: 1バイト
    let stat_proc_time_unit = read_u8(reader, "第4節:統計処理の時間の単位の指示符")?;
    // 統計処理した期間の長さ: 4バイト
    let stat_proc_time_length = read_u32(reader, "第4節:統計処理の時間増分の長さ")?;
    // 連続的な資料場間の増分に関する時間の単位の指示符: 1バイト
    let successive_time_unit = read_u8(
        reader,
        "第4節:連続的な資料場間の増分に関する時間の単位の指示符",
    )?;
    // 連続的な資料場間の時間の増分: 4バイト
    let successive_time_increment = read_u32(reader, "第4節:連続的な資料場間の時間の増分")?;
    // レーダー等運用情報その1: 8バイト
    let radar_info1 = read_u64(reader, "第4節:レーダー等運用情報その1")?;
    // レーダー等運用情報その2: 8バイト
    let radar_info2 = read_u64(reader, "第4節:レーダー等運用情報その2")?;
    // 雨量計運用情報: 8バイト
    let rain_gauge_info = read_u64(reader, "第4節:雨量計運用情報の読み込みに失敗しました。")?;

    Ok(Section4::Template4_50008(Section4_50008 {
        section_bytes,
        section_number,
        number_of_after_template_points,
        product_definition_template_number,
        parameter_category,
        parameter_number,
        type_of_generating_process,
        background_process,
        generating_process_identifier,
        hours_after_data_cutoff,
        minutes_after_data_cutoff,
        indicator_of_unit_of_time_range,
        forecast_time,
        type_of_first_fixed_surface,
        scale_factor_of_first_fixed_surface,
        scaled_value_of_first_fixed_surface,
        type_of_second_fixed_surface,
        scale_factor_of_second_fixed_surface,
        scaled_value_of_second_fixed_surface,
        end_of_all_time_intervals,
        number_of_time_range_specs,
        number_of_missing_values,
        type_of_stat_proc,
        type_of_stat_proc_time_increment,
        stat_proc_time_unit,
        stat_proc_time_length,
        successive_time_unit,
        successive_time_increment,
        radar_info1,
        radar_info2,
        rain_gauge_info,
    }))
}
