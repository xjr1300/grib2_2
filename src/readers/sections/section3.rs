/// 第3節: 格子系定義節
#[derive(Debug, Clone, Copy)]
pub struct Section3<T> {
    /// 節の長さ
    section_bytes: usize,
    /// 格子系定義の出典
    source_of_grid_definition: u8,
    /// 資料点数
    number_of_data_points: u32,
    /// 格子点数を定義するリストのオクテット数
    number_of_octets_for_number_of_points: u8,
    /// 格子点数を定義するリストの説明
    description_of_number_of_points: u8,
    /// 格子系定義テンプレート番号
    grid_definition_template_number: u16,
    /// テンプレート3
    template3: T,
}

impl<T> Section3<T> {
    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }

    /// 格子系定義の出典を返す。
    pub fn source_of_grid_definition(&self) -> u8 {
        self.source_of_grid_definition
    }

    /// 資料点数を返す。
    pub fn number_of_data_points(&self) -> u32 {
        self.number_of_data_points
    }

    /// 格子点数を定義するリストのオクテット数を返す。
    pub fn number_of_octets_for_number_of_points(&self) -> u8 {
        self.number_of_octets_for_number_of_points
    }

    /// 格子点数を定義するリストの説明を返す。
    pub fn description_of_number_of_points(&self) -> u8 {
        self.description_of_number_of_points
    }

    /// 格子系定義テンプレート番号を返す。
    pub fn grid_definition_template_number(&self) -> u16 {
        self.grid_definition_template_number
    }
}

/// テンプレート3.0
#[derive(Debug, Clone, Copy)]
pub struct Template3_0 {
    /// 地球の形状
    shape_of_earth: u8,
    /// 地球球体の半径の尺度因子
    scale_factor_of_radius_of_spherical_earth: u8,
    /// 地球球体の尺度付き半径
    scaled_value_of_radius_of_spherical_earth: u32,
    /// 地球回転楕円体の長軸の尺度因子
    scale_factor_of_earth_major_axis: u8,
    /// 地球回転楕円体の長軸の尺度付きの長さ
    scaled_value_of_earth_major_axis: u32,
    /// 地球回転楕円体の短軸の尺度因子
    scale_factor_of_earth_minor_axis: u8,
    /// 地球回転楕円体の短軸の尺度付きの長さ
    scaled_value_of_earth_minor_axis: u32,
    /// 緯線に沿った格子点数
    number_of_along_lat_points: u32,
    /// 経線に沿った格子点数
    number_of_along_lon_points: u32,
    /// 原作成領域の基本角
    basic_angle_of_initial_product_domain: u32,
    /// 端点の経度及び緯度並びに方向増分の定義に使われる基本角の細分
    subdivisions_of_basic_angle: u32,
    /// 最初の格子点の緯度（10e-6度単位）
    lat_of_first_grid_point: u32,
    /// 最初の格子点の経度（10e-6度単位）
    lon_of_first_grid_point: u32,
    /// 分解能及び成分フラグ
    resolution_and_component_flags: u8,
    /// 最後の格子点の緯度（10e-6度単位）
    lat_of_last_grid_point: u32,
    /// 最後の格子点の経度（10e-6度単位）
    lon_of_last_grid_point: u32,
    /// i方向（経度方向）の増分（10e-6度単位）
    i_direction_increment: u32,
    /// j方向（緯度方向）の増分（10e-6度単位）
    j_direction_increment: u32,
    /// 走査モード
    scanning_mode: u8,
}

pub type Section3_0 = Section3<Template3_0>;

impl Section3_0 {
    /// 地球の形状を返す。
    pub fn shape_of_earth(&self) -> u8 {
        self.template3.shape_of_earth
    }

    /// 地球球体の半径の尺度因子を返す。
    pub fn scale_factor_of_radius_of_spherical_earth(&self) -> u8 {
        self.template3.scale_factor_of_radius_of_spherical_earth
    }

    /// 地球球体の尺度付き半径を返す。
    pub fn scaled_value_of_radius_of_spherical_earth(&self) -> u32 {
        self.template3.scaled_value_of_radius_of_spherical_earth
    }

    /// 地球回転楕円体の長軸の尺度因子を返す。
    pub fn scale_factor_of_major_axis(&self) -> u8 {
        self.template3.scale_factor_of_earth_major_axis
    }

    /// 地球回転楕円体の長軸の尺度付きの長さを返す。
    pub fn scaled_value_of_earth_major_axis(&self) -> u32 {
        self.template3.scaled_value_of_earth_major_axis
    }

    /// 地球回転楕円体の短軸の尺度因子を返す。
    pub fn scale_factor_of_minor_axis(&self) -> u8 {
        self.template3.scale_factor_of_earth_minor_axis
    }

    /// 地球回転楕円体の短軸の尺度付きの長さを返す。
    pub fn scaled_value_of_earth_minor_axis(&self) -> u32 {
        self.template3.scaled_value_of_earth_minor_axis
    }

    /// 緯線に沿った格子点数を返す。
    pub fn number_of_along_lat_points(&self) -> u32 {
        self.template3.number_of_along_lat_points
    }

    /// 経線に沿った格子点数を返す。
    pub fn number_of_along_lon_points(&self) -> u32 {
        self.template3.number_of_along_lon_points
    }

    /// 原作成領域の基本角を返す。
    pub fn basic_angle_of_initial_product_domain(&self) -> u32 {
        self.template3.basic_angle_of_initial_product_domain
    }

    /// 端点の経度及び緯度並びに方向増分の定義に使われる基本角の細分を返す。
    pub fn subdivisions_of_basic_angle(&self) -> u32 {
        self.template3.subdivisions_of_basic_angle
    }

    /// 最初の格子点の緯度（10e-6度単位）を返す。
    pub fn lat_of_first_grid_point(&self) -> u32 {
        self.template3.lat_of_first_grid_point
    }

    /// 最初の格子点の経度（10e-6度単位）を返す。
    pub fn lon_of_first_grid_point(&self) -> u32 {
        self.template3.lon_of_first_grid_point
    }

    /// 分解能及び成分フラグを返す。
    pub fn resolution_and_component_flags(&self) -> u8 {
        self.template3.resolution_and_component_flags
    }

    /// 最後の格子点の緯度（10e-6度単位）を返す。
    pub fn lat_of_last_grid_point(&self) -> u32 {
        self.template3.lat_of_last_grid_point
    }

    /// 最後の格子点の経度（10e-6度単位）を返す。
    pub fn lon_of_last_grid_point(&self) -> u32 {
        self.template3.lon_of_last_grid_point
    }

    /// i方向（経度方向）の増分（10e-6度単位）を返す。
    pub fn i_direction_increment(&self) -> u32 {
        self.template3.i_direction_increment
    }

    /// j方向（緯度方向）の増分（10e-6度単位）を返す。
    pub fn j_direction_increment(&self) -> u32 {
        self.template3.j_direction_increment
    }

    /// 走査モードを返す。
    pub fn scanning_mode(&self) -> u8 {
        self.template3.scanning_mode
    }
}
