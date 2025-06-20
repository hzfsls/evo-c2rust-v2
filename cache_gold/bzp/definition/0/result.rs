pub type BZP_ERROR_BASE_NO = i32;
macro_rules! BZP_ERROR_MEMORY_OPER_FAILURE { () => { 1 } }
macro_rules! BZP_ERROR_PARAM { () => { 2 } }
macro_rules! BZP_ERROR_IO { () => { 3 } }
macro_rules! BZP_ERROR_DATA { () => { 4 } }
macro_rules! BZP_ERROR_DATA_MAGIC { () => { 5 } }
pub(crate) use BZP_ERROR_MEMORY_OPER_FAILURE;
pub(crate) use BZP_ERROR_PARAM;
pub(crate) use BZP_ERROR_IO;
pub(crate) use BZP_ERROR_DATA;
pub(crate) use BZP_ERROR_DATA_MAGIC;