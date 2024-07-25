use std::io::{BufReader, Read};

use crate::readers::sections::TemplateReader;
use crate::readers::utils::{read_i32, read_u16, read_u32, read_u8, validate_u8};
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
