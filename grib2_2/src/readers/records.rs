use std::io::{BufReader, Read};

use num_format::{Locale, ToFormattedString};

use crate::{Grib2Error, Grib2Result};

/// GRIB2が第7節に記録しているレコード
#[derive(Debug, Clone, Copy)]
pub struct Grib2Record<T>
where
    T: Clone + Copy,
{
    /// 10e-6度単位の緯度
    pub lat: u32,
    /// 10e-6度単位の経度
    pub lon: u32,
    /// レベル値
    pub level: u16,
    /// 値
    pub value: Option<T>,
}

pub struct Grib2RecordIter<'a, R, V>
where
    R: Read,
{
    /// ファイルリーダー
    reader: &'a mut BufReader<R>,
    /// GRIB2ファイルに記録されている座標数
    number_of_points: u32,
    /// ランレングス圧縮符号を記録しているバイト数
    total_bytes: usize,
    /// 経度の最小値（10e-6度単位）
    lon_min: u32,
    /// 経度の最大値（10e-6度単位）
    lon_max: u32,
    /// 緯度の増分（10e-6度単位）
    lat_inc: u32,
    /// 経度の増分（10e-6度単位）
    lon_inc: u32,
    /// 今回のレベルの最大値
    maxv: u16,
    /// LNGU進数
    lngu: u16,
    /// レベル別物理値
    level_values: &'a [V],
    /// ランレングス圧縮符号を読み込んだバイト数
    read_bytes: usize,
    /// 現在の緯度（10e-6度単位）
    current_lat: u32,
    /// 現在の経度（10e-6度単位）
    current_lon: u32,
    /// 現在のレベル値
    current_level: u16,
    /// 現在の物理値
    current_value: Option<V>,
    /// 現在値を返却する回数
    returning_times: u32,
    /// 読み込んだ座標数
    number_of_reads: u32,
    /// 最後に読み込んだランレングス圧縮符号
    last_run_length: Option<u16>,
}

impl<'a, R, V> Grib2RecordIter<'a, R, V>
where
    R: Read,
{
    /// GRIB2ファイルの現在のファイルポインターの位置から`u8`型の値を読み込む。
    ///
    /// # 戻り値
    ///
    /// * GRIB2ファイルの現在のファイルポインターの位置から読み込んだ`u8`型の値
    fn read_u8(&mut self) -> Grib2Result<u8> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf).map_err(|_| {
            Grib2Error::ReadError("ランレングス圧縮オクテットの読み込みに失敗しました。".into())
        })?;
        self.read_bytes += 1;

        Ok(buf[0])
    }

    /// GRIB2ファイルの現在のファイルポインターの位置からランレングス符号を読み込む。
    ///
    /// # 戻り値
    ///
    /// * ランレングス符号を記録したベクター
    fn retrieve_run_length(&mut self) -> Grib2Result<Vec<u16>> {
        let mut run_length: Vec<u16> = vec![];
        if self.last_run_length.is_some() {
            run_length.push(self.last_run_length.unwrap());
        }
        while self.read_bytes < self.total_bytes {
            let value = self.read_u8()? as u16;
            if value <= self.maxv && !run_length.is_empty() {
                self.last_run_length = Some(value);
                break;
            } else {
                run_length.push(value);
            }
        }

        Ok(run_length)
    }
}

impl<'a, R, V> Iterator for Grib2RecordIter<'a, R, V>
where
    R: Read,
    V: Copy,
{
    type Item = Grib2Result<Grib2Record<V>>;

    fn next(&mut self) -> Option<Self::Item> {
        // 現在値返却回数が0かつ、読み込んだバイト数がランレングス圧縮符号列を記録しているバイト数に達している場合は終了
        if self.returning_times == 0 && self.total_bytes <= self.read_bytes {
            if self.number_of_reads == self.number_of_points {
                return None;
            } else {
                return Some(Err(Grib2Error::Unexpected(
                    format!(
                        "読み込んだ座標数({})が第3節に記録されている資料点数({})と一致しません。\
                        ファイルが壊れている、またはクレートにバグがある可能性があります。",
                        self.number_of_reads.to_formatted_string(&Locale::ja),
                        self.number_of_points.to_formatted_string(&Locale::ja),
                    )
                    .into(),
                )));
            }
        }

        // 現在値返却回数が0の場合は、ランレングス圧縮符号を展開して現在値を更新
        if self.returning_times == 0 {
            // ランレングス圧縮符号を取得
            let run_length = self.retrieve_run_length();
            if run_length.is_err() {
                return Some(Err(run_length.err().unwrap()));
            }
            // ランレングス圧縮符号を展開
            let (level, times) = expand_run_length(&run_length.unwrap(), self.maxv, self.lngu);
            // 現在のレベル値、物理値及び返却回数を更新
            self.current_level = level;
            self.current_value = if 0 < level {
                Some(self.level_values[level as usize - 1])
            } else {
                None
            };
            self.returning_times = times;
        }

        // 結果を生成
        let result = Some(Ok(Grib2Record {
            lat: self.current_lat,
            lon: self.current_lon,
            level: self.current_level,
            value: self.current_value,
        }));
        // 現在値を返す回数を減らす
        self.returning_times -= 1;
        // 格子を移動
        self.current_lon += self.lon_inc;
        if self.lon_max < self.current_lon {
            self.current_lat -= self.lat_inc;
            self.current_lon = self.lon_min;
        }
        // 読み込んだ座標数をインクリメント
        self.number_of_reads += 1;

        result
    }
}

#[derive(Default)]
pub struct Grib2RecordIterBuilder<'a, R, V>
where
    R: Read,
    V: Clone + Copy,
{
    reader: Option<&'a mut BufReader<R>>,
    total_bytes: Option<usize>,
    number_of_points: Option<u32>,
    lat_max: Option<u32>,
    lon_min: Option<u32>,
    lon_max: Option<u32>,
    lat_inc: Option<u32>,
    lon_inc: Option<u32>,
    nbit: Option<u16>,
    maxv: Option<u16>,
    level_values: Option<&'a [V]>,
}

impl<'a, R, V> Grib2RecordIterBuilder<'a, R, V>
where
    R: Read,
    V: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            reader: None,
            total_bytes: None,
            number_of_points: None,
            lat_max: None,
            lon_min: None,
            lon_max: None,
            lat_inc: None,
            lon_inc: None,
            nbit: None,
            maxv: None,
            level_values: None,
        }
    }

    /// リーダーを設定する。
    pub fn reader(mut self, reader: &'a mut BufReader<R>) -> Self {
        self.reader = Some(reader);
        self
    }

    /// ランレングス圧縮符号全体のバイト数を設定する。
    pub fn total_bytes(mut self, total_bytes: usize) -> Self {
        self.total_bytes = Some(total_bytes);
        self
    }

    /// GRIB2ファイルに記録されている座標数を設定する。
    pub fn number_of_points(mut self, number_of_points: u32) -> Self {
        self.number_of_points = Some(number_of_points);
        self
    }

    /// 緯度の最大値（10e-6度単位）を設定する。
    pub fn lat_max(mut self, lat_max: u32) -> Self {
        self.lat_max = Some(lat_max);
        self
    }

    /// 経度の最小値（10e-6度単位）を設定する。
    pub fn lon_min(mut self, lon_min: u32) -> Self {
        self.lon_min = Some(lon_min);
        self
    }

    /// 経度の最大値（10e-6度単位）を設定する。
    pub fn lon_max(mut self, lon_max: u32) -> Self {
        self.lon_max = Some(lon_max);
        self
    }

    /// 緯度の増分（10e-6度単位）を設定する。
    pub fn lat_inc(mut self, lat_inc: u32) -> Self {
        self.lat_inc = Some(lat_inc);
        self
    }

    /// 経度の増分（10e-6度単位）を設定する。
    pub fn lon_inc(mut self, lon_inc: u32) -> Self {
        self.lon_inc = Some(lon_inc);
        self
    }

    /// 1格子点値当りのビット数を設定する。
    pub fn nbit(mut self, nbit: u16) -> Self {
        self.nbit = Some(nbit);
        self
    }

    /// 今回の圧縮に用いたレベルの最大値を設定する。
    pub fn maxv(mut self, maxv: u16) -> Self {
        self.maxv = Some(maxv);
        self
    }

    /// レベル別物理値を設定する。
    pub fn level_values(mut self, level_values: &'a [V]) -> Self {
        self.level_values = Some(level_values);
        self
    }

    pub fn build(self) -> Grib2Result<Grib2RecordIter<'a, R, V>> {
        let reader = self
            .reader
            .ok_or_else(|| Grib2Error::RuntimeError("リーダーが設定されていません。".into()))?;
        let total_bytes = self.total_bytes.ok_or_else(|| {
            Grib2Error::RuntimeError(
                "ランレングス圧縮符号全体のバイト数が設定されていません。".into(),
            )
        })?;
        let number_of_points = self.number_of_points.ok_or_else(|| {
            Grib2Error::RuntimeError(
                "GRIB2ファイルに記録されている座標数が設定されていません。".into(),
            )
        })?;
        let lat_max = self
            .lat_max
            .ok_or_else(|| Grib2Error::RuntimeError("緯度の最大値が設定されていません。".into()))?;
        let lon_min = self
            .lon_min
            .ok_or_else(|| Grib2Error::RuntimeError("経度の最小値が設定されていません。".into()))?;
        let lon_max = self
            .lon_max
            .ok_or_else(|| Grib2Error::RuntimeError("経度の最大値が設定されていません。".into()))?;
        let lat_inc = self
            .lat_inc
            .ok_or_else(|| Grib2Error::RuntimeError("緯度の増分が設定されていません。".into()))?;
        let lon_inc = self
            .lon_inc
            .ok_or_else(|| Grib2Error::RuntimeError("経度の増分が設定されていません。".into()))?;
        let nbit = self.nbit.ok_or_else(|| {
            Grib2Error::RuntimeError("1格子点値当りのビット数が設定されていません。".into())
        })?;
        let maxv = self.maxv.ok_or_else(|| {
            Grib2Error::RuntimeError(
                "今回の圧縮に用いたレベルの最大値が設定されていません。".into(),
            )
        })?;
        let level_values = self.level_values.ok_or_else(|| {
            Grib2Error::RuntimeError("レベル別物理値が設定されていません。".into())
        })?;

        Ok(Grib2RecordIter {
            reader,
            total_bytes,
            number_of_points,
            lon_min,
            lon_max,
            lat_inc,
            lon_inc,
            maxv,
            lngu: 2u16.pow(nbit as u32) - 1 - maxv,
            level_values,
            read_bytes: 0,
            current_lat: lat_max,
            current_lon: lon_min,
            current_level: 0,
            current_value: None,
            returning_times: 0,
            number_of_reads: 0,
            last_run_length: None,
        })
    }
}

/// 1セットのランレングス圧縮符号を展開する。
///
/// 引数valuesの最初の要素はレベル値で、それ以降はランレングス値である。
/// これを1セットのランレングス圧縮符号とする。
/// ランレングス値を含まない場合のvaluesの要素数は1で、ランレングス値を含む場合のvaluesの要素数
/// は2以上である。
///
/// この関数が展開する、GRIB2資料テンプレート7.200（気象庁定義資料テンプレート）で利用されている
/// ランレングス圧縮符号を以下に示す。
///
/// * 格子点値が取りうるレベル値
///   * レベル値は2次元矩形領域の格子点上に存在し、0以上maxv以下の整数を取る。
///   * ここでmaxvは、GRIB資料表現テンプレート5.200（気象庁定義資料表現テンプレート）
///     第5節13-14オクテットで示される「今回の圧縮に用いたレベルの最大値」である。
///     * 第5節15-16オクテットの「レベルの最大値」ではないことに注意すること。
/// * 2次元データの1次元化
///   * 主走査方向を2次元矩形領域の左から右（通常西から東）、副走査方向を上から下（通常北から南）と
///     して、2次元データを1次元化する。
///     * データは最も左上の格子点の値から始まり、東方向に向かって格子点のレベル値を記録する。
///     * その緯度の最東端に達したら、下の最西端の格子点に移動して、上記同様に格子点のレベル値を記録
///       する。
///   * 最初のデータは最も左上の格子点の値であり、最後のデータは最も右下の格子点の値である。
/// * ランレングス符号化後の1格子点値当りのビット数（nbit）
///   * nbitは、ランレングス符号化されたデータ列の中で、レベル値及びランレングス値を表現するビット数
///     である。
///   * nbitは、GRIB2資料表現テンプレート5.200第5節12オクテットで示される「1データのビット数」
///     である。
/// * 1セット内のレベル値とランレングス値の配置
///   * ランレングス符号化されたデータ列の中で0以上maxv以下の値は各格子点のレベル値で、maxvよりも
///     大きな値はランレングス値である。
///   * 1セットは、最初にレベル値を配置し、もしその値が連続するのであれば後ろにランレングス値を付加
///     して作成される。
///   * maxvよりも大きな値が続く場合、それらすべては当該セットのランレングス値である。
///   * データに、maxv以下の値が現れた時点で当該セットが終了し、このmaxv以下の値は次のセットの
///     レベル値となる。
///   * なお、同じレベル値が連続しない場合はランレングスは付加されず、次のセットに移る。
/// * ランレングス符号化方法
///   * (2 ^ nbit - maxv)よりも大きなランレングスが必要となった場合、1データでは表現すること
///     ができない。
///   * これに対応するために、2つ以上のランレングス値を連続させてランレングスを表現するが、連続した
///      データの単純な総和をランレングスとしても圧縮効率があがらない。
///   * よって、lngu(=2 ^ nbit - 1 - maxv)進数を用いてランレングスを表現する。
///   * レベル値のすぐ後に続く最初のランレングス値(data1)をlngu進数の1桁目
///     RL1={lngu ^ (1 - 1) * (data1 - (maxv + 1))}とする。
///   * それ以降n番目のランレングス値(dataN)は、lngu進数のn桁目
///     RLn={lngu ^ (n - 1) * (dataN - (maxv + 1))}とする。
///   * 最終的なランレングスは、それらの「総和 + 1(RL = ΣRLi + 1)」となる。
/// * ランレングス符号化例
///   * nbit = 4、maxv = 10とした場合、lngu = 2 ^ 4 - 1 - 10 = 16 - 1 - 10 = 5となる。
///   * ランレングス符号化列 = {3, 9, 12, 6, 4, 15, 2, 1, 0, 13, 12, 2, 3}は、以下の通り
///     展開される。
///   * {3, 9, 9, 6, 4, 4, 4, 4, 4, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3}
///   * 最初の3
///     * 最初の3はレベル値である。
///     * 3の次の9はmaxv以下であるため、9はレベル値である。
///     * よって、最初の3は1つだけで、繰り返されない。
///   * レベル値とランレングス値のセット{9, 12}
///     * 9がレベル値で12がランレングス値である。
///     * 12の次は6であり、10以下であるため6はレベル値である。
///     * RL1 = 5 ^ (1 - 1) * (12 - (10 + 1)) = 1 * 1 = 1
///     * RL = 1 + 1 = 2
///     * よって、9が２つ連続する。
///   * レベル値とランレングス値のセット{0, 13, 12}
///     * 0がレベル値で13と12がランレングス値である。
///     * RL1 = 5 ^ (1 - 1) * (13 - (10 + 1)) = 1 * 2 = 2
///     * RL2 = 5 ^ (2 - 1) * (12 - (10 + 1)) = 5 * 1 = 5
///     * RL = 2 + 5 + 1 = 8
///     * よって、0が8連続する。
///
/// # 引数
///
/// * `values` - 1セットのランレングス圧縮データ。
/// * `maxv` - 今回の圧縮に用いたレベルの最大値（第5節 13-14オクテット）。
/// * `lngu` - レベル値またはランレングス値のビット数をnbitとしたときの、2 ^ nbit -1 - maxvの値。
///
/// # 戻り値
///
/// レベル値とそのレベル値を繰り返す数を格納したタプル。
fn expand_run_length(values: &[u16], maxv: u16, lngu: u16) -> (u16, u32) {
    assert!(values[0] <= maxv, "values[0]={}, maxv={}", values[0], maxv);

    // ランレングス圧縮されていない場合
    if values.len() == 1 {
        return (values[0], 1);
    }

    // ランレングス圧縮を展開
    let values: Vec<u32> = values.iter().map(|v| *v as u32).collect();
    let lngu = lngu as u32;
    let maxv = maxv as u32;
    let times: u32 = values[1..]
        .iter()
        .enumerate()
        .map(|(i, &v)| lngu.pow(i as u32) * (v - (maxv + 1)))
        .sum();

    (values[0] as u16, times + 1)
}

#[cfg(test)]
mod tests {
    use super::expand_run_length;

    #[test]
    fn expand_run_length0_ok() {
        let nbit = 4;
        let maxv = 10;
        let lngu = 2u16.pow(nbit) - 1 - maxv;
        let values = vec![3u16];
        let expected = (3u16, 1u32);
        assert_eq!(expected, expand_run_length(&values, maxv, lngu));
    }

    #[test]
    fn expand_run_length1_ok() {
        let nbit = 4;
        let maxv = 10;
        let lngu = 2u16.pow(nbit) - 1 - maxv;
        let values = vec![9u16, 12];
        let expected = (9u16, 2u32);
        assert_eq!(expected, expand_run_length(&values, maxv, lngu));
    }

    #[test]
    fn expand_run_length2_ok() {
        let nbit = 4;
        let maxv = 10;
        let lngu = 2u16.pow(nbit) - 1 - maxv;
        let values = vec![4u16, 15];
        let expected = (4u16, 5u32);
        assert_eq!(expected, expand_run_length(&values, maxv, lngu));
    }

    #[test]
    fn expand_run_length3() {
        let nbit = 4;
        let maxv = 10;
        let lngu = 2u16.pow(nbit) - 1 - maxv;
        let values = vec![0u16, 13, 12];
        let expected = (0u16, 8u32);
        assert_eq!(expected, expand_run_length(&values, maxv, lngu));
    }
}
