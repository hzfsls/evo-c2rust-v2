pub type EnCmptErrNo = i32;
macro_rules! CMPTLZ_ERROR_DATA { () => { 1 } }
pub(crate) use CMPTLZ_ERROR_DATA;
macro_rules! CMPTLZ_ERROR_MEM { () => { 2 } }
pub(crate) use CMPTLZ_ERROR_MEM;
macro_rules! CMPTLZ_ERROR_UNSUPPORTED { () => { 3 } }
pub(crate) use CMPTLZ_ERROR_UNSUPPORTED;
macro_rules! CMPTLZ_ENC_ERROR_FILESIZE { () => { 4 } }
pub(crate) use CMPTLZ_ENC_ERROR_FILESIZE;
macro_rules! CMPTLZ_ENC_CTX_INIT_FAIL { () => { 5 } }
pub(crate) use CMPTLZ_ENC_CTX_INIT_FAIL;
macro_rules! CMPTLZ_ENC_RC_INIT_FAIL { () => { 6 } }
pub(crate) use CMPTLZ_ENC_RC_INIT_FAIL;
macro_rules! CMPTLZ_ENC_MF_INIT_FAIL { () => { 7 } }
pub(crate) use CMPTLZ_ENC_MF_INIT_FAIL;
macro_rules! CMPTLZ_ENC_ERROR_WRITE { () => { 8 } }
pub(crate) use CMPTLZ_ENC_ERROR_WRITE;
macro_rules! CMPTLZ_ENC_ERROR_HEAD { () => { 9 } }
pub(crate) use CMPTLZ_ENC_ERROR_HEAD;
macro_rules! CMPTLZ_ENC_ERROR_PARAM { () => { 10 } }
pub(crate) use CMPTLZ_ENC_ERROR_PARAM;
macro_rules! CMPTLZ_ERROR_BUTT { () => { 11 } }
pub(crate) use CMPTLZ_ERROR_BUTT;
