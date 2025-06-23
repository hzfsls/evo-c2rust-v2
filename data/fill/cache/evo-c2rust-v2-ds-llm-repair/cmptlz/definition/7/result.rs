pub type EnCmptLzStatus = i32;
macro_rules! CMPTLZ_STATUS_NOT_SPECIFIED { () => { 0 } }
pub(crate) use CMPTLZ_STATUS_NOT_SPECIFIED;
macro_rules! CMPTLZ_STATUS_FINISHED_WITH_MARK { () => { 1 } }
pub(crate) use CMPTLZ_STATUS_FINISHED_WITH_MARK;
macro_rules! CMPTLZ_STATUS_NOT_FINISHED { () => { 2 } }
pub(crate) use CMPTLZ_STATUS_NOT_FINISHED;
macro_rules! CMPTLZ_STATUS_NEEDS_MORE_INPUT { () => { 3 } }
pub(crate) use CMPTLZ_STATUS_NEEDS_MORE_INPUT;
macro_rules! CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK { () => { 4 } }
pub(crate) use CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
macro_rules! CMPTLZ_STATUS_BUT { () => { 5 } }
pub(crate) use CMPTLZ_STATUS_BUT;
