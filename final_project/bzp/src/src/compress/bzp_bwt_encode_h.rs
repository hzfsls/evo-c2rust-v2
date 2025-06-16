use crate::translation_utils::*;
pub use crate::src::public::bzp_utils_h::*;
pub use crate::src::compress::bzp_bwt_encode_c::BzpQSortSingle;
pub use crate::src::compress::bzp_bwt_encode_c::BzpBinaryLiftingSort;
pub use crate::src::compress::bzp_bwt_encode_c::BzpUpdateflag;
pub use crate::src::compress::bzp_bwt_encode_c::BzpQuickSort;
pub use crate::src::compress::bzp_bwt_encode_c::BzpBwtFinish;
pub use crate::src::compress::bzp_bwt_encode_c::BzpSwap3Elem;
pub use crate::src::compress::bzp_bwt_encode_c::BzpSwap2Elem;
pub use crate::src::compress::bzp_bwt_encode_c::BzpSelectMidVal;
pub use crate::src::compress::bzp_bwt_encode_c::BzpBlockSortMain;
pub use crate::src::compress::bzp_bwt_encode_c::BzpBlockSortInit;
pub use crate::src::compress::bzp_bwt_encode_c::BzpShellSort;

#[repr(C)]
#[derive(Default)]
pub struct BzpBwtInfo {
    pub sortBlock: Ptr<i32>,
    pub idx: Ptr<i32>,
    pub isStartPos: Ptr<i32>,
    pub block: Ptr<u8>,
    pub blockCRC: u32,
    pub combinedCRC: u32,
    pub nBlockMax: i32,
    pub blockId: i32,
    pub nBlock: i32,
    pub oriPtr: i32,
    pub inUse: Array<bool, { BZP_ASCII_SIZE!() }>,
}


#[repr(C)]
#[derive(Default)]
pub struct BzpQSortInfo {
    pub stackL: Array<i32, { BZP_MAX_STACK_SIZE!() }>,
    pub stackR: Array<i32, { BZP_MAX_STACK_SIZE!() }>,
    pub cnt: i32,
    pub tl: i32,
    pub tr: i32,
}


macro_rules! BZP_BWT_ENCODE_H { () => { } }
pub(crate) use BZP_BWT_ENCODE_H;


