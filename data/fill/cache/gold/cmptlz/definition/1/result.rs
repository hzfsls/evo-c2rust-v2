pub type EnCmptErrNo = i32;
macro_rules! CMPTLZ_ERROR_DATA { () => { 1 } }
macro_rules! CMPTLZ_ERROR_MEM { () => { 2 } }
macro_rules! CMPTLZ_ERROR_UNSUPPORTED { () => { 3 } }
macro_rules! CMPTLZ_ENC_ERROR_FILESIZE { () => { 4 } }
macro_rules! CMPTLZ_ENC_CTX_INIT_FAIL { () => { 5 } }
macro_rules! CMPTLZ_ENC_RC_INIT_FAIL { () => { 6 } }
macro_rules! CMPTLZ_ENC_MF_INIT_FAIL { () => { 7 } }
macro_rules! CMPTLZ_ENC_ERROR_WRITE { () => { 8 } }
macro_rules! CMPTLZ_ENC_ERROR_HEAD { () => { 9 } }
macro_rules! CMPTLZ_ENC_ERROR_PARAM { () => { 10 } }
macro_rules! CMPTLZ_ERROR_BUTT { () => { 11 } }
pub(crate) use CMPTLZ_ERROR_DATA;
pub(crate) use CMPTLZ_ERROR_MEM;
pub(crate) use CMPTLZ_ERROR_UNSUPPORTED;
pub(crate) use CMPTLZ_ENC_ERROR_FILESIZE;
pub(crate) use CMPTLZ_ENC_CTX_INIT_FAIL;
pub(crate) use CMPTLZ_ENC_RC_INIT_FAIL;
pub(crate) use CMPTLZ_ENC_MF_INIT_FAIL;
pub(crate) use CMPTLZ_ENC_ERROR_WRITE;
pub(crate) use CMPTLZ_ENC_ERROR_HEAD;
pub(crate) use CMPTLZ_ENC_ERROR_PARAM;
pub(crate) use CMPTLZ_ERROR_BUTT;