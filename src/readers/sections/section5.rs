/// 第5節: 資料表現節
#[derive(Debug, Clone)]
pub struct Section5<T> {
    /// 節の長さ（バイト数）
    section_bytes: usize,
    /// 全資料点の数
    number_of_values: u32,
    /// 資料表現テンプレート番号
    data_representation_template_number: u16,
    /// 1データのビット数
    bits_per_value: u8,
    /// テンプレート5
    template5: T,
}

impl<T> Section5<T> {
    /// 節の長さ（バイト数）を返す。
    pub fn section_bytes(&self) -> usize {
        self.section_bytes
    }

    /// 全資料点の数を返す。
    pub fn number_of_values(&self) -> u32 {
        self.number_of_values
    }

    /// 資料表現テンプレート番号を返す。
    pub fn data_representation_template_number(&self) -> u16 {
        self.data_representation_template_number
    }

    /// 1データのビット数を返す。
    pub fn bits_per_value(&self) -> u8 {
        self.bits_per_value
    }
}

/// テンプレート5.200
#[derive(Debug, Clone)]
pub struct Template5_200i16 {
    /// 今回の圧縮に用いたレベルの最大値
    max_level_value: u16,
    /// データの取り得るレベルの最大値
    number_of_level_values: u16,
    /// データ代表値の尺度因子
    decimal_scale_factor: u8,
    /// レベル値と物理値(mm/h)の対応を格納するコレクション
    level_values: Vec<i16>,
}

pub type Section5_200i16 = Section5<Template5_200i16>;

impl Section5_200i16 {
    /// 今回の圧縮に用いたレベルの最大値を返す。
    pub fn max_level_value(&self) -> u16 {
        self.template5.max_level_value
    }

    /// データの取り得るレベルの最大値を返す。
    pub fn number_of_level_values(&self) -> u16 {
        self.template5.number_of_level_values
    }

    /// データ代表値の尺度因子を返す。
    pub fn decimal_scale_factor(&self) -> u8 {
        self.template5.decimal_scale_factor
    }

    /// レベルmに対応するデータ代表値を返す。
    pub fn level_values(&self) -> &[i16] {
        &self.template5.level_values
    }
}
