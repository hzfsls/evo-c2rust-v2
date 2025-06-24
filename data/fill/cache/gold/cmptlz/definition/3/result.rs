pub type EnCmptLzMemType = i32;
macro_rules! CMPTLZ_DICT_MEM { () => { 1 } }
macro_rules! CMPTLZ_PROB_MEM { () => { 2 } }
macro_rules! CMPTLZ_ENC_CCTX { () => { 3 } }
macro_rules! CMPTLZ_MF_CCTX { () => { 4 } }
macro_rules! CMPTLZ_MF_HASH { () => { 5 } }
macro_rules! CMPTLZ_MF_SON { () => { 6 } }
macro_rules! CMPTLZ_RC_CCTX { () => { 7 } }
macro_rules! CMPTLZ_RC_BUF { () => { 8 } }
macro_rules! CMPTLZ_MEM_TYPE_BUT { () => { 9 } }
pub(crate) use CMPTLZ_DICT_MEM;
pub(crate) use CMPTLZ_PROB_MEM;
pub(crate) use CMPTLZ_ENC_CCTX;
pub(crate) use CMPTLZ_MF_CCTX;
pub(crate) use CMPTLZ_MF_HASH;
pub(crate) use CMPTLZ_MF_SON;
pub(crate) use CMPTLZ_RC_CCTX;
pub(crate) use CMPTLZ_RC_BUF;
pub(crate) use CMPTLZ_MEM_TYPE_BUT;