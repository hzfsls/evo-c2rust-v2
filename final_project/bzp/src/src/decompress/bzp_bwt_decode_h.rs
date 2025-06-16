use crate::translation_utils::*;
pub use crate::src::public::bzp_utils_h::*;
pub use crate::src::decompress::bzp_bwt_decode_c::BzpBwtDecodeFinish;
pub use crate::src::decompress::bzp_bwt_decode_c::BzpBwtDecodeInit;
pub use crate::src::decompress::bzp_bwt_decode_c::BzpBwtDecode;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpBwtDecodeInfo {
    pub sorted: Ptr<i32>,
    pub block: Ptr<u8>,
    pub deCode: Ptr<u8>,
    pub nBlock: i32,
    pub oriPtr: i32,
}


macro_rules! BZP_BWT_DECODE_H { () => { } }
pub(crate) use BZP_BWT_DECODE_H;


