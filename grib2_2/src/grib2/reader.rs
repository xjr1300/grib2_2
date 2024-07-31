use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek};
use std::path::Path;

use num_format::{Locale, ToFormattedString as _};

use crate::readers::records::expand_run_length;
use crate::{Grib2Error, Grib2Result};

use super::sections::{
    Section0, Section1, Section2, Section3, Section4, Section5, Section6, Section7, Section8,
};

/// GRIB2ファイルリーダー
pub struct Grib2Reader {
    /// ファイルリーダー
    reader: BufReader<File>,
    /// 第0節:指示節
    pub section0: Section0,
    /// 第1節:識別節
    pub section1: Section1,
    /// 第2節:地域使用節
    pub section2: Section2,
    /// 第3節:格子系定義節
    pub section3: Section3,
    /// 第4節:プロダクト定義節
    pub section4: Section4,
    /// 第5節:資料表現節
    pub section5: Section5,
    /// 第6節:ビットマップ節
    pub section6: Section6,
    /// 第7節:資料節
    pub section7: Section7,
    /// 第8節: 終端節
    pub section8: Section8,
}

impl Grib2Reader {
    /// GRIB2ファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 開くGRIB2ファイルのパス。
    ///
    /// # GRIB2リーダー
    pub fn new<P: AsRef<Path>>(path: P) -> Grib2Result<Self> {
        let path = path.as_ref();
        if !path.is_file() {
            return Err(Grib2Error::FileDoesNotExist);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;
        let mut reader = BufReader::new(file);
        let section0 = Section0::from_reader(&mut reader)?;
        let section1 = Section1::from_reader(&mut reader)?;
        let section2 = Section2::from_reader(&mut reader)?;
        let section3 = Section3::from_reader(&mut reader)?;
        let section4 = Section4::from_reader(&mut reader)?;
        let section5 = Section5::from_reader(&mut reader)?;
        let section6 = Section6::from_reader(&mut reader)?;
        let section7 = Section7::from_reader(&mut reader)?;
        let section8 = Section8::from_reader(&mut reader)?;

        Ok(Self {
            reader,
            section0,
            section1,
            section2,
            section3,
            section4,
            section5,
            section6,
            section7,
            section8,
        })
    }

    /// GRIB2の第7節に記録されているレコードを反復処理するイテレーターを返す。
    ///
    /// # 戻り値
    ///
    /// * GRIB2のレコードを反復処理するイテレーター
    pub fn record_iter(&mut self) -> Grib2Result<Grib2RecordIter<'_, File>> {
        Grib2RecordIterBuilder::new()
            .reader(&mut self.reader)
            .run_length_position(self.section7.run_length_position()?)
            .run_length_bytes(self.section7.run_length_bytes()?)
            .number_of_points(self.section3.number_of_points()?)
            .lat_max(self.section3.lat_of_first_grid_point()?)
            .lon_min(self.section3.lon_of_first_grid_point()?)
            .lon_max(self.section3.lon_of_last_grid_point()?)
            .lat_inc(self.section3.j_direction_increment()?)
            .lon_inc(self.section3.i_direction_increment()?)
            .nbit(self.section5.bit_per_value()? as u16)
            .maxv(self.section5.max_level_value()?)
            .level_values(self.section5.level_values()?)
            .build()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Grib2Record {
    /// 1e-6度単位の緯度
    pub lat: u32,
    /// 1e-6度単位の経度
    pub lon: u32,
    /// 値を表現するバイト列
    pub value: Option<[u8; 2]>,
}

pub struct Grib2RecordIter<'a, R>
where
    R: Read,
{
    /// ファイルリーダー
    reader: &'a mut BufReader<R>,
    /// GRIB2ファイルに記録されている座標数
    number_of_points: u32,
    /// ランレングス圧縮符号を記録しているバイト数
    total_bytes: usize,
    /// 経度の最小値（1e-6度単位）
    lon_min: u32,
    /// 経度の最大値（1e-6度単位）
    lon_max: u32,
    /// 緯度の増分（1e-6度単位）
    lat_inc: u32,
    /// 経度の増分（1e-6度単位）
    lon_inc: u32,
    /// 今回のレベルの最大値
    maxv: u16,
    /// LNGU進数
    lngu: u16,
    /// レベル別物理値
    level_values: &'a [[u8; 2]],
    /// ランレングス圧縮符号を読み込んだバイト数
    read_bytes: usize,
    /// 現在の緯度（1e-6度単位）
    current_lat: u32,
    /// 現在の経度（1e-6度単位）
    current_lon: u32,
    /// 現在のレベル値
    current_level: u16,
    /// 現在の物理値
    current_value: Option<[u8; 2]>,
    /// 現在値を返却する回数
    returning_times: u32,
    /// 読み込んだ座標数
    number_of_reads: u32,
    /// 最後に読み込んだランレングス圧縮符号
    last_run_length: Option<u16>,
}

impl<'a, R> Grib2RecordIter<'a, R>
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

impl<'a, R> Iterator for Grib2RecordIter<'a, R>
where
    R: Read,
{
    type Item = Grib2Result<Grib2Record>;

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

struct Grib2RecordIterBuilder<'a, R>
where
    R: Read + Seek,
{
    reader: Option<&'a mut BufReader<R>>,
    run_length_position: Option<usize>,
    run_length_bytes: Option<usize>,
    number_of_points: Option<u32>,
    lat_max: Option<u32>,
    lon_min: Option<u32>,
    lon_max: Option<u32>,
    lat_inc: Option<u32>,
    lon_inc: Option<u32>,
    nbit: Option<u16>,
    maxv: Option<u16>,
    level_values: Option<&'a [[u8; 2]]>,
}

impl<'a, R> Grib2RecordIterBuilder<'a, R>
where
    R: Read + Seek,
{
    pub fn new() -> Self {
        Self {
            reader: None,
            run_length_position: None,
            run_length_bytes: None,
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

    /// ランレングス圧縮符号列の開始位置を設定する。
    pub fn run_length_position(mut self, run_length_position: usize) -> Self {
        self.run_length_position = Some(run_length_position);
        self
    }
    /// ランレングス圧縮符号全体のバイト数を設定する。
    pub fn run_length_bytes(mut self, run_length_bytes: usize) -> Self {
        self.run_length_bytes = Some(run_length_bytes);
        self
    }

    /// GRIB2ファイルに記録されている座標数を設定する。
    pub fn number_of_points(mut self, number_of_points: u32) -> Self {
        self.number_of_points = Some(number_of_points);
        self
    }

    /// 緯度の最大値（1e-6度単位）を設定する。
    pub fn lat_max(mut self, lat_max: u32) -> Self {
        self.lat_max = Some(lat_max);
        self
    }

    /// 経度の最小値（1e-6度単位）を設定する。
    pub fn lon_min(mut self, lon_min: u32) -> Self {
        self.lon_min = Some(lon_min);
        self
    }

    /// 経度の最大値（1e-6度単位）を設定する。
    pub fn lon_max(mut self, lon_max: u32) -> Self {
        self.lon_max = Some(lon_max);
        self
    }

    /// 緯度の増分（1e-6度単位）を設定する。
    pub fn lat_inc(mut self, lat_inc: u32) -> Self {
        self.lat_inc = Some(lat_inc);
        self
    }

    /// 経度の増分（1e-6度単位）を設定する。
    pub fn lon_inc(mut self, lon_inc: u32) -> Self {
        self.lon_inc = Some(lon_inc);
        self
    }

    /// 1データのビット数を設定する。
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
    pub fn level_values(mut self, level_values: &'a [[u8; 2]]) -> Self {
        self.level_values = Some(level_values);
        self
    }

    pub fn build(self) -> Grib2Result<Grib2RecordIter<'a, R>> {
        let reader = self
            .reader
            .ok_or_else(|| Grib2Error::RuntimeError("リーダーが設定されていません。".into()))?;
        let run_length_position = self.run_length_position.ok_or_else(|| {
            Grib2Error::RuntimeError(
                "ランレングス圧縮符号列の開始位置が設定されていません。".into(),
            )
        })?;
        let run_length_bytes = self.run_length_bytes.ok_or_else(|| {
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

        // ランレングス圧縮符号列の開始位置にファイルポインターを移動
        reader
            .seek(std::io::SeekFrom::Start(run_length_position as u64))
            .map_err(|e| {
                Grib2Error::ReadError(
                    format!(
                        "ランレングス圧縮符号列の開始位置にファイルポインターを移動できません。{}",
                        e
                    )
                    .into(),
                )
            })?;

        Ok(Grib2RecordIter {
            reader,
            total_bytes: run_length_bytes,
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
