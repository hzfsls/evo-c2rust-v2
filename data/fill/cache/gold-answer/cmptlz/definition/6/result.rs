pub type EnCmptLzFinMode = i32;
macro_rules! CMPTLZ_FINISH_ANY { () => { 0 } }
macro_rules! CMPTLZ_FINISH_END { () => { 1 } }
pub(crate) use CMPTLZ_FINISH_ANY;
pub(crate) use CMPTLZ_FINISH_END;