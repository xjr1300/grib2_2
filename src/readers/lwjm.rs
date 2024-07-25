use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::readers::records::Grib2RecordIter;
use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_50000, Section5_200i16, Section6,
    Section7_200, Section8,
};
use crate::{Grib2Error, Grib2Result};

use super::records::Grib2RecordIterBuilder;

/// 土砂災害警戒判定メッシュファイルリーダー
///
/// 次の土砂災害警戒判定メッシュファイルを読み込む。
///
/// * 実況の土砂災害警戒判定
/// * 実況と1時間から3時間予想までの土砂災害警戒判定
pub struct LwjmReader {
    /// ファイルパス
    pub path: PathBuf,
    /// 土砂災害警戒判定メッシュファイルが、1時間から3時間までの判定を含んでいるかを示すフラグ
    pub has_forecast: bool,
    /// 第0節:指示節
    pub section0: Section0,
    /// 第1節:識別節
    pub section1: Section1,
    /// 第2節:地域使用節
    pub section2: Section2,
    /// 第3節:格子系定義節
    pub section3: Section3_0,
    /// 第4節:プロダクト定義節から第7節:資料節を格納したベクター
    pub judgments: Vec<LwjmJudgment>,
    /// 第8節:終端節
    pub section8: Section8,
}

/// 第4節:プロダクト定義節から第7節:資料節
pub struct LwjmJudgment {
    /// 第4節:プロダクト定義節
    pub section4: Section4_50000,
    /// 第5節:資料表現節
    pub section5: Section5_200i16,
    /// 第6節:ビットマップ節
    pub section6: Section6,
    /// 第7節:資料節
    pub section7: Section7_200,
}

impl LwjmReader {
    /// 土砂災害警戒判定メッシュファイルを開く。
    ///
    /// # 引数
    ///
    /// * `path` - 土砂災害警戒判定メッシュファイルのパス
    /// * `has_forecast` - 土砂災害警戒判定メッシュファイルが実況のみを記録している場合は`false`、
    ///                    実況と1時間から3時間までの予想を記録している場合は`true`
    ///
    /// # 戻り値
    ///
    /// * 土砂災害警戒判定メッシュリーダー
    pub fn new<P: AsRef<Path>>(path: P, has_forecast: bool) -> Grib2Result<Self> {
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
        let section2 = Section2;
        let section3 = Section3_0::from_reader(&mut reader)?;
        let judgments = match has_forecast {
            false => vec![LwjmJudgment::from_reader(&mut reader)?],
            true => vec![
                LwjmJudgment::from_reader(&mut reader)?,
                LwjmJudgment::from_reader(&mut reader)?,
                LwjmJudgment::from_reader(&mut reader)?,
                LwjmJudgment::from_reader(&mut reader)?,
            ],
        };
        let section8 = Section8::from_reader(&mut reader)?;

        Ok(Self {
            path: path.to_path_buf(),
            has_forecast,
            section0,
            section1,
            section2,
            section3,
            judgments,
            section8,
        })
    }

    /// 開いている土砂災害警戒判定メッシュファイルのパスを返す。
    ///
    /// # 戻り値
    ///
    /// * 開いている土砂災害警戒判定メッシュファイルのパス
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// 第0節:指示節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第0節:指示節
    pub fn section0(&self) -> &Section0 {
        &self.section0
    }

    /// 第1節:識別節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第1節:識別節
    pub fn section1(&self) -> &Section1 {
        &self.section1
    }

    /// 第2節:地域使用節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第2節:地域使用節
    pub fn section2(&self) -> &Section2 {
        &self.section2
    }

    /// 第3節:格子系定義節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第3節:格子系定義節
    pub fn section3(&self) -> &Section3_0 {
        &self.section3
    }

    /// 指定された土砂災害警戒判定時間別の第4節:プロダクト定義節から第7節:資料節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    pub fn judgment(&self, hour: LwjmHour) -> Grib2Result<&LwjmJudgment> {
        // 指定された土砂災害警戒判定時間の判定を取得
        // 実況以外、つまり1時間から3時間までの予測のいずれかで、土砂災害警戒判定メッシュファイルが
        // 予測を記録していない場合はエラー
        if hour != LwjmHour::Live && !self.has_forecast {
            return Err(Grib2Error::RuntimeError(
                "土砂災害警戒判定メッシュファイルは予測を記録していません。".into(),
            ));
        }
        Ok(&self.judgments[(hour as u8) as usize])
    }

    /// 第8節:終端節を返す。
    ///
    /// # 戻り値
    ///
    /// * 第8節:終端節
    pub fn section8(&self) -> &Section8 {
        &self.section8
    }

    /// 指定された土砂災害警戒判定時間のレコードを反復処理するイテレーターを返す。
    ///
    /// # 引数
    ///
    /// * `hour` - レコードを取得する土砂災害警戒判定時間
    ///
    /// # 戻り値
    ///
    /// * 指定された土砂災害警戒判定時間のレコードを反復処理するイテレーター
    pub fn record_iter(&mut self, hour: LwjmHour) -> Grib2Result<Grib2RecordIter<'_, File, i16>> {
        // 指定された土砂災害警戒判定時間の判定を取得
        // 実況以外、つまり1時間から3時間までの予測のいずれかで、土砂災害警戒判定メッシュファイルが
        // 予測を記録していない場合はエラー
        if hour != LwjmHour::Live && !self.has_forecast {
            return Err(Grib2Error::RuntimeError(
                "土砂災害警戒判定メッシュファイルは予測を記録していません。".into(),
            ));
        }
        let judgment = &self.judgments[hour as u8 as usize];

        // 土砂災害警戒判定メッシュファイルを開く
        if !self.path.is_file() {
            return Err(Grib2Error::FileDoesNotExist);
        }
        let file = OpenOptions::new()
            .read(true)
            .open(&self.path)
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;
        let mut reader = BufReader::new(file);

        // ランレングス符号の開始位置にファイルポインターを移動
        reader
            .seek(SeekFrom::Start(
                judgment.section7.run_length_position() as u64
            ))
            .map_err(|e| Grib2Error::Unexpected(e.into()))?;

        // イテレーターを構築
        Grib2RecordIterBuilder::new()
            .reader(reader)
            .total_bytes(judgment.section7.run_length_bytes())
            .number_of_points(self.section3.number_of_data_points())
            .lat_max(self.section3.lat_of_first_grid_point())
            .lon_min(self.section3.lon_of_first_grid_point())
            .lon_max(self.section3.lon_of_last_grid_point())
            .lat_inc(self.section3.j_direction_increment())
            .lon_inc(self.section3.i_direction_increment())
            .nbit(judgment.section5.bits_per_value() as u16)
            .maxv(judgment.section5.max_level_value())
            .level_values(judgment.section5.level_values())
            .build()
    }
}

impl LwjmJudgment {
    /// 第4節:プロダクト定義節から第7節:資料節を読み込む。
    ///
    /// # 引数
    ///
    /// * `reader` - 土砂災害警戒判定メッシュファイルリーダー
    ///
    /// # 戻り値
    ///
    /// * 第4節:プロダクト定義節から第7節:資料節
    fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Grib2Result<Self> {
        let section4 = Section4_50000::from_reader(reader)?;
        let section5 = Section5_200i16::from_reader(reader)?;
        let section6 = Section6::from_reader(reader)?;
        let section7 = Section7_200::from_reader(reader)?;

        Ok(Self {
            section4,
            section5,
            section6,
            section7,
        })
    }
}

/// 土砂災害警戒判定時間
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LwjmHour {
    /// 実況
    Live = 0,
    /// 1時間予想
    Hour1 = 1,
    /// 2時間予想
    Hour2 = 2,
    /// 3時間予想
    Hour3 = 3,
}

impl TryFrom<u8> for LwjmHour {
    type Error = Grib2Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Live),
            1 => Ok(Self::Hour1),
            2 => Ok(Self::Hour2),
            3 => Ok(Self::Hour3),
            _ => Err(Grib2Error::ConvertError(
                format!("`{value}`を`LwjmHour`型に変換できません。").into(),
            )),
        }
    }
}
