use crate::translation_utils::*;
pub use crate::src::public::bzp_utils_h::*;
pub use crate::src::compress::bzp_mtf_encode_c::BzpMtfFinish;
pub use crate::src::compress::bzp_mtf_encode_c::BzpNumEncode;
pub use crate::src::compress::bzp_mtf_encode_c::BzpMapInputChar;
pub use crate::src::compress::bzp_mtf_encode_c::BzpMtfInit;
pub use crate::src::compress::bzp_mtf_encode_c::BzpMtfReSet;
pub use crate::src::compress::bzp_mtf_encode_c::BzpMtfMain;

#[repr(C)]
#[derive(Default)]
pub struct BzpMtfInfo {
    pub block: Ptr<u8>,
    pub map: Ptr<i32>,
    pub mtfV: Ptr<i32>,
    pub inUse: Ptr<bool>,
    pub mtfFreq: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub nBlock: i32,
    pub nMtf: i32,
    pub nUse: i32,
    pub pad: i32,
}


macro_rules! BZP_MTF_ENCODE_H { () => { } }
pub(crate) use BZP_MTF_ENCODE_H;


