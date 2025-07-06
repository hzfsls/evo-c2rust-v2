pub type EnCmptLzStatus = i32;
macro_rules! CMPTLZ_STATUS_NOT_SPECIFIED { () => { 0 } }
macro_rules! CMPTLZ_STATUS_FINISHED_WITH_MARK { () => { 1 } }
macro_rules! CMPTLZ_STATUS_NOT_FINISHED { () => { 2 } }
macro_rules! CMPTLZ_STATUS_NEEDS_MORE_INPUT { () => { 3 } }
macro_rules! CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK { () => { 4 } }
macro_rules! CMPTLZ_STATUS_BUT { () => { 5 } }
pub(crate) use CMPTLZ_STATUS_NOT_SPECIFIED;
pub(crate) use CMPTLZ_STATUS_FINISHED_WITH_MARK;
pub(crate) use CMPTLZ_STATUS_NOT_FINISHED;
pub(crate) use CMPTLZ_STATUS_NEEDS_MORE_INPUT;
pub(crate) use CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
pub(crate) use CMPTLZ_STATUS_BUT;