use crate::translation_utils::*;
pub use crate::src::public::bzp_utils_h::*;
pub use crate::src::compress::bzp_huffman_encode_c::BzpBuildTreeBalanceHeight;
pub use crate::src::compress::bzp_huffman_encode_c::BzpGenerateSelectMTF;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHeapInit;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanGroupsReset;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanGroupsInit;
pub use crate::src::compress::bzp_huffman_encode_c::BzpSelectTree;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHeapAdjustUp;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanMain;
pub use crate::src::compress::bzp_huffman_encode_c::BzpGetCodeLen;
pub use crate::src::compress::bzp_huffman_encode_c::BzpInitLenArray;
pub use crate::src::compress::bzp_huffman_encode_c::BzpGetHuffmanTable;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHeapAdjustDown;
pub use crate::src::compress::bzp_huffman_encode_c::BzpBuildHuffmanTree;
pub use crate::src::compress::bzp_huffman_encode_c::BzpBzpHuffmanGroupsFinish;
pub use crate::src::compress::bzp_huffman_encode_c::BzpGetHuffmanGroups;
pub use crate::src::compress::bzp_huffman_encode_c::BzpCalculateCost;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanInit;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanInitArray;
pub use crate::src::compress::bzp_huffman_encode_c::BzpHuffmanWeightAdd;

#[repr(C)]
#[derive(Default)]
pub struct BzpHuffmanInfo {
    pub heap: Array<i32, { BZP_MAX_ALPHA_SIZE!() + 1 }>,
    pub weight: Array<i32, { BZP_MAX_ALPHA_SIZE!() * 2 }>,
    pub parent: Array<i32, { BZP_MAX_ALPHA_SIZE!() * 2 }>,
    pub len: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub table: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub nHeap: i32,
    pub nWeight: i32,
    pub alphaSize: i32,
}


#[repr(C)]
#[derive(Default)]
pub struct BzpHuffmanGroups {
    pub block: Ptr<i32>,
    pub mtfFreq: Ptr<i32>,
    pub select: Ptr<i32>,
    pub selectMTF: Ptr<i32>,
    pub huffmanGroups: Array<BzpHuffmanInfo, { BZP_MAX_GROUPS_NUM!() }>,
    pub cost: Array<i32, { BZP_MAX_GROUPS_NUM!() }>,
    pub nGroups: i32,
    pub nBlock: i32,
    pub nSelect: i32,
    pub alphaSize: i32,
}


macro_rules! BZP_HUFFMAN_ENCODE_H { () => {  } }
pub(crate) use BZP_HUFFMAN_ENCODE_H;


