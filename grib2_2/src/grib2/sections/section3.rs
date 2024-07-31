use std::io::{BufReader, Read};

use crate::readers::utils::{read_u16, read_u32, read_u8, validate_u8};
use crate::{Grib2Error, Grib2Result};

/// 第3節:格子系定義節
pub enum Section3 {
    /// テンプレート3.0
    Template3_0(Section3_0),
}

impl Section3 {
    /// GRIB2ファイルから第3節:格子系定義節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - GRIB2ファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第3節:格子系定義節
    pub(crate) fn from_reader<R: Read>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        // 節の長さ: 4バイト
        let section_bytes = read_u32(reader, "第3節:節の長さ")? as usize;
        // 節番号: 1バイト
        let section_number = validate_u8(reader, 3, "第3節:節番号")?;
        // 格子系定義の出典: 1バイト
        let source_of_grid_definition = read_u8(reader, "第3節:格子系定義の出典")?;
        // 資料点数: 4バイト
        let number_of_data_points = read_u32(reader, "第3節:格子点数")?;
        // 格子点数を定義するリストのオクテット数: 1バイト
        let number_of_octets_for_number_of_points =
            read_u8(reader, "第3節:格子点数を定義するリストのオクテット数")?;
        // 格子点数を定義するリストの節明
        let description_of_number_of_points =
            read_u8(reader, "第3節:格子点数を定義するリストの節明")?;
        // 格子系定義テンプレート番号: 2バイト
        let grid_definition_template_number = read_u16(reader, "第3節:格子系定義テンプレート番号")?;
        match grid_definition_template_number {
            0 => read_section3_0(reader, section_bytes, section_number, source_of_grid_definition, number_of_data_points, number_of_octets_for_number_of_points, description_of_number_of_points, grid_definition_template_number),
            _ => Err(Grib2Error::NotImplemented(format!("第３節の格子系定義テンプレート番号`{grid_definition_template_number}`は未実装です。").into())),
        }
    }

    /// 資料点数を返す。
    ///
    /// # 戻り値
    ///
    /// * 資料点数
    pub fn number_of_points(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.number_of_points),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}は資料点数を記録していません。").into(),
            //)),
        }
    }

    /// 最初の格子点の緯度（1e-6度単位）を返す。
    ///
    /// # 戻り値
    ///
    /// * 最初の格子点の緯度（1e-6度単位）
    pub fn lat_of_first_grid_point(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.lat_of_first_grid_point),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}は最初の格子点の緯度を記録していません。").into(),
            //)),
        }
    }

    /// 最初の格子点の経度（1e-6度単位）を返す。
    ///
    /// # 戻り値
    ///
    /// * 最初の格子点の経度（1e-6度単位）
    pub fn lon_of_first_grid_point(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.lon_of_first_grid_point),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}は最初の格子点の経度を記録していません。").into(),
            //)),
        }
    }

    /// 最後の格子点の経度（1e-6度単位）を返す。
    ///
    /// # 戻り値
    ///
    /// * 最後の格子点の経度（1e-6度単位）
    pub fn lon_of_last_grid_point(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.lon_of_last_grid_point),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}は最初の格子点の経度を記録していません。").into(),
            //)),
        }
    }

    /// i方向（経度方向）の増分値（1e-6度単位）を返す。
    ///
    /// # 戻り値
    ///
    /// * i方向（経度方向）の増分値（1e-6度単位）
    pub fn i_direction_increment(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.i_direction_increment),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}はi方向の増分値を記録していません。").into(),
            //)),
        }
    }

    /// j方向（緯度方向）の増分（1e-6度単位）を返す。
    ///
    /// # 戻り値
    ///
    /// * j方向（緯度方向）の増分（1e-6度単位）
    pub fn j_direction_increment(&self) -> Grib2Result<u32> {
        match self {
            Self::Template3_0(s) => Ok(s.j_direction_increment),
            //_ => Err(Grib2Error::RuntimeError(
            //    format!("{self}はj方向の増分値を記録していません。").into(),
            //)),
        }
    }
}

impl std::fmt::Display for Section3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Section3::Template3_0(_) => write!(f, "第3節テンプレート3.0"),
        }
    }
}

pub struct Section3_0 {
    /// 節の長さ
    pub section_bytes: usize,
    /// 節番号
    pub section_number: u8,
    /// 格子系定義の出典
    pub source_of_grid_definition: u8,
    /// 資料点数
    pub number_of_points: u32,
    /// 格子点数を定義するリストのオクテット数
    pub number_of_octets_for_number_of_points: u8,
    /// 格子点数を定義するリストの節明
    pub description_of_number_of_points: u8,
    /// 格子系定義テンプレート番号
    pub grid_definition_template_number: u16,
    /// 地球の形状
    pub shape_of_earth: u8,
    /// 地球球体の半径の尺度因子
    pub scale_factor_of_radius_of_spherical_earth: u8,
    /// 地球球体の尺度付き半径
    pub scaled_value_of_radius_of_spherical_earth: u32,
    /// 地球回転楕円体の長軸の尺度因子
    pub scale_factor_of_earth_major_axis: u8,
    /// 地球回転楕円体の長軸の尺度付きの長さ
    pub scaled_value_of_earth_major_axis: u32,
    /// 地球回転楕円体の短軸の尺度因子
    pub scale_factor_of_earth_minor_axis: u8,
    /// 地球回転楕円体の短軸の尺度付きの長さ
    pub scaled_value_of_earth_minor_axis: u32,
    /// 緯線に沿った格子点数
    pub number_of_along_lat_points: u32,
    /// 経線に沿った格子点数
    pub number_of_along_lon_points: u32,
    /// 原作成領域の基本角
    pub basic_angle_of_initial_product_domain: u32,
    /// 端点の経度及び緯度並びに方向増分の定義に使われる基本角の細分
    pub subdivisions_of_basic_angle: u32,
    /// 最初の格子点の緯度（1e-6度単位）
    pub lat_of_first_grid_point: u32,
    /// 最初の格子点の経度（1e-6度単位）
    pub lon_of_first_grid_point: u32,
    /// 分解能及び成分フラグ
    pub resolution_and_component_flags: u8,
    /// 最後の格子点の緯度（1e-6度単位）
    pub lat_of_last_grid_point: u32,
    /// 最後の格子点の経度（1e-6度単位）
    pub lon_of_last_grid_point: u32,
    /// i方向（経度方向）の増分（1e-6度単位）
    pub i_direction_increment: u32,
    /// j方向（緯度方向）の増分（1e-6度単位）
    pub j_direction_increment: u32,
    /// 走査モード
    pub scanning_mode: u8,
}

/// GRIB2ファイルから第3節:格子系定義節（テンプレート3.0）を読み込む。
///
/// # 引数
///
/// * `reader` - ファイルリーダー
/// * `section_bytes` - 節の長さ
/// * `section_number` - 節番号
/// * `source_of_grid_definition` - 格子系定義の出典
/// * `number_of_points` - 格子点数
/// * `number_of_octets_for_number_of_points` - 格子点数を定義するリストのオクテット数
/// * `description_of_number_of_points` - 格子点数を定義するリストの説明
/// * `grid_definition_template_number` - 格子系定義テンプレート番号
///
/// # 戻り値
///
/// * `Section3::Template3_0`
#[allow(clippy::too_many_arguments)]
fn read_section3_0<R: Read>(
    reader: &mut BufReader<R>,
    section_bytes: usize,
    section_number: u8,
    source_of_grid_definition: u8,
    number_of_points: u32,
    number_of_octets_for_number_of_points: u8,
    description_of_number_of_points: u8,
    grid_definition_template_number: u16,
) -> Grib2Result<Section3> {
    // 地球の形状: 1バイト
    let shape_of_earth = read_u8(reader, "第3節:地球の形状")?;
    // 地球球体の半径の尺度因子: 1バイト
    let scale_factor_of_radius_of_spherical_earth =
        read_u8(reader, "第3節:地球球体の半径の尺度因子")?;
    // 地球球体の尺度付き半径: 4バイト
    let scaled_value_of_radius_of_spherical_earth =
        read_u32(reader, "第3節:地球球体の尺度付き半径")?;
    // 地球回転楕円体の長軸の尺度因子: 1バイト
    let scale_factor_of_earth_major_axis = read_u8(reader, "第3節:地球回転楕円体の長軸の尺度因子")?;
    // 地球回転楕円体の長軸の尺度付きの長さ: 4バイト
    let scaled_value_of_earth_major_axis =
        read_u32(reader, "第3節:地球回転楕円体の長軸の尺度付きの長さ")?;
    // 地球回転楕円体の短軸の尺度因子: 1バイト
    let scale_factor_of_earth_minor_axis = read_u8(reader, "第3節:地球回転楕円体の短軸の尺度因子")?;
    // 地球回転楕円体の短軸の尺度付きの長さ: 4バイト
    let scaled_value_of_earth_minor_axis =
        read_u32(reader, "第3節:地球回転楕円体の短軸の尺度付きの長さ")?;
    // 緯線に沿った格子点数: 4バイト
    let number_of_along_lat_points = read_u32(reader, "第3節:緯線に沿った格子点数")?;
    // 経線に沿った格子点数: 4バイト
    let number_of_along_lon_points = read_u32(reader, "第3節:経線に沿った格子点数")?;
    // 原作成領域の基本角: 4バイト
    let basic_angle_of_initial_product_domain = read_u32(reader, "第3節:原作成領域の基本角")?;
    // 端点の経度及び緯度並びに方向増分の定義に使われる基本角の細分: 4バイト
    let subdivisions_of_basic_angle =
        read_u32(reader, "第3節:端点の経度及び緯度並びに方向増分の定義")?;
    // 最初の格子点の緯度（1e-6度単位）: 4バイト
    let lat_of_first_grid_point = read_u32(reader, "第3節:最初の格子点の緯度")?;
    // 最初の格子点の経度（1e-6度単位）: 4バイト
    let lon_of_first_grid_point = read_u32(reader, "第3節:最初の格子点の経度")?;
    // 分解能及び成分フラグ: 1バイト
    let resolution_and_component_flags = read_u8(reader, "第3節:分解能及び成分フラグ")?;
    // 最後の格子点の緯度（1e-6度単位）: 4バイト
    let lat_of_last_grid_point = read_u32(reader, "第3節:最後の格子点の緯度")?;
    // 最後の格子点の経度（1e-6度単位）: 4バイト
    let lon_of_last_grid_point = read_u32(reader, "第3節:最後の格子点の経度")?;
    // i方向（経度方向）の増分（1e-6度単位）: 4バイト
    let i_direction_increment = read_u32(reader, "第3節:i方向の増分")?;
    // j方向（緯度方向）の増分（1e-6度単位）: 4バイト
    let j_direction_increment = read_u32(reader, "第3節:j方向の増分")?;
    // 走査モード: 1バイト
    let scanning_mode = read_u8(reader, "第3節:走査モード")?;

    Ok(Section3::Template3_0(Section3_0 {
        section_bytes,
        section_number,
        source_of_grid_definition,
        number_of_points,
        number_of_octets_for_number_of_points,
        description_of_number_of_points,
        grid_definition_template_number,
        shape_of_earth,
        scale_factor_of_radius_of_spherical_earth,
        scaled_value_of_radius_of_spherical_earth,
        scale_factor_of_earth_major_axis,
        scaled_value_of_earth_major_axis,
        scale_factor_of_earth_minor_axis,
        scaled_value_of_earth_minor_axis,
        number_of_along_lat_points,
        number_of_along_lon_points,
        basic_angle_of_initial_product_domain,
        subdivisions_of_basic_angle,
        lat_of_first_grid_point,
        lon_of_first_grid_point,
        resolution_and_component_flags,
        lat_of_last_grid_point,
        lon_of_last_grid_point,
        i_direction_increment,
        j_direction_increment,
        scanning_mode,
    }))
}
