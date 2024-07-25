use std::path::PathBuf;

use crate::readers::sections::{
    Section0, Section1, Section2, Section3_0, Section4_50008, Section6, Section7_200, Section8,
};

/// 解析雨量ファイルリーダー
pub struct PrrReader {
    /// ファイルパス
    pub path: PathBuf,
    /// 第0節:指示節
    section0: Section0,
    /// 第1節:識別節
    section1: Section1,
    /// 第2節:地域使用節
    section2: Section2,
    /// 第３節:格子系定義節
    section3: Section3_0,
    /// 第４節:プロダクト定義節
    section4: Section4_50008,
    /// 第５節:資料表現節
    // section5: Section5_200u16,
    /// 第６節:ビットマップ節
    section6: Section6,
    /// 第７節:資料節
    section7: Section7_200,
    /// 第８節:終端節
    section8: Section8,
}
