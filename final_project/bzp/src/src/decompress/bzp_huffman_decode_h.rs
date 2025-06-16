use crate::translation_utils::*;
pub use crate::src::public::bzp_utils_h::*;
pub use crate::src::decompress::bzp_huffman_decode_c::BzpGetOneTable;
pub use crate::src::decompress::bzp_huffman_decode_c::BzpHuffmanDecodeFinish;
pub use crate::src::decompress::bzp_huffman_decode_c::BzpGenerateDecodeTable;
pub use crate::src::decompress::bzp_huffman_decode_c::BzpHuffmanDecodeReset;
pub use crate::src::decompress::bzp_huffman_decode_c::BzpHuffmanDecodeInit;

#[repr(C)]
#[derive(Default)]
pub struct BzpHuffmanDecode {
    pub select: Ptr<i32>,
    pub len: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub perm: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub limit: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub base: Array<Array<i32, { BZP_MAX_ALPHA_SIZE!() }>, { BZP_MAX_GROUPS_NUM!() }>,
    pub minLens: Array<i32, { BZP_MAX_GROUPS_NUM!() }>,
    pub nGroups: i32,
    pub nSelect: i32,
    pub alphaSize: i32,
    pub deCodeNum: i32,
    pub selectCnt: i32,
    pub nBlock: i32,
}


macro_rules! BZP_HUFFMAN_DECODE_H { () => { } }
pub(crate) use BZP_HUFFMAN_DECODE_H;


