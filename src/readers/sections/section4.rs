use std::io::{BufReader, Read};

use time::OffsetDateTime;

use crate::readers::sections::TemplateReader;
use crate::readers::utils::{
    read_date_time, read_i32, read_u16, read_u32, read_u64, read_u8, validate_u8,
};
use crate::Grib2Result;

/// 第4節:プロダクト定義節
pub struct Section4<T>
where
    T: TemplateReader,
{
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// テンプレート直後の座標値の数
    number_of_after_template_points: u16,
    /// プロダクト定義テンプレート番号
    product_definition_template_number: u16,
    /// テンプレート4
    template4: T,
}

impl<T> Section4<T>
where
    T: TemplateReader,
{
    /// 第4節:プロダクト定義節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第4節:節の長さ")? as usize;
        // 節番号: 1バイト
        validate_u8(reader, 4, "第4節:節番号")?;
        // テンプレート直後の座標値の数: 2バイト
        let number_of_after_template_points =
            read_u16(reader, "第4節:テンプレート直後の座標値の数")?;
        // プロダクト定義テンプレート番号: 2バイト
        let product_definition_template_number =
            read_u16(reader, "第4節:プロダクト定義テンプレート番号")?;
        // テンプレート4
        let template4 = T::from_reader(reader)?;

        Ok(Self {
            section_bytes,
            number_of_after_template_points,
            product_definition_template_number,
            template4,
        })
    }

    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }

    /// テンプレート直後の座標値の数を返す。
    pub fn number_of_after_template_points(&self) -> u16 {
        self.number_of_after_template_points
    }

    /// プロダクト定義テンプレート番号を返す。
    pub fn product_definition_template_number(&self) -> u16 {
        self.product_definition_template_number
    }
}

/// テンプレート4.0
#[derive(Debug, Clone, Copy)]
pub struct Template4_0 {
    /// パラメータカテゴリー
    parameter_category: u8,
    /// パラメータ番号
    parameter_number: u8,
    /// 作成処理の種類
    type_of_generating_process: u8,
    /// 背景作成処理識別符
    background_process: u8,
    /// 予報の作成処理識別符
    generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    indicator_of_unit_of_time_range: u8,
    /// 予報時間
    forecast_time: i32,
    /// 第一固定面の種類
    type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    scaled_value_of_second_fixed_surface: u32,
}

impl TemplateReader for Template4_0 {
    fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
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
        let hours_after_data_cutoff =
            read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
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
        let scaled_value_of_first_fixed_surface =
            read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
        // 第二固定面の種類: 1バイト
        let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
        // 第二固定面の尺度因子: 1バイト
        let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
        // 第二固定面の尺度付きの値: 4バイト
        let scaled_value_of_second_fixed_surface =
            read_u32(reader, "第4節:第二固定面の尺度付きの値")?;

        Ok(Self {
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
        })
    }
}

pub type Section4_0 = Section4<Template4_0>;

impl Section4_0 {
    /// パラメータカテゴリーを返す。
    pub fn parameter_category(&self) -> u8 {
        self.template4.parameter_category
    }
    /// パラメータ番号を返す。
    pub fn parameter_number(&self) -> u8 {
        self.template4.parameter_number
    }
    /// 作成処理の種類を返す。
    pub fn type_of_generating_process(&self) -> u8 {
        self.template4.type_of_generating_process
    }
    /// 背景作成処理識別符を返す。
    pub fn background_process(&self) -> u8 {
        self.template4.background_process
    }
    /// 予報の作成処理識別符を返す。
    pub fn generating_process_identifier(&self) -> u8 {
        self.template4.generating_process_identifier
    }
    /// 観測資料の参照時刻からの締切時間（時）を返す。
    pub fn hours_after_data_cutoff(&self) -> u16 {
        self.template4.hours_after_data_cutoff
    }
    /// 観測資料の参照時刻からの締切時間（分）を返す。
    pub fn minutes_after_data_cutoff(&self) -> u8 {
        self.template4.minutes_after_data_cutoff
    }
    /// 期間の単位の指示符を返す。
    pub fn indicator_of_unit_of_time_range(&self) -> u8 {
        self.template4.indicator_of_unit_of_time_range
    }
    /// 予報時間を返す。
    pub fn forecast_time(&self) -> i32 {
        self.template4.forecast_time
    }
    /// 第一固定面の種類を返す。
    pub fn type_of_first_fixed_surface(&self) -> u8 {
        self.template4.type_of_first_fixed_surface
    }
    /// 第一固定面の尺度因子を返す。
    pub fn scale_factor_of_first_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_first_fixed_surface
    }
    /// 第一固定面の尺度付きの値を返す。
    pub fn scaled_value_of_first_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_first_fixed_surface
    }
    /// 第二固定面の種類を返す。
    pub fn type_of_second_fixed_surface(&self) -> u8 {
        self.template4.type_of_second_fixed_surface
    }
    /// 第二固定面の尺度因子を返す。
    pub fn scale_factor_of_second_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_second_fixed_surface
    }
    /// 第二固定面の尺度付きの値を返す。
    pub fn scaled_value_of_second_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_second_fixed_surface
    }
}

/// テンプレート4.50000
#[derive(Debug, Clone, Copy)]
pub struct Template4_50000 {
    /// パラメータカテゴリー
    parameter_category: u8,
    /// パラメータ番号
    parameter_number: u8,
    /// 作成処理の種類
    type_of_generating_process: u8,
    /// 背景作成処理識別符
    background_process: u8,
    /// 解析又は予報の作成処理識別符
    generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    indicator_of_unit_of_time_range: u8,
    /// 予報時間
    forecast_time: i32,
    /// 第一固定面の種類
    type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    scaled_value_of_second_fixed_surface: u32,
    /// 資料作成に用いた関連資料の名称1
    source_document1: u8,
    /// 上記関連資料の解析時刻と参照時刻との差（時）1
    hours_from_source_document1: u16,
    /// 上記関連資料の解析時刻と参照時刻との差（分）1
    minutes_from_source_document1: u8,
    /// 資料作成に用いた関連資料の名称2
    source_document2: u8,
    /// 上記関連資料の解析時刻と参照時刻との差（時）2
    hours_from_source_document2: u16,
    /// 上記関連資料の解析時刻と参照時刻との差（分）2
    minutes_from_source_document2: u8,
}

impl TemplateReader for Template4_50000 {
    /// テンプレート4.50000を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2リーダー
    ///
    /// # 戻り値
    ///
    /// * テンプレート4.50000
    fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // パラメータカテゴリー: 1バイト
        let parameter_category = read_u8(reader, "第4節:パラメータカテゴリー")?;
        // パラメータ番号: 1バイト
        let parameter_number = read_u8(reader, "第4節:パラメータ番号")?;
        // 作成処理の種類: 1バイト
        let type_of_generating_process = read_u8(reader, "第4節:作成処理の種類")?;
        // 背景作成処理識別符: 1バイト
        let background_process = read_u8(reader, "第4節:背景作成処理識別符")?;
        // 解析又は予報の作成処理識別符: 1バイト
        let generating_process_identifier = read_u8(reader, "第4節:解析又は予報の作成処理識別符")?;
        // 観測資料の参照時刻からの締切時間（時）: 2バイト
        let hours_after_data_cutoff =
            read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
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
        let scaled_value_of_first_fixed_surface =
            read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
        // 第二固定面の種類: 1バイト
        let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
        // 第二固定面の尺度因子: 1バイト
        let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
        // 第二固定面の尺度付きの値: 4バイト
        let scaled_value_of_second_fixed_surface =
            read_u32(reader, "第4節:第二固定面の尺度付きの値")?;
        // 資料作成に用いた関連資料の名称1: 1バイト
        let source_document1 = read_u8(reader, "第4節:資料作成に用いた関連資料の名称1")?;
        // 上記関連資料の解析時刻と参照時刻との差（時）1: 2バイト
        let hours_from_source_document1 =
            read_u16(reader, "第4節:記関連資料の解析時刻と参照時刻との差（時）1")?;
        // 上記関連資料の解析時刻と参照時刻との差（分）1: 1バイト
        let minutes_from_source_document1 =
            read_u8(reader, "第4節:記関連資料の解析時刻と参照時刻との差（分）1")?;
        // 資料作成に用いた関連資料の名称2: 1バイト
        let source_document2 = read_u8(reader, "第4節:資料作成に用いた関連資料の名称2")?;
        // 上記関連資料の解析時刻と参照時刻との差（時）2: 2バイト
        let hours_from_source_document2 =
            read_u16(reader, "第4節:記関連資料の解析時刻と参照時刻との差（時）2")?;
        // 上記関連資料の解析時刻と参照時刻との差（分）2: 1バイト
        let minutes_from_source_document2 =
            read_u8(reader, "第4節:記関連資料の解析時刻と参照時刻との差（分）2")?;

        Ok(Self {
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
            source_document1,
            hours_from_source_document1,
            minutes_from_source_document1,
            source_document2,
            hours_from_source_document2,
            minutes_from_source_document2,
        })
    }
}

pub type Section4_50000 = Section4<Template4_50000>;

impl Section4_50000 {
    /// パラメータカテゴリーを返す。
    pub fn parameter_category(&self) -> u8 {
        self.template4.parameter_category
    }

    /// パラメータ番号を返す。
    pub fn parameter_number(&self) -> u8 {
        self.template4.parameter_number
    }

    /// 作成処理の種類を返す。
    pub fn type_of_generating_process(&self) -> u8 {
        self.template4.type_of_generating_process
    }

    /// 背景作成処理識別符を返す。
    pub fn background_process(&self) -> u8 {
        self.template4.background_process
    }

    /// 予報の作成処理識別符を返す。
    pub fn generating_process_identifier(&self) -> u8 {
        self.template4.generating_process_identifier
    }

    /// 観測資料の参照時刻からの締切時間（時）を返す。
    pub fn hours_after_data_cutoff(&self) -> u16 {
        self.template4.hours_after_data_cutoff
    }

    /// 観測資料の参照時刻からの締切時間（分）を返す。
    pub fn minutes_after_data_cutoff(&self) -> u8 {
        self.template4.minutes_after_data_cutoff
    }

    /// 期間の単位の指示符を返す。
    pub fn indicator_of_unit_of_time_range(&self) -> u8 {
        self.template4.indicator_of_unit_of_time_range
    }

    /// 予報時間を返す。
    pub fn forecast_time(&self) -> i32 {
        self.template4.forecast_time
    }

    /// 第一固定面の種類を返す。
    pub fn type_of_first_fixed_surface(&self) -> u8 {
        self.template4.type_of_first_fixed_surface
    }

    /// 第一固定面の尺度因子を返す。
    pub fn scale_factor_of_first_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_first_fixed_surface
    }

    /// 第一固定面の尺度付きの値を返す。
    pub fn scaled_value_of_first_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_first_fixed_surface
    }

    /// 第二固定面の種類を返す。
    pub fn type_of_second_fixed_surface(&self) -> u8 {
        self.template4.type_of_second_fixed_surface
    }

    /// 第二固定面の尺度因子を返す。
    pub fn scale_factor_of_second_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_second_fixed_surface
    }

    /// 第二固定面の尺度付きの値を返す。
    pub fn scaled_value_of_second_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_second_fixed_surface
    }

    /// 資料作成に用いた関連資料の名称1を返す。
    pub fn source_document1(&self) -> u8 {
        self.template4.source_document1
    }

    /// 資料作成に用いた関連資料の時間（時）1を返す。
    pub fn hours_from_source_document1(&self) -> u16 {
        self.template4.hours_from_source_document1
    }

    /// 資料作成に用いた関連資料の時間（分）1を返す。
    pub fn minutes_from_source_document1(&self) -> u8 {
        self.template4.minutes_from_source_document1
    }

    /// 資料作成に用いた関連資料の名称2を返す。
    pub fn source_document2(&self) -> u8 {
        self.template4.source_document2
    }

    /// 資料作成に用いた関連資料の時間（時）2を返す。
    pub fn hours_from_source_document2(&self) -> u16 {
        self.template4.hours_from_source_document2
    }

    /// 資料作成に用いた関連資料の時間（分）2を返す。
    pub fn minutes_from_source_document2(&self) -> u8 {
        self.template4.minutes_from_source_document2
    }
}

/// テンプレート4.50008
#[derive(Debug, Clone, Copy)]
pub struct Template4_50008 {
    /// パラメータカテゴリー
    parameter_category: u8,
    /// パラメータ番号
    parameter_number: u8,
    /// 作成処理の種類
    type_of_generating_process: u8,
    /// 背景作成処理識別符
    background_process: u8,
    /// 予報の作成処理識別符
    generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    indicator_of_unit_of_time_range: u8,
    /// 予報時間
    forecast_time: i32,
    /// 第一固定面の種類
    type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    scaled_value_of_second_fixed_surface: u32,
    /// 全時間間隔の終了時(UTC)
    end_of_all_time_intervals: OffsetDateTime,
    /// 統計を算出するために使用した時間間隔を記述する期間の仕様の数
    number_of_time_range_specs: u8,
    /// 統計処理における欠測資料の総数
    number_of_missing_values: u32,
    /// 統計処理の種類
    type_of_stat_proc: u8,
    /// 統計処理の時間増分の種類
    type_of_stat_proc_time_increment: u8,
    /// 統計処理の時間の単位の指示符
    stat_proc_time_unit: u8,
    /// 統計処理した時間の長さ
    stat_proc_time_length: u32,
    /// 連続的な資料場間の増分に関する時間の単位の指示符
    successive_time_unit: u8,
    /// 連続的な資料場間の時間の増分
    successive_time_increment: u32,
    /// レーダー等運用情報その1
    radar_info1: u64,
    /// レーダー等運用情報その2
    radar_info2: u64,
    /// 雨量計運用情報
    rain_gauge_info: u64,
}

impl TemplateReader for Template4_50008 {
    fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
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
        let hours_after_data_cutoff =
            read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
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
        let scaled_value_of_first_fixed_surface =
            read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
        // 第二固定面の種類: 1バイト
        let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
        // 第二固定面の尺度因子: 1バイト
        let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
        // 第二固定面の尺度付きの値: 4バイト
        let scaled_value_of_second_fixed_surface =
            read_u32(reader, "第4節:第二固定面の尺度付きの値")?;
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

        Ok(Self {
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
        })
    }
}

pub type Section4_50008 = Section4<Template4_50008>;

impl Section4_50008 {
    /// パラメータカテゴリーを返す。
    pub fn parameter_category(&self) -> u8 {
        self.template4.parameter_category
    }
    /// パラメータ番号を返す。
    pub fn parameter_number(&self) -> u8 {
        self.template4.parameter_number
    }
    /// 作成処理の種類を返す。
    pub fn type_of_generating_process(&self) -> u8 {
        self.template4.type_of_generating_process
    }
    /// 背景作成処理識別符を返す。
    pub fn background_process(&self) -> u8 {
        self.template4.background_process
    }
    /// 予報の作成処理識別符を返す。
    pub fn generating_process_identifier(&self) -> u8 {
        self.template4.generating_process_identifier
    }
    /// 観測資料の参照時刻からの締切時間（時）を返す。
    pub fn hours_after_data_cutoff(&self) -> u16 {
        self.template4.hours_after_data_cutoff
    }
    /// 観測資料の参照時刻からの締切時間（分）を返す。
    pub fn minutes_after_data_cutoff(&self) -> u8 {
        self.template4.minutes_after_data_cutoff
    }
    /// 期間の単位の指示符を返す。
    pub fn indicator_of_unit_of_time_range(&self) -> u8 {
        self.template4.indicator_of_unit_of_time_range
    }
    /// 予報時間を返す。
    pub fn forecast_time(&self) -> i32 {
        self.template4.forecast_time
    }
    /// 第一固定面の種類を返す。
    pub fn type_of_first_fixed_surface(&self) -> u8 {
        self.template4.type_of_first_fixed_surface
    }
    /// 第一固定面の尺度因子を返す。
    pub fn scale_factor_of_first_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_first_fixed_surface
    }
    /// 第一固定面の尺度付きの値を返す。
    pub fn scaled_value_of_first_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_first_fixed_surface
    }
    /// 第二固定面の種類を返す。
    pub fn type_of_second_fixed_surface(&self) -> u8 {
        self.template4.type_of_second_fixed_surface
    }
    /// 第二固定面の尺度因子を返す。
    pub fn scale_factor_of_second_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_second_fixed_surface
    }
    /// 第二固定面の尺度付きの値を返す。
    pub fn scaled_value_of_second_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_second_fixed_surface
    }
    /// 全時間間隔の終了時(UTC)を返す。
    pub fn end_of_all_time_intervals(&self) -> OffsetDateTime {
        self.template4.end_of_all_time_intervals
    }
    /// 統計を算出するために使用した時間間隔を記述する期間の仕様の数を返す。
    pub fn number_of_time_range_specs(&self) -> u8 {
        self.template4.number_of_time_range_specs
    }
    /// 統計処理における欠測資料の総数を返す。
    pub fn number_of_missing_values(&self) -> u32 {
        self.template4.number_of_missing_values
    }
    /// 統計処理の種類を返す。
    pub fn type_of_stat_proc(&self) -> u8 {
        self.template4.type_of_stat_proc
    }
    /// 統計処理の時間増分の種類を返す。
    pub fn type_of_stat_proc_time_increment(&self) -> u8 {
        self.template4.type_of_stat_proc_time_increment
    }
    /// 統計処理の時間の単位の指示符を返す。
    pub fn stat_proc_time_unit(&self) -> u8 {
        self.template4.stat_proc_time_unit
    }
    /// 統計処理した時間の長さを返す。
    pub fn stat_proc_time_length(&self) -> u32 {
        self.template4.stat_proc_time_length
    }
    /// 連続的な資料場間の増分に関する時間の単位の指示符を返す。
    pub fn successive_time_unit(&self) -> u8 {
        self.template4.successive_time_unit
    }
    /// 連続的な資料場間の時間の増分を返す。
    pub fn successive_time_increment(&self) -> u32 {
        self.template4.successive_time_increment
    }
    /// レーダー等運用情報その1を返す。
    pub fn radar_info1(&self) -> u64 {
        self.template4.radar_info1
    }
    /// レーダー等運用情報その2を返す。
    pub fn radar_info2(&self) -> u64 {
        self.template4.radar_info2
    }
    /// 雨量計運用情報を返す。
    pub fn rain_gauge_info(&self) -> u64 {
        self.template4.rain_gauge_info
    }
}

/// テンプレート4.50009
#[derive(Debug, Clone)]
pub struct Template4_50009 {
    /// パラメータカテゴリー
    parameter_category: u8,
    /// パラメータ番号
    parameter_number: u8,
    /// 作成処理の種類
    type_of_generating_process: u8,
    /// 背景作成処理識別符
    background_process: u8,
    /// 予報の作成処理識別符
    generating_process_identifier: u8,
    /// 観測資料の参照時刻からの締切時間（時）
    hours_after_data_cutoff: u16,
    /// 観測資料の参照時刻からの締切時間（分）
    minutes_after_data_cutoff: u8,
    /// 期間の単位の指示符
    indicator_of_unit_of_time_range: u8,
    /// 予報時間
    forecast_time: i32,
    /// 第一固定面の種類
    type_of_first_fixed_surface: u8,
    /// 第一固定面の尺度因子
    scale_factor_of_first_fixed_surface: u8,
    /// 第一固定面の尺度付きの値
    scaled_value_of_first_fixed_surface: u32,
    /// 第二固定面の種類
    type_of_second_fixed_surface: u8,
    /// 第二固定面の尺度因子
    scale_factor_of_second_fixed_surface: u8,
    /// 第二固定面の尺度付きの値
    scaled_value_of_second_fixed_surface: u32,
    /// 全時間間隔の終了時(UTC)
    end_of_all_time_intervals: OffsetDateTime,
    /// 統計を算出するために使用した時間間隔を記述する期間の仕様の数
    number_of_time_range_specs: u8,
    /// 統計処理における欠測資料の総数
    number_of_missing_values: u32,
    /// 統計処理の種類
    type_of_stat_proc: u8,
    /// 統計処理の時間増分の種類
    type_of_stat_proc_time_increment: u8,
    /// 統計処理の時間の単位の指示符
    stat_proc_time_unit: u8,
    /// 統計処理した時間の長さ
    stat_proc_time_length: u32,
    /// 連続的な資料場間の増分に関する時間の単位の指示符
    successive_time_unit: u8,
    /// 連続的な資料場間の時間の増分
    successive_time_increment: u32,
    /// レーダー等運用情報その1
    radar_info1: u64,
    /// レーダー等運用情報その2
    radar_info2: u64,
    /// 雨量計運用情報
    rain_gauge_info: u64,
    /// メソモデル予想値の結合比率の計算領域数
    number_of_calculation_areas: u16,
    /// メソモデル予想値の結合比率の尺度因子
    scale_factor_of_combined_ratio: u8,
    /// 各領域のメソモデル予想値の結合比率
    combined_ratios_of_forecast_areas: Vec<u16>,
}

impl TemplateReader for Template4_50009 {
    fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
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
        let hours_after_data_cutoff =
            read_u16(reader, "第4節:観測資料の参照時刻からの締切時間（時）")?;
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
        let scaled_value_of_first_fixed_surface =
            read_u32(reader, "第4節:第一固定面の尺度付きの値")?;
        // 第二固定面の種類: 1バイト
        let type_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の種類")?;
        // 第二固定面の尺度因子: 1バイト
        let scale_factor_of_second_fixed_surface = read_u8(reader, "第4節:第二固定面の尺度因子")?;
        // 第二固定面の尺度付きの値: 4バイト
        let scaled_value_of_second_fixed_surface =
            read_u32(reader, "第4節:第二固定面の尺度付きの値")?;
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
        // メソモデル予想値の結合比率の計算領域数
        let number_of_calculation_areas =
            read_u16(reader, "メソモデル予想値の結合比率の計算領域数")?;
        // メソモデル予想値の結合比率の尺度因子
        let scale_factor_of_combined_ratio =
            read_u8(reader, "メソモデル予想値の結合比率の尺度因子")?;
        // 各領域のメソモデル予想値の結合比率
        let mut combined_ratios_of_forecast_areas = vec![];
        for _ in 0..number_of_calculation_areas {
            combined_ratios_of_forecast_areas
                .push(read_u16(reader, "各領域のメソモデル予想値の結合比率")?);
        }

        Ok(Self {
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
            number_of_calculation_areas,
            scale_factor_of_combined_ratio,
            combined_ratios_of_forecast_areas,
        })
    }
}

pub type Section4_50009 = Section4<Template4_50009>;

impl Section4_50009 {
    /// パラメータカテゴリーを返す。
    pub fn parameter_category(self) -> u8 {
        self.template4.parameter_category
    }
    /// パラメータ番号を返す。
    pub fn parameter_number(&self) -> u8 {
        self.template4.parameter_number
    }
    /// 作成処理の種類を返す。
    pub fn type_of_generating_process(&self) -> u8 {
        self.template4.type_of_generating_process
    }
    /// 背景作成処理識別符を返す。
    pub fn background_process(&self) -> u8 {
        self.template4.background_process
    }
    /// 予報の作成処理識別符を返す。
    pub fn generating_process_identifier(&self) -> u8 {
        self.template4.generating_process_identifier
    }
    /// 観測資料の参照時刻からの締切時間（時）を返す。
    pub fn hours_after_data_cutoff(&self) -> u16 {
        self.template4.hours_after_data_cutoff
    }
    /// 観測資料の参照時刻からの締切時間（分）を返す。
    pub fn minutes_after_data_cutoff(&self) -> u8 {
        self.template4.minutes_after_data_cutoff
    }
    /// 期間の単位の指示符を返す。
    pub fn indicator_of_unit_of_time_range(&self) -> u8 {
        self.template4.indicator_of_unit_of_time_range
    }
    /// 予報時間を返す。
    pub fn forecast_time(&self) -> i32 {
        self.template4.forecast_time
    }
    /// 第一固定面の種類を返す。
    pub fn type_of_first_fixed_surface(&self) -> u8 {
        self.template4.type_of_first_fixed_surface
    }
    /// 第一固定面の尺度因子を返す。
    pub fn scale_factor_of_first_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_first_fixed_surface
    }
    /// 第一固定面の尺度付きの値を返す。
    pub fn scaled_value_of_first_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_first_fixed_surface
    }
    /// 第二固定面の種類を返す。
    pub fn type_of_second_fixed_surface(&self) -> u8 {
        self.template4.type_of_second_fixed_surface
    }
    /// 第二固定面の尺度因子を返す。
    pub fn scale_factor_of_second_fixed_surface(&self) -> u8 {
        self.template4.scale_factor_of_second_fixed_surface
    }
    /// 第二固定面の尺度付きの値を返す。
    pub fn scaled_value_of_second_fixed_surface(&self) -> u32 {
        self.template4.scaled_value_of_second_fixed_surface
    }
    /// 全時間間隔の終了時(UTC)を返す。
    pub fn end_of_all_time_intervals(&self) -> OffsetDateTime {
        self.template4.end_of_all_time_intervals
    }
    /// 統計を算出するために使用した時間間隔を記述する期間の仕様の数を返す。
    pub fn number_of_time_range_specs(&self) -> u8 {
        self.template4.number_of_time_range_specs
    }
    /// 統計処理における欠測資料の総数を返す。
    pub fn number_of_missing_values(&self) -> u32 {
        self.template4.number_of_missing_values
    }
    /// 統計処理の種類を返す。
    pub fn type_of_stat_proc(&self) -> u8 {
        self.template4.type_of_stat_proc
    }
    /// 統計処理の時間増分の種類を返す。
    pub fn type_of_stat_proc_time_increment(&self) -> u8 {
        self.template4.type_of_stat_proc_time_increment
    }
    /// 統計処理の時間の単位の指示符を返す。
    pub fn stat_proc_time_unit(&self) -> u8 {
        self.template4.stat_proc_time_unit
    }
    /// 統計処理した時間の長さを返す。
    pub fn stat_proc_time_length(&self) -> u32 {
        self.template4.stat_proc_time_length
    }
    /// 連続的な資料場間の増分に関する時間の単位の指示符を返す。
    pub fn successive_time_unit(&self) -> u8 {
        self.template4.successive_time_unit
    }
    /// 連続的な資料場間の時間の増分を返す。
    pub fn successive_time_increment(&self) -> u32 {
        self.template4.successive_time_increment
    }
    /// レーダー等運用情報その1を返す。
    pub fn radar_info1(&self) -> u64 {
        self.template4.radar_info1
    }
    /// レーダー等運用情報その2を返す。
    pub fn radar_info2(&self) -> u64 {
        self.template4.radar_info2
    }
    /// 雨量計運用情報を返す。
    pub fn rain_gauge_info(&self) -> u64 {
        self.template4.rain_gauge_info
    }
    /// メソモデル予想値の結合比率の計算領域数を返す。
    pub fn number_of_calculation_areas(&self) -> u16 {
        self.template4.number_of_calculation_areas
    }
    /// メソモデル予想値の結合比率の尺度因子を返す。
    pub fn scale_factor_of_combined_ratio(&self) -> u8 {
        self.template4.scale_factor_of_combined_ratio
    }
    /// 各領域のメソモデル予想値の結合比率を返す。
    pub fn combined_ratios_of_forecast_areas(&self) -> &[u16] {
        &self.template4.combined_ratios_of_forecast_areas
    }
}
