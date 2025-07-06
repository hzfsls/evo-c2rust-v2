use crate::translation_utils::*;

pub type BZP_ERROR_BASE_NO = i32;
macro_rules! BZP_ERROR_MEMORY_OPER_FAILURE {
    () => {
        1
    };
}
pub(crate) use BZP_ERROR_MEMORY_OPER_FAILURE;
macro_rules! BZP_ERROR_PARAM {
    () => {
        2
    };
}
pub(crate) use BZP_ERROR_PARAM;
macro_rules! BZP_ERROR_IO {
    () => {
        3
    };
}
pub(crate) use BZP_ERROR_IO;
macro_rules! BZP_ERROR_DATA {
    () => {
        4
    };
}
pub(crate) use BZP_ERROR_DATA;
macro_rules! BZP_ERROR_DATA_MAGIC {
    () => {
        5
    };
}
pub(crate) use BZP_ERROR_DATA_MAGIC;

#[repr(C)]
#[derive(Default)]
pub struct BzpStream {
    pub filePtr: FilePtr,
    pub nBuf: i32,
    pub pos: i32,
    pub buf: Array<u8, { BZP_BUF_SIZE!() }>,
}

#[repr(C)]
#[derive(Default)]
pub struct InDeComdata {
    pub input: Ptr<BzpStream>,
    pub output: Ptr<BzpStream>,
    pub lasChar: i32,
    pub num: i32,
    pub buf: u32,
    pub nBuf: i32,
    pub blockSize: i32,
    pub blockCRC: u32,
    pub list: Array<i32, { BZP_ASCII_SIZE!() }>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpFile {
    pub input: Ptr<BzpStream>,
    pub output: Ptr<BzpStream>,
    pub state: i32,
    pub lasChar: i32,
    pub num: i32,
    pub pad: i32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpOutComdata {
    pub out: Ptr<u8>,
    pub num: i32,
    pub buf: u32,
    pub nBuf: i32,
    pub blockSize: i32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpAlgorithmInfo {
    pub bwt: Ptr<BzpBwtInfo>,
    pub huffman: Ptr<BzpHuffmanGroups>,
    pub mtf: Ptr<BzpMtfInfo>,
    pub compressFile: Ptr<BzpFile>,
    pub outData: Ptr<BzpOutComdata>,
}

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

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpBwtDecodeInfo {
    pub sorted: Ptr<i32>,
    pub block: Ptr<u8>,
    pub deCode: Ptr<u8>,
    pub nBlock: i32,
    pub oriPtr: i32,
}

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

pub static g_bzpCRC32Table: Global<Array<i32, 256>> = global!(arr![
    0x00000000, 0x04c11db7, 0x09823b6e, 0x0d4326d9, 0x130476dc, 0x17c56b6b, 0x1a864db2, 0x1e475005,
    0x2608edb8, 0x22c9f00f, 0x2f8ad6d6, 0x2b4bcb61, 0x350c9b64, 0x31cd86d3, 0x3c8ea00a, 0x384fbdbd,
    0x4c11db70, 0x48d0c6c7, 0x4593e01e, 0x4152fda9, 0x5f15adac, 0x5bd4b01b, 0x569796c2, 0x52568b75,
    0x6a1936c8, 0x6ed82b7f, 0x639b0da6, 0x675a1011, 0x791d4014, 0x7ddc5da3, 0x709f7b7a, 0x745e66cd,
    0x9823b6e0, 0x9ce2ab57, 0x91a18d8e, 0x95609039, 0x8b27c03c, 0x8fe6dd8b, 0x82a5fb52, 0x8664e6e5,
    0xbe2b5b58, 0xbaea46ef, 0xb7a96036, 0xb3687d81, 0xad2f2d84, 0xa9ee3033, 0xa4ad16ea, 0xa06c0b5d,
    0xd4326d90, 0xd0f37027, 0xddb056fe, 0xd9714b49, 0xc7361b4c, 0xc3f706fb, 0xceb42022, 0xca753d95,
    0xf23a8028, 0xf6fb9d9f, 0xfbb8bb46, 0xff79a6f1, 0xe13ef6f4, 0xe5ffeb43, 0xe8bccd9a, 0xec7dd02d,
    0x34867077, 0x30476dc0, 0x3d044b19, 0x39c556ae, 0x278206ab, 0x23431b1c, 0x2e003dc5, 0x2ac12072,
    0x128e9dcf, 0x164f8078, 0x1b0ca6a1, 0x1fcdbb16, 0x018aeb13, 0x054bf6a4, 0x0808d07d, 0x0cc9cdca,
    0x7897ab07, 0x7c56b6b0, 0x71159069, 0x75d48dde, 0x6b93dddb, 0x6f52c06c, 0x6211e6b5, 0x66d0fb02,
    0x5e9f46bf, 0x5a5e5b08, 0x571d7dd1, 0x53dc6066, 0x4d9b3063, 0x495a2dd4, 0x44190b0d, 0x40d816ba,
    0xaca5c697, 0xa864db20, 0xa527fdf9, 0xa1e6e04e, 0xbfa1b04b, 0xbb60adfc, 0xb6238b25, 0xb2e29692,
    0x8aad2b2f, 0x8e6c3698, 0x832f1041, 0x87ee0df6, 0x99a95df3, 0x9d684044, 0x902b669d, 0x94ea7b2a,
    0xe0b41de7, 0xe4750050, 0xe9362689, 0xedf73b3e, 0xf3b06b3b, 0xf771768c, 0xfa325055, 0xfef34de2,
    0xc6bcf05f, 0xc27dede8, 0xcf3ecb31, 0xcbffd686, 0xd5b88683, 0xd1799b34, 0xdc3abded, 0xd8fba05a,
    0x690ce0ee, 0x6dcdfd59, 0x608edb80, 0x644fc637, 0x7a089632, 0x7ec98b85, 0x738aad5c, 0x774bb0eb,
    0x4f040d56, 0x4bc510e1, 0x46863638, 0x42472b8f, 0x5c007b8a, 0x58c1663d, 0x558240e4, 0x51435d53,
    0x251d3b9e, 0x21dc2629, 0x2c9f00f0, 0x285e1d47, 0x36194d42, 0x32d850f5, 0x3f9b762c, 0x3b5a6b9b,
    0x0315d626, 0x07d4cb91, 0x0a97ed48, 0x0e56f0ff, 0x1011a0fa, 0x14d0bd4d, 0x19939b94, 0x1d528623,
    0xf12f560e, 0xf5ee4bb9, 0xf8ad6d60, 0xfc6c70d7, 0xe22b20d2, 0xe6ea3d65, 0xeba91bbc, 0xef68060b,
    0xd727bbb6, 0xd3e6a601, 0xdea580d8, 0xda649d6f, 0xc423cd6a, 0xc0e2d0dd, 0xcda1f604, 0xc960ebb3,
    0xbd3e8d7e, 0xb9ff90c9, 0xb4bcb610, 0xb07daba7, 0xae3afba2, 0xaafbe615, 0xa7b8c0cc, 0xa379dd7b,
    0x9b3660c6, 0x9ff77d71, 0x92b45ba8, 0x9675461f, 0x8832161a, 0x8cf30bad, 0x81b02d74, 0x857130c3,
    0x5d8a9099, 0x594b8d2e, 0x5408abf7, 0x50c9b640, 0x4e8ee645, 0x4a4ffbf2, 0x470cdd2b, 0x43cdc09c,
    0x7b827d21, 0x7f436096, 0x7200464f, 0x76c15bf8, 0x68860bfd, 0x6c47164a, 0x61043093, 0x65c52d24,
    0x119b4be9, 0x155a565e, 0x18197087, 0x1cd86d30, 0x029f3d35, 0x065e2082, 0x0b1d065b, 0x0fdc1bec,
    0x3793a651, 0x3352bbe6, 0x3e119d3f, 0x3ad08088, 0x2497d08d, 0x2056cd3a, 0x2d15ebe3, 0x29d4f654,
    0xc5a92679, 0xc1683bce, 0xcc2b1d17, 0xc8ea00a0, 0xd6ad50a5, 0xd26c4d12, 0xdf2f6bcb, 0xdbee767c,
    0xe3a1cbc1, 0xe760d676, 0xea23f0af, 0xeee2ed18, 0xf0a5bd1d, 0xf464a0aa, 0xf9278673, 0xfde69bc4,
    0x89b8fd09, 0x8d79e0be, 0x803ac667, 0x84fbdbd0, 0x9abc8bd5, 0x9e7d9662, 0x933eb0bb, 0x97ffad0c,
    0xafb010b1, 0xab710d06, 0xa6322bdf, 0xa2f33668, 0xbcb4666d, 0xb8757bda, 0xb5365d03, 0xb1f740b4
]);

macro_rules! BZP_OK {
    () => {
        0
    };
}
pub(crate) use BZP_OK;

macro_rules! BZP_BASE_BLOCK_SIZE {
    () => {
        100000
    };
}
pub(crate) use BZP_BASE_BLOCK_SIZE;

macro_rules! BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT {
    () => {
        9
    };
}
pub(crate) use BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT;

macro_rules! BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT {
    () => {
        1
    };
}
pub(crate) use BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT;

macro_rules! BZP_BLOCK_RESERVED_SPACE_SIZE {
    () => {
        19
    };
}
pub(crate) use BZP_BLOCK_RESERVED_SPACE_SIZE;

macro_rules! BZP_THRESHOLD_SHELL_SORT {
    () => {
        10
    };
}
pub(crate) use BZP_THRESHOLD_SHELL_SORT;

macro_rules! BZP_MAX_STACK_SIZE {
    () => {
        100
    };
}
pub(crate) use BZP_MAX_STACK_SIZE;

macro_rules! BZP_ASCII_SIZE {
    () => {
        256
    };
}
pub(crate) use BZP_ASCII_SIZE;

macro_rules! BZP_SHELL_SORT_INCREMENT_NUMS {
    () => {
        2
    };
}
pub(crate) use BZP_SHELL_SORT_INCREMENT_NUMS;

macro_rules! BZP_SHELL_SORT_INCREMENT0 {
    () => {
        1
    };
}
pub(crate) use BZP_SHELL_SORT_INCREMENT0;

macro_rules! BZP_SHELL_SORT_INCREMENT1 {
    () => {
        4
    };
}
pub(crate) use BZP_SHELL_SORT_INCREMENT1;

macro_rules! BZP_MTF_ENCODE0 {
    () => {
        0
    };
}
pub(crate) use BZP_MTF_ENCODE0;

macro_rules! BZP_MTF_ENCODE1 {
    () => {
        1
    };
}
pub(crate) use BZP_MTF_ENCODE1;

macro_rules! BZP_MTF_ENCODE_BASE {
    () => {
        2
    };
}
pub(crate) use BZP_MTF_ENCODE_BASE;

macro_rules! BZP_INIT_BLOCK_CRC {
    () => {
        0xffffffff
    };
}
pub(crate) use BZP_INIT_BLOCK_CRC;

macro_rules! BZP_MAX_ALPHA_SIZE {
    () => {
        258
    };
}
pub(crate) use BZP_MAX_ALPHA_SIZE;

macro_rules! BZP_MAX_GROUPS_NUM {
    () => {
        6
    };
}
pub(crate) use BZP_MAX_GROUPS_NUM;

macro_rules! BZP_MAX_ITER_NUM {
    () => {
        4
    };
}
pub(crate) use BZP_MAX_ITER_NUM;

macro_rules! BZP_MAX_TREE_HEIGHT_ENCODE {
    () => {
        17
    };
}
pub(crate) use BZP_MAX_TREE_HEIGHT_ENCODE;

macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT0 {
    () => {
        200
    };
}
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT0;

macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT1 {
    () => {
        600
    };
}
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT1;

macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT2 {
    () => {
        1200
    };
}
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT2;

macro_rules! BZP_NGROUPS_BLOCK_NUM_LIMIT3 {
    () => {
        2400
    };
}
pub(crate) use BZP_NGROUPS_BLOCK_NUM_LIMIT3;

macro_rules! BZP_NGROUPS_NUM_0 {
    () => {
        2
    };
}
pub(crate) use BZP_NGROUPS_NUM_0;

macro_rules! BZP_NGROUPS_NUM_1 {
    () => {
        3
    };
}
pub(crate) use BZP_NGROUPS_NUM_1;

macro_rules! BZP_NGROUPS_NUM_2 {
    () => {
        4
    };
}
pub(crate) use BZP_NGROUPS_NUM_2;

macro_rules! BZP_NGROUPS_NUM_3 {
    () => {
        5
    };
}
pub(crate) use BZP_NGROUPS_NUM_3;

macro_rules! BZP_NGROUPS_NUM_4 {
    () => {
        6
    };
}
pub(crate) use BZP_NGROUPS_NUM_4;

macro_rules! BZP_ELEMS_NUM_IN_ONE_GROUP {
    () => {
        50
    };
}
pub(crate) use BZP_ELEMS_NUM_IN_ONE_GROUP;

macro_rules! BZP_HUFFMAN_HEIGHT_WEIGHT_BITS {
    () => {
        8
    };
}
pub(crate) use BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;

macro_rules! BZP_HUFFMAN_LEN_MAX_COST {
    () => {
        15
    };
}
pub(crate) use BZP_HUFFMAN_LEN_MAX_COST;

macro_rules! BZP_HUFFMAN_LEN_UPPER_LIMIT {
    () => {
        20
    };
}
pub(crate) use BZP_HUFFMAN_LEN_UPPER_LIMIT;

macro_rules! BZP_HUFFMAN_MAX_SIZE_SELECT {
    () => {
        BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!() * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!()
    };
}
pub(crate) use BZP_HUFFMAN_MAX_SIZE_SELECT;

macro_rules! BZP_HDR_B {
    () => {
        0x42
    };
}
pub(crate) use BZP_HDR_B;

macro_rules! BZP_HDR_Z {
    () => {
        0x5a
    };
}
pub(crate) use BZP_HDR_Z;

macro_rules! BZP_HDR_H {
    () => {
        0x68
    };
}
pub(crate) use BZP_HDR_H;

macro_rules! BZP_HDR_0 {
    () => {
        0x30
    };
}
pub(crate) use BZP_HDR_0;

macro_rules! BZP_BLOCK_HEAD_0 {
    () => {
        0x31
    };
}
pub(crate) use BZP_BLOCK_HEAD_0;

macro_rules! BZP_BLOCK_HEAD_1 {
    () => {
        0x41
    };
}
pub(crate) use BZP_BLOCK_HEAD_1;

macro_rules! BZP_BLOCK_HEAD_2 {
    () => {
        0x59
    };
}
pub(crate) use BZP_BLOCK_HEAD_2;

macro_rules! BZP_BLOCK_HEAD_3 {
    () => {
        0x26
    };
}
pub(crate) use BZP_BLOCK_HEAD_3;

macro_rules! BZP_BLOCK_HEAD_4 {
    () => {
        0x53
    };
}
pub(crate) use BZP_BLOCK_HEAD_4;

macro_rules! BZP_BLOCK_HEAD_5 {
    () => {
        0x59
    };
}
pub(crate) use BZP_BLOCK_HEAD_5;

macro_rules! BZP_FILE_END_0 {
    () => {
        0x17
    };
}
pub(crate) use BZP_FILE_END_0;

macro_rules! BZP_FILE_END_1 {
    () => {
        0x72
    };
}
pub(crate) use BZP_FILE_END_1;

macro_rules! BZP_FILE_END_2 {
    () => {
        0x45
    };
}
pub(crate) use BZP_FILE_END_2;

macro_rules! BZP_FILE_END_3 {
    () => {
        0x38
    };
}
pub(crate) use BZP_FILE_END_3;

macro_rules! BZP_FILE_END_4 {
    () => {
        0x50
    };
}
pub(crate) use BZP_FILE_END_4;

macro_rules! BZP_FILE_END_5 {
    () => {
        0x90
    };
}
pub(crate) use BZP_FILE_END_5;

macro_rules! BZP_BUF_SIZE {
    () => {
        5000
    };
}
pub(crate) use BZP_BUF_SIZE;

macro_rules! BZP_EOF {
    () => {
        -1
    };
}
pub(crate) use BZP_EOF;

macro_rules! BZP_BIT {
    () => {
        1
    };
}
pub(crate) use BZP_BIT;

macro_rules! BZP_BITS2 {
    () => {
        2
    };
}
pub(crate) use BZP_BITS2;

macro_rules! BZP_BITS3 {
    () => {
        3
    };
}
pub(crate) use BZP_BITS3;

macro_rules! BZP_BITS5 {
    () => {
        5
    };
}
pub(crate) use BZP_BITS5;

macro_rules! BZP_BITS8 {
    () => {
        8
    };
}
pub(crate) use BZP_BITS8;

macro_rules! BZP_BITS15 {
    () => {
        15
    };
}
pub(crate) use BZP_BITS15;

macro_rules! BZP_BITS16 {
    () => {
        16
    };
}
pub(crate) use BZP_BITS16;

macro_rules! BZP_BITS24 {
    () => {
        24
    };
}
pub(crate) use BZP_BITS24;

macro_rules! BZP_BITS32 {
    () => {
        32
    };
}
pub(crate) use BZP_BITS32;

macro_rules! BZP_RLC_NUM_1 {
    () => {
        1
    };
}
pub(crate) use BZP_RLC_NUM_1;

macro_rules! BZP_RLC_NUM_2 {
    () => {
        2
    };
}
pub(crate) use BZP_RLC_NUM_2;

macro_rules! BZP_RLC_NUM_3 {
    () => {
        3
    };
}
pub(crate) use BZP_RLC_NUM_3;

macro_rules! BZP_RLC_NUM_4 {
    () => {
        4
    };
}
pub(crate) use BZP_RLC_NUM_4;

macro_rules! BZP_RLC_NUM_LOWER_LIMIT {
    () => {
        1
    };
}
pub(crate) use BZP_RLC_NUM_LOWER_LIMIT;

macro_rules! BZP_RLC_NUM_UPPER_LIMIT {
    () => {
        255
    };
}
pub(crate) use BZP_RLC_NUM_UPPER_LIMIT;

macro_rules! BZP_GROUPS_ASCII {
    () => {
        16
    };
}
pub(crate) use BZP_GROUPS_ASCII;

macro_rules! BZP_CHARS_PER_GROUP_ASCII {
    () => {
        16
    };
}
pub(crate) use BZP_CHARS_PER_GROUP_ASCII;

macro_rules! BZP_CRC_MOVE_RIGHT_VAL {
    () => {
        31
    };
}
pub(crate) use BZP_CRC_MOVE_RIGHT_VAL;

macro_rules! BZP_HUFFMAN_LEN_INCREASE {
    () => {
        2
    };
}
pub(crate) use BZP_HUFFMAN_LEN_INCREASE;

macro_rules! BZP_HUFFMAN_LEN_REDUCED {
    () => {
        3
    };
}
pub(crate) use BZP_HUFFMAN_LEN_REDUCED;

macro_rules! BZP_EXTRA_CHARS_NUM {
    () => {
        2
    };
}
pub(crate) use BZP_EXTRA_CHARS_NUM;

macro_rules! BZP_INPUT_COMPRESS {
    () => {
        0
    };
}
pub(crate) use BZP_INPUT_COMPRESS;

macro_rules! BZP_OUTPUT_COMPRESS {
    () => {
        1
    };
}
pub(crate) use BZP_OUTPUT_COMPRESS;

macro_rules! BZP_RETUEN_COMPRESS {
    () => {
        2
    };
}
pub(crate) use BZP_RETUEN_COMPRESS;

macro_rules! BZP_INVALID_BLOCK_SIZE {
    ($blockSize:expr) => {
        ($blockSize < BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT!())
            || ($blockSize > BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT!())
    };
}
pub(crate) use BZP_INVALID_BLOCK_SIZE;

macro_rules! BZP_INVALID_ALPHA_SIZE {
    ($alphaSize:expr) => {
        $alphaSize > BZP_MAX_ALPHA_SIZE!() || $alphaSize < 1
    };
}
pub(crate) use BZP_INVALID_ALPHA_SIZE;

macro_rules! BZP_MAX_FUN {
    ($a:expr, $b:expr) => {
        if $a > $b {
            $a
        } else {
            $b
        }
    };
}
pub(crate) use BZP_MAX_FUN;

macro_rules! BZP_MIN_FUN {
    ($a:expr, $b:expr) => {
        if $a < $b {
            $a
        } else {
            $b
        }
    };
}
pub(crate) use BZP_MIN_FUN;

macro_rules! BZP_BLOCK_FULL {
    ($bwt:expr) => {
        $bwt.nBlock >= $bwt.nBlockMax
    };
}
pub(crate) use BZP_BLOCK_FULL;

macro_rules! BZP_BUFF_READ_EMPTY {
    ($bzpf:expr) => {
        $bzpf.input.pos >= $bzpf.input.nBuf
    };
}
pub(crate) use BZP_BUFF_READ_EMPTY;

macro_rules! BZP_UPDATE_CRC {
    ($crcVar:expr, $cha:expr) => {
        $crcVar = (($crcVar << 8) ^ g_bzpCRC32Table[(($crcVar >> 24) ^ ($cha as u8)) as usize]);
    };
}
pub(crate) use BZP_UPDATE_CRC;

pub fn BzpStreamInit() -> Ptr<BzpStream> {
    let mut stream: Ptr<BzpStream> = c_malloc!(c_sizeof!(BzpStream));
    if (stream == NULL!()).as_bool() {
        return NULL!();
    }
    stream.filePtr = NULL!();
    stream.pos = 0;
    stream.nBuf = 0;
    return stream.cast();
}

pub fn BzpStreamFinish(mut stream: Ptr<BzpStream>) {
    if (stream != NULL!()).as_bool() {
        c_free!(stream);
        stream = NULL!();
    }
}

pub fn BzpInDeComdataInit() -> Ptr<InDeComdata> {
    let mut inData: Ptr<InDeComdata> = c_malloc!(c_sizeof!(InDeComdata));
    if (inData == NULL!()).as_bool() {
        return NULL!();
    }
    inData.input = NULL!();
    inData.output = NULL!();
    inData.num = 0;
    inData.lasChar = BZP_ASCII_SIZE!();
    inData.nBuf = 0;
    inData.buf = 0;
    inData.num = 0;
    inData.blockCRC = BZP_INIT_BLOCK_CRC!();
    return inData.cast();
}

pub fn BzpInDeComdataFinish(mut inData: Ptr<InDeComdata>) {
    if (inData != NULL!()).as_bool() {
        c_free!(inData);
        inData = NULL!();
    }
}

pub fn BzpReadBits(mut nBit: i32, mut inData: Ptr<InDeComdata>) -> u32 {
    let mut res: u32 = 0;
    while (inData.nBuf < nBit) {
        if (inData.input.nBuf == inData.input.pos) {
            inData.input.nBuf = c_fread!(
                inData.input.buf,
                c_sizeof!(char),
                c_sizeofval!(inData.input.buf),
                inData.input.filePtr
            );
            inData.input.pos = 0;
        }
        let tmp0 = inData.input.pos;
        let mut data: i32 = (inData.input.buf[tmp0]).cast::<u32>().cast::<i32>();
        inData.buf = (inData.buf << BZP_BITS8!()) | data.cast::<u32>();
        inData.input.pos += 1;
        inData.nBuf += BZP_BITS8!();
    }
    res = (inData.buf >> (inData.nBuf - nBit));
    res = (res & ((1 << nBit) - 1));
    inData.nBuf -= nBit;
    return res;
}

pub fn BzpWriteChar(mut ch: u8, mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inData.output.nBuf >= BZP_BUF_SIZE!()) {
        let mut n2: i32 = c_fwrite!(
            inData.output.buf.cast::<Ptr<Void>>(),
            c_sizeof!(u8),
            inData.output.nBuf,
            inData.output.filePtr
        );
        if (n2 != inData.output.nBuf) {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }
    let tmp0 = inData.output.nBuf.suffix_plus_plus();
    inData.output.buf[tmp0] = ch;
    return ret;
}

pub fn BzpHuffmanDecodeStep(
    mut huffman: Ptr<BzpHuffmanDecode>,
    mut inData: Ptr<InDeComdata>,
) -> i32 {
    if (huffman.deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP!()) {
        huffman.deCodeNum = 0;
        huffman.selectCnt += 1;
    }
    let tmp0 = huffman.selectCnt;
    let mut gid: i32 = huffman.select[tmp0];
    let mut chlen: i32 = huffman.minLens[gid];
    let mut val: i32 = BzpReadBits(chlen, inData).cast();
    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT!()) && (val > huffman.limit[gid][chlen]) {
        chlen += 1;
        let mut nxtbit: i32 = BzpReadBits(1, inData).cast();
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT!()) {
        return -1;
    }
    val = val - huffman.base[gid][chlen];
    val = huffman.perm[gid][val];
    huffman.deCodeNum += 1;
    return val;
}

pub fn BzpCheckFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = Default::default();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_1!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_2!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_3!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_4!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_BLOCK_HEAD_5!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}

pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val.cast();
}

pub fn BzpReadUInt32(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val.cast();
}

pub fn BzpDeHuffmanSelect(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut selectmtf: Array<i32, { BZP_HUFFMAN_MAX_SIZE_SELECT!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut j: i32 = -1;
        c_do!({
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            j.suffix_plus_plus();
        } while ch != 0);
        if (j >= huffman.nGroups).as_bool() {
            return BZP_ERROR_DATA!();
        }
        selectmtf[i] = j.cast();
    });
    let mut listSelect: Array<i32, { BZP_MAX_GROUPS_NUM!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        listSelect[i] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = selectmtf[i].cast();
        let mut tmpv: i32 = listSelect[pos].cast();
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            listSelect[j] = listSelect[j - 1].cast();
        });
        listSelect[0] = tmpv.cast();
        huffman.select[i] = tmpv.cast();
    });
    return BZP_OK!();
}

pub fn BzpDeHuffmanLen(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = BzpReadBits(BZP_BITS5!(), inData.cast()).cast();
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            while (ch != 0).as_bool() {
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
                val += if ch == 0 { 1 } else { -1 };
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            }
            if (val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT!()).as_bool() {
                return BZP_ERROR_DATA!();
            }
            huffman.len[i][j] = val.cast();
        });
    });
    return BZP_OK!();
}

pub fn BzpMTFDeCode(
    mut inData: Ptr<InDeComdata>,
    mut huffman: Ptr<BzpHuffmanDecode>,
    mut debwt: Ptr<BzpBwtDecodeInfo>,
) -> i32 {
    debwt.nBlock = 0;
    let mut ch: u8 = Default::default();
    let mut ninUse: i32 = huffman.alphaSize - BZP_EXTRA_CHARS_NUM!();
    let mut eob: i32 = ninUse + 1;
    let mut val: i32 = BzpHuffmanDecodeStep(huffman, inData);
    while (val != eob) && (val != -1) {
        if (val == 0) || (val == 1) {
            let mut res: i32 = 0;
            let mut basenum: i32 = 1;
            while (val == 0) || (val == 1) {
                res = res + (val + 1) * basenum;
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman, inData);
            }
            c_for!(let mut j: i32 = 0; j < res; j.suffix_plus_plus(); {
                let tmp0 = debwt.nBlock;
                debwt.block[tmp0] = inData.list[0];
                debwt.nBlock += 1;
            });
        } else {
            let mut pos: i32 = val - 1;
            ch = inData.list[pos].cast();
            debwt.block[debwt.nBlock] = ch;
            debwt.nBlock += 1;
            c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
                inData.list[j] = inData.list[j - 1];
            });
            inData.list[0] = ch.cast();
            val = BzpHuffmanDecodeStep(huffman, inData);
        }
    }
    if (val == -1) {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}

pub fn BzpDeCodeToStream(mut inData: Ptr<InDeComdata>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut ret: i32 = BZP_OK!();
    c_for!(let mut i: i32 = 0; i < debwt.nBlock; i.suffix_plus_plus(); {
        ch = debwt.deCode[i];
        if (inData.num == BZP_RLC_NUM_4!()) {
            c_for!(let mut j: i32 = 0; j < ch.cast::<i32>(); j.suffix_plus_plus(); {
                BZP_UPDATE_CRC!(inData.blockCRC, inData.lasChar.cast::<u32>());
                ret |= BzpWriteChar(inData.lasChar.cast(), inData);
            });
            inData.lasChar = BZP_ASCII_SIZE!();
            inData.num = 0;
        } else if (ch == inData.lasChar.cast::<u8>()) {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u32>());
            ret = BzpWriteChar(ch, inData);
            inData.num += 1;
        } else {
            BZP_UPDATE_CRC!(inData.blockCRC, ch.cast::<u32>());
            ret = BzpWriteChar(ch, inData);
            inData.lasChar = ch.cast();
            inData.num = 1;
        }
        if (ret != BZP_OK!()) {
            break;
        }
    });
    return ret;
}

pub fn BzpGetDictionaryList(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ninUse: i32 = 0;
    let mut use16: Array<bool, 16> = arr![false; 16];
    let mut inUse: Array<bool, { BZP_ASCII_SIZE!() }> = arr![false; BZP_ASCII_SIZE!()];
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        use16[i] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        if use16[i].as_bool() {
            c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
                inUse[i * BZP_GROUPS_ASCII!() + j] = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            });
        }
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        if inUse[i].as_bool() {
            inData.list[ninUse.suffix_plus_plus()] = i.cast();
        }
    });
    return ninUse.cast();
}

pub fn BzpDeCompressOneBlock(
    mut inData: Ptr<InDeComdata>,
    mut huffman: Ptr<BzpHuffmanDecode>,
    mut debwt: Ptr<BzpBwtDecodeInfo>,
) -> i32 {
    let mut ret: i32 = BZP_OK!();
    BzpCheckFileHead(inData);
    let mut blockCRC: u32 = BzpReadUInt32(inData);
    BzpReadBits(BZP_BIT!(), inData).cast::<Void>();
    let mut oriPtr: i32 = BzpReadUInt24(inData).cast();
    if (oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE!() * inData.blockSize) {
        return BZP_ERROR_DATA!();
    }
    let mut ninUse: i32 = BzpGetDictionaryList(inData);
    huffman.alphaSize = (ninUse + BZP_EXTRA_CHARS_NUM!());
    huffman.nGroups = BzpReadBits(BZP_BITS3!(), inData).cast::<i32>();
    if (huffman.nGroups < BZP_NGROUPS_NUM_0!() || huffman.nGroups > BZP_NGROUPS_NUM_4!()) {
        return BZP_ERROR_DATA!();
    }
    huffman.nSelect = BzpReadBits(BZP_BITS15!(), inData).cast();
    let mut nSelectUpperLimit: i32 =
        (inData.blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!() + 1);
    if (huffman.nSelect < 1 || huffman.nSelect > nSelectUpperLimit) {
        return BZP_ERROR_DATA!();
    }
    ret |= BzpDeHuffmanSelect(inData, huffman);
    ret |= BzpDeHuffmanLen(inData, huffman);
    if (ret != BZP_OK!()) {
        return ret;
    }
    BzpGenerateDecodeTable(huffman);
    debwt.oriPtr = oriPtr;
    ret = BzpMTFDeCode(inData, huffman, debwt);
    if (ret != BZP_OK!() || (debwt.nBlock >= BZP_BASE_BLOCK_SIZE!() * inData.blockSize)) {
        return BZP_ERROR_DATA!();
    }
    BzpBwtDecode(debwt);
    ret = BzpDeCodeToStream(inData, debwt);
    if (ret != BZP_OK!()) {
        return ret;
    }
    inData.blockCRC = !(inData.blockCRC);
    if (blockCRC != inData.blockCRC) {
        ret = BZP_ERROR_DATA!();
    }
    return ret;
}

pub fn BZPReadFileEnd(mut inData: Ptr<InDeComdata>, mut caltotalCRC: u32) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_1!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_2!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_3!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_4!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_FILE_END_5!()).as_bool() {
        return BZP_ERROR_DATA!();
    }
    let mut storedcombinedcrc: u32 = BzpReadUInt32(inData.cast()).cast();
    if (caltotalCRC != storedcombinedcrc).as_bool() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}

pub fn BzpReadFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_B!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_Z!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_H!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    let mut blockSize: i32 = (ch - BZP_HDR_0!()).cast();
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    inData.blockSize = blockSize.cast();
    return BZP_OK!();
}

pub fn BZPDeCompressData(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut caltotalCRC: u32 = 0;
    let mut ch: u8;
    ret = BzpReadFileHead(inData.cast()).cast();
    if (ret != BZP_OK!()).as_bool() {
        return ret;
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = BzpHuffmanDecodeInit(inData.blockSize.cast());
    let mut debwt: Ptr<BzpBwtDecodeInfo> = BzpBwtDecodeInit(inData.blockSize.cast());
    while {
        ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
        ch != BZP_FILE_END_0!()
    } {
        if (ch != BZP_BLOCK_HEAD_0!()).as_bool() {
            ret = BZP_ERROR_DATA!();
            break;
        }
        BzpHuffmanDecodeReset(huffman.cast());
        inData.blockCRC = BZP_INIT_BLOCK_CRC!();
        ret = BzpDeCompressOneBlock(inData.cast(), huffman.cast(), debwt.cast()).cast();
        if (ret != BZP_OK!()).as_bool() {
            break;
        }
        caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
        caltotalCRC ^= inData.blockCRC;
    }
    if (ret == BZP_OK!()).as_bool() {
        ret = BZPReadFileEnd(inData.cast(), caltotalCRC.cast()).cast();
    }
    BzpHuffmanDecodeFinish(huffman.cast());
    BzpBwtDecodeFinish(debwt.cast());
    return ret.cast();
}

pub fn BzpDeComStreamFinish(
    mut inData: Ptr<InDeComdata>,
    mut inStream: Ptr<BzpStream>,
    mut outStream: Ptr<BzpStream>,
) {
    if (inStream.filePtr != NULL!()).as_bool() {
        c_fclose!(inStream.filePtr);
        inStream.filePtr = NULL!();
    }
    if (outStream.filePtr != NULL!()).as_bool() {
        c_fclose!(outStream.filePtr);
        outStream.filePtr = NULL!();
    }
    BzpStreamFinish(inStream.cast());
    BzpStreamFinish(outStream.cast());
    BzpInDeComdataFinish(inData.cast());
}

pub fn BzpDeCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inName == NULL!()).as_bool() || (outName == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if (inStream == NULL!()).as_bool() || (outStream == NULL!()).as_bool() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inStream.filePtr = c_fopen!(inName, cstr!("rb"));
    outStream.filePtr = c_fopen!(outName, cstr!("wb"));
    if (inStream.filePtr == NULL!()).as_bool() || (outStream.filePtr == NULL!()).as_bool() {
        c_free!(inStream);
        inStream = NULL!();
        c_free!(outStream);
        outStream = NULL!();
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    let mut inData: Ptr<InDeComdata> = BzpInDeComdataInit();
    if (inData == NULL!()).as_bool() {
        BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
        c_remove!(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inData.input = inStream.cast();
    inData.output = outStream.cast();
    ret = BZPDeCompressData(inData.cast()).cast();
    if (inData.output.nBuf > 0).as_bool() {
        let mut n2: i32 = c_fwrite!(
            inData.output.buf.cast::<Ptr<Void>>(),
            c_sizeof!(u8),
            inData.output.nBuf,
            inData.output.filePtr
        );
        if (n2 != inData.output.nBuf).as_bool() {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }
    BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
    if (ret != BZP_OK!()).as_bool() {
        c_remove!(outName);
    }
    return ret.cast();
}

pub fn BzpAlgorithmInfoInit(mut blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if (bzpInfo == NULL!()).as_bool() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize.cast());
    bzpInfo.mtf = BzpMtfInit(blockSize.cast());
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize.cast());
    bzpInfo.outData = BzpOutComDataInit(blockSize.cast());
    bzpInfo.compressFile = BzpFileInit();
    if (bzpInfo.bwt == NULL!()).as_bool()
        || (bzpInfo.outData == NULL!()).as_bool()
        || (bzpInfo.compressFile == NULL!()).as_bool()
        || (bzpInfo.mtf == NULL!()).as_bool()
        || (bzpInfo.huffman == NULL!()).as_bool()
    {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        return NULL!();
    }
    return bzpInfo.cast();
}

pub fn BzpOpenFile(
    mut bzpInfo: Ptr<BzpAlgorithmInfo>,
    mut inName: Ptr<u8>,
    mut outName: Ptr<u8>,
) -> i32 {
    if (bzpInfo == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    bzpInfo.compressFile.input.filePtr = c_fopen!(inName, cstr!("rb"));
    bzpInfo.compressFile.output.filePtr = c_fopen!(outName, cstr!("wb"));
    if (bzpInfo.compressFile.input.filePtr == NULL!()).as_bool()
        || (bzpInfo.compressFile.output.filePtr == NULL!()).as_bool()
    {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    return BZP_OK!();
}

pub fn BzpAlgorithmInfoFinish(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo != NULL!()).as_bool() {
        BzpBwtFinish(bzpInfo.bwt.cast());
        BzpMtfFinish(bzpInfo.mtf.cast());
        BzpHuffmanGroupsFinish(bzpInfo.huffman.cast());
        BzpFileFinish(bzpInfo.compressFile.cast());
        BzpOutComDataFinish(bzpInfo.outData.cast());
        c_free!(bzpInfo);
    }
}

pub fn BzpFileInit() -> Ptr<BzpFile> {
    let mut compressFile: Ptr<BzpFile> = c_malloc!(c_sizeof!(BzpFile));
    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if (compressFile == NULL!()).as_bool()
        || (inStream == NULL!()).as_bool()
        || (outStream == NULL!()).as_bool()
    {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        BzpFileFinish(compressFile.cast());
        return NULL!();
    }
    compressFile.input = inStream.cast();
    compressFile.output = outStream.cast();
    compressFile.input.pos = 0;
    compressFile.output.pos = 0;
    compressFile.num = 0;
    compressFile.lasChar = BZP_ASCII_SIZE!();
    compressFile.state = BZP_INPUT_COMPRESS!();
    return compressFile.cast();
}

pub fn BzpFileFinish(mut bzpF: Ptr<BzpFile>) {
    if (bzpF != NULL!()).as_bool() {
        BzpStreamFinish(bzpF.input.cast());
        BzpStreamFinish(bzpF.output.cast());
        c_free!(bzpF);
        bzpF = NULL!();
    }
}

pub fn BzpOutComDataInit(mut blockSize: i32) -> Ptr<BzpOutComdata> {
    let mut outData: Ptr<BzpOutComdata> = c_malloc!(c_sizeof!(BzpOutComdata));
    if (outData == NULL!()).as_bool() {
        return NULL!();
    }
    outData.out = NULL!();
    outData.out = c_malloc!(blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(u32));
    if (outData.out == NULL!()).as_bool() {
        c_free!(outData);
        return NULL!();
    }
    outData.nBuf = 0;
    outData.buf = 0;
    outData.num = 0;
    outData.blockSize = blockSize;
    return outData.cast();
}

pub fn BzpOutComDataFinish(mut data: Ptr<BzpOutComdata>) {
    if (data != NULL!()).as_bool() {
        if (data.out != NULL!()).as_bool() {
            c_free!(data.out);
            data.out = NULL!();
        }
        c_free!(data);
        data = NULL!();
    }
}

pub fn BzpWriteToArray(mut val: i32, mut n: i32, mut data: Ptr<BzpOutComdata>) {
    while (data.nBuf >= BZP_BITS8!()) {
        let tmp0 = data.num;
        data.out[tmp0] = (data.buf >> BZP_BITS24!()).cast::<u8>();
        data.num += 1;
        data.nBuf -= BZP_BITS8!();
        data.buf <<= BZP_BITS8!();
    }
    data.buf |= (val << (BZP_BITS32!() - n - data.nBuf)).cast::<u32>();
    data.nBuf += n;
}

pub fn BzpWriteInt32(mut val: i32, mut data: Ptr<BzpOutComdata>) {
    BzpWriteToArray(
        ((val >> BZP_BITS24!()) & 0xff).cast::<i32>(),
        BZP_BITS8!(),
        data,
    );
    BzpWriteToArray(
        ((val >> BZP_BITS16!()) & 0xff).cast::<i32>(),
        BZP_BITS8!(),
        data,
    );
    BzpWriteToArray(
        ((val >> BZP_BITS8!()) & 0xff).cast::<i32>(),
        BZP_BITS8!(),
        data,
    );
    BzpWriteToArray((val & 0xff).cast::<i32>(), BZP_BITS8!(), data);
}

pub fn BzpFileEOF(mut f: FilePtr) -> bool {
    let mut c: i32 = c_fgetc!(f);
    if (c == BZP_EOF!()).as_bool() {
        return true;
    }
    c_ungetc!(c, f).cast::<Void>();
    return false;
}

pub fn BzpWriteFileHead(mut outData: Ptr<BzpOutComdata>, mut blockId: i32) {
    if (blockId == 0).as_bool() {
        BzpWriteToArray(BZP_HDR_B!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_Z!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_H!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(
            (BZP_HDR_0!() + outData.blockSize).cast(),
            BZP_BITS8!(),
            outData.cast(),
        );
    }
}

pub fn BzpCalculateCRC(mut bwt: Ptr<BzpBwtInfo>) {
    bwt.blockCRC = !(bwt.blockCRC);
    bwt.combinedCRC = (bwt.combinedCRC << 1) | (bwt.combinedCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
    bwt.combinedCRC ^= bwt.blockCRC;
}

pub fn BzpWriteBlockHead(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    BzpWriteToArray(BZP_BLOCK_HEAD_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(bwt.blockCRC.cast(), outData.cast());
    BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    BzpWriteToArray(bwt.oriPtr.cast(), BZP_BITS24!(), outData.cast());
}

pub fn BzpWriteValidASCII(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    let mut validGid: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    let mut cnt: i32 = 0;
    let mut use16: Array<bool, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(use16, c_sizeofval!(use16), 0, c_sizeofval!(use16)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        let mut gid: i32 = i / BZP_CHARS_PER_GROUP_ASCII!();
        use16[gid] |= bwt.inUse[i];
    });
    c_for!(let mut i: i32 = 0; i < BZP_GROUPS_ASCII!().cast(); i.suffix_plus_plus(); {
        BzpWriteToArray(use16[i].cast::<i32>(), BZP_BIT!(), outData.cast());
        if use16[i].as_bool() {
            validGid[cnt] = i.cast();
            cnt += 1;
        }
    });
    c_for!(let mut i: i32 = 0; i < cnt; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < BZP_CHARS_PER_GROUP_ASCII!(); j.suffix_plus_plus(); {
            let mut valid: i32 = validGid[i] * BZP_CHARS_PER_GROUP_ASCII!() + j;
            BzpWriteToArray(bwt.inUse[valid].cast::<i32>(), BZP_BIT!(), outData.cast());
        });
    });
}

pub fn BzpWriteSelect(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    BzpWriteToArray(huffman.nSelect.cast(), BZP_BITS15!(), outData.cast());
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.selectMTF[i]; j.suffix_plus_plus(); {
            BzpWriteToArray(1, BZP_BIT!(), outData.cast());
        });
        BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    });
}

pub fn BzpWriteLen(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = huffman.huffmanGroups[i].len[0];
        BzpWriteToArray(val.cast(), BZP_BITS5!(), outData.cast());
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            let mut tar: i32 = huffman.huffmanGroups[i].len[j];
            let mut deta: i32 = 0;
            let mut saveVal: i32 = 0;
            if (val < tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_INCREASE!();
                deta = 1;
            } else if (val > tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_REDUCED!();
                deta = -1;
            }
            while (val != tar).as_bool() {
                BzpWriteToArray(saveVal.cast(), BZP_BITS2!(), outData.cast());
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT!(), outData.cast());
        });
    });
}

pub fn BzpWriteInputEncode(
    mut outData: Ptr<BzpOutComdata>,
    mut mtf: Ptr<BzpMtfInfo>,
    mut huffman: Ptr<BzpHuffmanGroups>,
) {
    c_for!(let mut i: i32 = 0; i < mtf.nMtf; i.suffix_plus_plus(); {
        let mut val: i32 = mtf.mtfV[i].cast();
        let mut gid: i32 = huffman.select[i / BZP_ELEMS_NUM_IN_ONE_GROUP!()].cast();
        let mut code: i32 = huffman.huffmanGroups[gid].table[val].cast();
        let mut len: i32 = huffman.huffmanGroups[gid].len[val].cast();
        BzpWriteToArray(code.cast(), len.cast(), outData.cast());
    });
}

pub fn BzpWriteFileEnd(mut outData: Ptr<BzpOutComdata>, mut combinedCRC: i32) {
    BzpWriteToArray(BZP_FILE_END_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(combinedCRC.cast(), outData.cast());
}

pub fn BzpFlushbuf(mut outData: Ptr<BzpOutComdata>) {
    while (outData.nBuf > 0) {
        let tmp0 = outData.num;
        outData.out[tmp0] = (outData.buf >> BZP_BITS24!()).cast::<u8>();
        outData.num += 1;
        outData.nBuf -= BZP_BITS8!();
        outData.buf <<= BZP_BITS8!();
    }
}

pub fn BzpCompressOneBlock(
    mut bzpInfo: Ptr<BzpAlgorithmInfo>,
    mut outData: Ptr<BzpOutComdata>,
) -> i32 {
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();
    let mut mtf: Ptr<BzpMtfInfo> = bzpInfo.mtf.cast();
    let mut huffman: Ptr<BzpHuffmanGroups> = bzpInfo.huffman.cast();
    let mut ret: i32 = BZP_OK!();
    if (bwt.nBlock == 0).as_bool() {
        return BZP_OK!();
    }
    BzpWriteFileHead(outData.cast(), bwt.blockId.cast());
    if (bwt.nBlock > 0).as_bool() {
        BzpCalculateCRC(bwt.cast());
        BzpBlockSortMain(bwt.cast());
        BzpMtfReSet(mtf.cast());
        mtf.block = bwt.block.cast();
        mtf.map = bwt.sortBlock.cast();
        mtf.inUse = bwt.inUse.cast();
        mtf.nBlock = bwt.nBlock.cast();
        BzpMtfMain(mtf.cast());
        ret = BzpHuffmanGroupsReset(huffman.cast(), (mtf.nUse + BZP_EXTRA_CHARS_NUM!()).cast())
            .cast();
        if (ret != BZP_OK!()).as_bool() {
            return ret;
        }
        huffman.block = mtf.mtfV.cast();
        huffman.mtfFreq = mtf.mtfFreq.cast();
        huffman.nBlock = mtf.nMtf.cast();
        BzpHuffmanMain(huffman.cast());
        BzpWriteBlockHead(outData.cast(), bwt.cast());
        BzpWriteValidASCII(outData.cast(), bwt.cast());
        BzpWriteToArray(huffman.nGroups.cast(), BZP_BITS3!(), outData.cast());
        BzpWriteSelect(outData.cast(), huffman.cast());
        BzpWriteLen(outData.cast(), huffman.cast());
        BzpWriteInputEncode(outData.cast(), mtf.cast(), huffman.cast());
    }
    return BZP_OK!();
}

pub fn BzpBuffToStream(mut bzpf: Ptr<BzpFile>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos: i32 = 0;
    while (pos < outData.num) {
        bzpf.output.nBuf = 0;
        while (pos < outData.num) && (bzpf.output.nBuf < BZP_BUF_SIZE!()) {
            let tmp0 = bzpf.output.nBuf;
            bzpf.output.buf[tmp0] = outData.out[pos];
            bzpf.output.nBuf += 1;
            pos += 1;
        }
        let mut n2: i32 = c_fwrite!(
            bzpf.output.buf.cast::<Ptr<Void>>(),
            c_sizeof!(u8),
            bzpf.output.nBuf,
            bzpf.output.filePtr
        );
        if (n2 != bzpf.output.nBuf) {
            return BZP_ERROR_IO!();
        }
    }
    return BZP_OK!();
}

pub fn BzpAddCharToBlock(mut lasch: u8, mut num: i32, mut bwt: Ptr<BzpBwtInfo>) {
    if (num < BZP_RLC_NUM_LOWER_LIMIT!()) || (num > BZP_RLC_NUM_UPPER_LIMIT!()) {
        return;
    }
    c_for!(let mut i: i32 = 0; i < num; i.suffix_plus_plus(); {
        BZP_UPDATE_CRC!(bwt.blockCRC, lasch);
    });
    let mut val: i32 = BZP_MIN_FUN!(num, BZP_RLC_NUM_4!());
    c_switch!(val, {
        BZP_RLC_NUM_4!() => {
            let tmp0 = bwt.nBlock.suffix_plus_plus();
            bwt.block[tmp0] = lasch;
            break;
        },
        BZP_RLC_NUM_3!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
            break;
        },
        BZP_RLC_NUM_2!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
            break;
        },
        BZP_RLC_NUM_1!() => {
            bwt.block[bwt.nBlock.suffix_plus_plus()] = lasch;
            break;
        },
        _ => {
            break;
        },
    });
    if (num >= BZP_RLC_NUM_4!()) {
        bwt.block[bwt.nBlock.suffix_plus_plus()] = (num - BZP_RLC_NUM_4!()).cast::<u8>();
        bwt.inUse[num - BZP_RLC_NUM_4!()] = true;
    }
    bwt.inUse[lasch] = true;
}

pub fn BzpBuffToBlockRLC(mut bzpf: Ptr<BzpFile>, mut bwt: Ptr<BzpBwtInfo>, mut IsLastdata: bool) {
    while (!BZP_BLOCK_FULL!(bwt).as_bool() && (!BZP_BUFF_READ_EMPTY!(bzpf)).as_bool()) {
        let mut pos: i32 = bzpf.input.pos.cast();
        let mut ch: u8 = bzpf.input.buf[pos].cast::<u8>();
        let mut lasch: u8 = bzpf.lasChar.cast::<u8>();
        if (ch != lasch).as_bool() || (bzpf.num == BZP_RLC_NUM_UPPER_LIMIT!()).as_bool() {
            BzpAddCharToBlock(lasch.cast(), bzpf.num.cast(), bwt.cast());
            bzpf.lasChar = ch.cast();
            bzpf.num = 1;
        } else {
            bzpf.num += 1;
        }
        bzpf.input.pos += 1;
    }
    if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
        BzpAddCharToBlock(bzpf.lasChar.cast(), bzpf.num.cast(), bwt.cast());
        bzpf.lasChar = BZP_ASCII_SIZE!();
        bzpf.num = 0;
    }
}

pub fn BzpResetCompress(mut bwt: Ptr<BzpBwtInfo>, mut outData: Ptr<BzpOutComdata>) {
    outData.num = 0;
    bwt.nBlock = 0;
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    c_memset_s!(
        bwt.inUse,
        c_sizeofval!(bwt.inUse),
        0,
        c_sizeofval!(bwt.inUse)
    )
    .cast::<Void>();
    let mut n: i32 = outData.blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32);
    c_memset_s!(bwt.isStartPos, n, 0, n).cast::<Void>();
    bwt.blockId += 1;
}

pub fn BzpProcessData(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut IsLastdata: bool) -> i32 {
    let mut bzpf: Ptr<BzpFile> = bzpInfo.compressFile.cast();
    let mut outData: Ptr<BzpOutComdata> = bzpInfo.outData.cast();
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();
    bzpf.state = BZP_INPUT_COMPRESS!();
    let mut ret: i32 = BZP_OK!();
    while (bzpf.state != BZP_RETUEN_COMPRESS!()).as_bool() {
        if (bzpf.state == BZP_OUTPUT_COMPRESS!()).as_bool() {
            ret = BzpBuffToStream(bzpf.cast(), outData.cast()).cast();
            BzpResetCompress(bwt.cast(), outData.cast());
            bzpf.state = BZP_INPUT_COMPRESS!();
            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (bzpf.state == BZP_INPUT_COMPRESS!()).as_bool() {
            BzpBuffToBlockRLC(bzpf.cast(), bwt.cast(), IsLastdata.cast());
            if IsLastdata.as_bool() && BZP_BUFF_READ_EMPTY!(bzpf).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();
                BzpWriteFileEnd(outData.cast(), bwt.combinedCRC.cast());
                BzpFlushbuf(outData.cast());
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else if BZP_BLOCK_FULL!(bwt).as_bool() {
                ret = BzpCompressOneBlock(bzpInfo.cast(), outData.cast()).cast();
                bzpf.state = BZP_OUTPUT_COMPRESS!();
            } else {
                bzpf.state = BZP_RETUEN_COMPRESS!();
            }
        }
        if (ret != BZP_OK!()).as_bool() {
            return ret.cast();
        }
    }
    return ret.cast();
}

pub fn BzpCompressEnd(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo.compressFile.input.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.input.filePtr);
    }
    if (bzpInfo.compressFile.output.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.output.filePtr);
    }
    BzpAlgorithmInfoFinish(bzpInfo.cast());
}

pub fn BzpCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>, mut blockSize: i32) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut IsLastdata: bool = false;
    if (inName == NULL!()) || (outName == NULL!()) || BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return BZP_ERROR_PARAM!();
    }
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = BzpAlgorithmInfoInit(blockSize);
    if (bzpInfo == NULL!()) {
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    ret = BzpOpenFile(bzpInfo, inName, outName);
    if (ret != BZP_OK!()) {
        return ret;
    }
    let mut inStream: Ptr<BzpStream> = bzpInfo.compressFile.input;
    while !IsLastdata {
        inStream.nBuf = c_fread!(
            inStream.buf,
            c_sizeof!(char),
            c_sizeofval!(inStream.buf),
            inStream.filePtr
        );
        inStream.pos = 0;
        IsLastdata = BzpFileEOF(inStream.filePtr);
        ret = BzpProcessData(bzpInfo, IsLastdata);
        if (ret != BZP_OK!()) {
            break;
        }
    }
    BzpCompressEnd(bzpInfo);
    if (ret != BZP_OK!()) {
        c_remove!(outName);
    }
    return ret;
}

pub fn BzpBlockSortInit(mut blockSize: i32) -> Ptr<BzpBwtInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtInfo> = c_malloc!(c_sizeof!(BzpBwtInfo));
    if (bwt == NULL!()).as_bool() {
        return NULL!();
    }
    c_memset_s!(bwt, c_sizeof!(BzpBwtInfo), 0, c_sizeof!(BzpBwtInfo)).cast::<Void>();
    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!();
    bwt.nBlockMax = (spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE!()).cast();
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sortBlock = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.idx = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.isStartPos = c_malloc!(spaceSize * c_sizeof!(i32));
    if (bwt.block == NULL!()).as_bool()
        || (bwt.sortBlock == NULL!()).as_bool()
        || (bwt.idx == NULL!()).as_bool()
        || (bwt.isStartPos == NULL!()).as_bool()
    {
        BzpBwtFinish(bwt.cast());
        return NULL!();
    }
    c_memset_s!(
        bwt.isStartPos,
        spaceSize * c_sizeof!(i32),
        0,
        spaceSize * c_sizeof!(i32)
    )
    .cast::<Void>();
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    return bwt.cast();
}

pub fn BzpShellSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut increments: Array<i32, 2> =
        arr![BZP_SHELL_SORT_INCREMENT1!(), BZP_SHELL_SORT_INCREMENT0!()];
    let mut i: i32 = Default::default();
    let mut j: i32 = Default::default();
    if (l >= r).as_bool() {
        return;
    }
    c_for!(let mut id: i32 = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS!(); id.suffix_plus_plus(); {
        let mut H: i32 = increments[id];
        if (r - l + 1 <= H).as_bool() {
            continue;
        }
        c_for!(i = l + H; i <= r; i.suffix_plus_plus(); {
            let mut tmpIdx: i32 = sortBlock[i];
            let mut tmpVal: i32 = idx[tmpIdx];
            c_for!(j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H; {
                sortBlock[j + H] = sortBlock[j];
            });
            sortBlock[j + H] = tmpIdx;
        });
    });
}

pub fn BzpSwap2Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = value.cast();
}

pub fn BzpSwap3Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut ePos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = sortBlock[ePos].cast();
    sortBlock[ePos] = value.cast();
}

pub fn BzpSelectMidVal(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) -> i32 {
    let mut mid: i32 = (l + r) >> 1;
    let mut vl: i32 = idx[sortBlock[l]].cast();
    let mut vmid: i32 = idx[sortBlock[mid]].cast();
    let mut vr: i32 = idx[sortBlock[r]].cast();
    if (vl > vr).as_bool() {
        let mut tmp: i32 = l.cast();
        l = r.cast();
        r = tmp.cast();
        vl = idx[sortBlock[l]].cast();
        vr = idx[sortBlock[r]].cast();
    }
    if (vmid <= vl).as_bool() {
        return vl.cast();
    } else if (vmid <= vr).as_bool() {
        return vmid.cast();
    } else {
        return vr.cast();
    }
}

pub fn BzpQSortSingle(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut stack: Ptr<BzpQSortInfo>) {
    let mut tl: i32 = stack.tl;
    let mut tr: i32 = stack.tr;
    let mut value: i32 = BzpSelectMidVal(sortBlock, idx, tl, tr);
    let mut lPos: i32 = tl;
    let mut rPos: i32 = tr;
    let mut ePos: i32 = tl;
    while (ePos <= rPos) {
        if (idx[sortBlock[ePos]] < value) {
            BzpSwap2Elem(sortBlock, ePos, lPos);
            ePos += 1;
            lPos += 1;
        } else if (idx[sortBlock[ePos]] == value) {
            ePos += 1;
        } else {
            while (rPos >= ePos) && (idx[sortBlock[rPos]] > value) {
                rPos -= 1;
            }
            if (rPos < ePos) {
                break;
            }
            if (idx[sortBlock[rPos]] == value) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
            } else if (lPos == ePos) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
                lPos += 1;
            } else {
                BzpSwap3Elem(sortBlock, lPos, ePos, rPos);
                lPos += 1;
            }
            ePos += 1;
            rPos -= 1;
        }
    }
    if (lPos - tl > tr - rPos) {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
    } else {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
    }
}

pub fn BzpQuickSort(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) {
    let mut stack: BzpQSortInfo = Default::default();
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l.cast();
    stack.stackR[stack.cnt] = r.cast();
    stack.cnt += 1;
    while (stack.cnt > 0).as_bool() {
        stack.cnt -= 1;
        let mut tl: i32 = stack.stackL[stack.cnt].cast();
        let mut tr: i32 = stack.stackR[stack.cnt].cast();
        if (tl >= tr).as_bool() {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT!()).as_bool() {
            BzpShellSort(sortBlock.cast(), idx.cast(), tl.cast(), tr.cast());
            continue;
        }
        stack.tl = tl.cast();
        stack.tr = tr.cast();
        BzpQSortSingle(sortBlock.cast(), idx.cast(), c_ref!(stack).cast());
    }
}

pub fn BzpUpdateflag(mut bwt: Ptr<BzpBwtInfo>, mut l: i32, mut r: i32) {
    let mut tmpst: i32 = -1;
    c_for!(let mut i: i32 = l; i <= r; i.suffix_plus_plus(); {
        let mut tmpnow: i32 = bwt.idx[bwt.sortBlock[i]];
        if (tmpst != tmpnow).as_bool() {
            bwt.isStartPos[i] = 1;
            tmpst = tmpnow;
        }
    });
}

pub fn BzpBinaryLiftingSort(mut bwt: Ptr<BzpBwtInfo>) {
    let mut ftab: Array<i32, { BZP_ASCII_SIZE!() }> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i]] += 1;
    });
    c_for!(let mut i: i32 = 1; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: i32 = bwt.block[i].cast();
        ftab[ch] -= 1;
        bwt.sortBlock[ftab[ch]] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        bwt.isStartPos[ftab[i]] = 1;
    });
    let mut M: i32 = 1;
    let mut sortflag: bool = true;
    while (M < bwt.nBlock).as_bool() && sortflag.as_bool() {
        let mut st: i32 = 0;
        sortflag = false;
        c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
            if bwt.isStartPos[i].as_bool() {
                st = i.cast();
            }
            let mut pos: i32 = bwt.sortBlock[i] - M;
            if (pos < 0).as_bool() {
                pos += bwt.nBlock;
            }
            bwt.idx[pos] = st.cast();
        });
        let mut l: i32 = 0;
        let mut r: i32 = 1;
        while (l < bwt.nBlock).as_bool() {
            while (r < bwt.nBlock).as_bool() && (!bwt.isStartPos[r].as_bool()) {
                r += 1;
            }
            r -= 1;
            if (l < r).as_bool() {
                sortflag = true;
                BzpQuickSort(bwt.sortBlock.cast(), bwt.idx.cast(), l.cast(), r.cast());
                BzpUpdateflag(bwt.cast(), l.cast(), r.cast());
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}

pub fn BzpBlockSortMain(mut bwt: Ptr<BzpBwtInfo>) {
    BzpBinaryLiftingSort(bwt.cast());
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        if (bwt.sortBlock[i] == 0).as_bool() {
            bwt.oriPtr = i.cast();
            break;
        }
    });
}

pub fn BzpBwtFinish(mut bwt: Ptr<BzpBwtInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.sortBlock != NULL!()).as_bool() {
            c_free!(bwt.sortBlock);
            bwt.sortBlock = NULL!();
        }
        if (bwt.idx != NULL!()).as_bool() {
            c_free!(bwt.idx);
            bwt.idx = NULL!();
        }
        if (bwt.isStartPos != NULL!()).as_bool() {
            c_free!(bwt.isStartPos);
            bwt.isStartPos = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}

pub fn BzpMtfInit(mut blockSize: i32) -> Ptr<BzpMtfInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut mtf: Ptr<BzpMtfInfo> = c_malloc!(c_sizeof!(BzpMtfInfo));
    if (mtf == NULL!()).as_bool() {
        return NULL!();
    }
    mtf.mtfV = NULL!();
    mtf.mtfV = c_malloc!(blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32));
    if (mtf.mtfV == NULL!()).as_bool() {
        c_free!(mtf);
        mtf = NULL!();
        return NULL!();
    }
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
    return mtf.cast();
}

pub fn BzpMtfReSet(mut mtf: Ptr<BzpMtfInfo>) {
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
}

pub fn BzpMapInputChar(mut mtf: Ptr<BzpMtfInfo>, mut list: Ptr<u8>, mut lenList: i32) {
    if (BZP_ASCII_SIZE!() > lenList).as_bool() {
        return;
    }
    c_for!(let mut i: i32 = 0; i < BZP_ASCII_SIZE!().cast(); i.suffix_plus_plus(); {
        if mtf.inUse[i].as_bool() {
            list[mtf.nUse] = i.cast::<u8>();
            mtf.nUse += 1;
        }
    });
}

pub fn BzpNumEncode(mut mtf: Ptr<BzpMtfInfo>, mut num: i32) {
    num <<= 1;
    c_do!({
        num >>= 1;
        num -= 1;
        if (num & 1) {
            let tmp0 = mtf.nMtf.suffix_plus_plus();
            mtf.mtfV[tmp0] = BZP_MTF_ENCODE1!();
            mtf.mtfFreq[BZP_MTF_ENCODE1!()] += 1;
        } else {
            mtf.mtfV[mtf.nMtf.suffix_plus_plus()] = BZP_MTF_ENCODE0!();
            mtf.mtfFreq[BZP_MTF_ENCODE0!()] += 1;
        }
    } while num >= BZP_MTF_ENCODE_BASE!());
}

pub fn BzpMtfMain(mut mtf: Ptr<BzpMtfInfo>) {
    let mut list: Array<u8, { BZP_MAX_ALPHA_SIZE!() }> = Default::default();
    let mut EOB: i32 = Default::default();
    let mut num: i32 = 0;
    BzpMapInputChar(mtf, list.as_mut_ptr(), BZP_MAX_ALPHA_SIZE!());
    EOB = (mtf.nUse + 1);
    c_for!(let mut i: i32 = 0; i <= EOB; i.suffix_plus_plus(); {
        mtf.mtfFreq[i] = 0;
    });
    c_for!(let mut i: i32 = 0; i < mtf.nBlock; i.suffix_plus_plus(); {
        let mut pos: i32 = (mtf.map[i] - 1);
        if (pos < 0) {
            pos += mtf.nBlock;
        }
        let mut ch: u8 = mtf.block[pos];
        if (ch == list[0]) {
            num += 1;
        } else {
            if (num > 0) {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            let mut pos_: i32 = 1;
            while (ch != list[pos_]) && (pos_ < mtf.nUse) {
                pos_ += 1;
            }
            c_for!(let mut j: i32 = pos_; j > 0; j.suffix_minus_minus(); {
                list[j] = list[j - 1];
            });
            list[0] = ch;
            let tmp0 = mtf.nMtf;
            mtf.mtfV[tmp0] = (pos_ + 1);
            mtf.mtfFreq[pos_ + 1] += 1;
            mtf.nMtf += 1;
        }
    });
    if (num > 0) {
        BzpNumEncode(mtf, num);
    }
    mtf.mtfV[mtf.nMtf] = EOB;
    mtf.mtfFreq[EOB] += 1;
    mtf.nMtf += 1;
}

pub fn BzpMtfFinish(mut mtf: Ptr<BzpMtfInfo>) {
    if (mtf != NULL!()).as_bool() {
        if (mtf.mtfV != NULL!()).as_bool() {
            c_free!(mtf.mtfV);
            mtf.mtfV = NULL!();
        }
        c_free!(mtf);
        mtf = NULL!();
    }
}

pub fn BzpHuffmanInit(mut alphaSize: i32, mut huffman: Ptr<BzpHuffmanInfo>) {
    c_memset_s!(
        huffman.len,
        c_sizeofval!(huffman.len),
        0,
        c_sizeofval!(huffman.len)
    )
    .cast::<Void>();
    huffman.nHeap = 0;
    huffman.nWeight = 0;
    huffman.alphaSize = alphaSize;
}

pub fn BzpHuffmanInitArray(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32;
    huffman.nHeap = 0;
    huffman.nWeight = huffman.alphaSize.cast();
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.parent[i] = -1;
    });
}

pub fn BzpHeapAdjustUp(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut pos: i32) {
    let mut tmpw: i32 = weight[heap[pos]].cast();
    let mut tmpv: i32 = heap[pos].cast();
    while (pos > 1).as_bool() {
        if (tmpw < weight[heap[pos >> 1]]).as_bool() {
            heap[pos] = heap[pos >> 1].cast();
            pos >>= 1;
        } else {
            break;
        }
    }
    heap[pos] = tmpv.cast();
}

pub fn BzpHeapAdjustDown(mut heap: Ptr<i32>, mut weight: Ptr<i32>, mut nHeap: i32) {
    let mut pos: i32 = 1;
    let mut chpos: i32 = pos << 1;
    let mut tmpid: i32 = heap[pos];
    let mut tmpv: i32 = weight[tmpid];
    while (chpos <= nHeap).as_bool() {
        if ((chpos | 1) <= nHeap).as_bool()
            && (weight[heap[chpos]] > weight[heap[chpos | 1]]).as_bool()
        {
            chpos |= 1;
        }
        if (tmpv < weight[heap[chpos]]).as_bool() {
            break;
        }
        heap[pos] = heap[chpos].cast();
        pos = chpos.cast();
        chpos = pos << 1;
    }
    heap[pos] = tmpid.cast();
}

pub fn BzpHeapInit(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32 = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.nHeap += 1;
        let tmp0 = huffman.nHeap;
        huffman.heap[tmp0] = i;
        BzpHeapAdjustUp(huffman.heap.as_ptr(), huffman.weight.cast(), huffman.nHeap);
    });
}

pub fn BzpHuffmanWeightAdd(mut w1: i32, mut w2: i32) -> i32 {
    return ((w1 & 0xffffff00) + (w2 & 0xffffff00))
        | (BZP_MAX_FUN!((w1 & 0x000000ff), (w2 & 0x000000ff)) + 1);
}

pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    let mut idx1: i32;
    let mut idx2: i32;
    while (huffman.nHeap > 1) {
        idx1 = huffman.heap[1];
        let tmp0 = 1;
        huffman.heap[tmp0];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap, huffman.weight.cast(), huffman.nHeap);
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        huffman.weight[huffman.nWeight] =
            BzpHuffmanWeightAdd(huffman.weight[idx1], huffman.weight[idx2]);
        huffman.parent[idx1] = huffman.nWeight;
        huffman.parent[idx2] = huffman.nWeight;
        huffman.parent[huffman.nWeight] = -1;
        huffman.nHeap += 1;
        huffman.heap[huffman.nHeap] = huffman.nWeight;
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    }
}

pub fn BzpGetCodeLen(mut huffman: Ptr<BzpHuffmanInfo>) -> i32 {
    let mut maxlen: i32 = 0;
    BzpBuildHuffmanTree(huffman.cast());
    let mut i: i32;
    maxlen = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        let mut x: i32 = i.cast();
        let mut tlen: i32 = 0;
        while (huffman.parent[x] >= 0).as_bool() {
            x = huffman.parent[x].cast();
            tlen += 1;
        }
        huffman.len[i] = tlen.cast();
        maxlen = BZP_MAX_FUN!(maxlen, tlen);
    });
    return maxlen.cast();
}

pub fn BzpBuildTreeBalanceHeight(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut maxlen: i32 = 0;
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        if (huffman.weight[i] == 0).as_bool() {
            huffman.weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        } else {
            huffman.weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        }
    });
    c_do!({
        maxlen = BzpGetCodeLen(huffman.cast()).cast();
        if (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE!()).as_bool() {
            c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
                let mut w: i32 = (huffman.weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!()).cast();
                w = ((w >> 1) + 1).cast();
                huffman.weight[i] = (w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!()).cast();
            });
        }
    } while maxlen > BZP_MAX_TREE_HEIGHT_ENCODE!());
}

pub fn BzpGetHuffmanTable(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut vec: i32 = 0;
    let mut mi: i32 = huffman.len[0].cast();
    let mut mx: i32 = huffman.len[0].cast();
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[i]).cast();
        mx = BZP_MAX_FUN!(mx, huffman.len[i]).cast();
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[j] == i).as_bool() {
                huffman.table[j] = vec.cast();
                vec += 1;
            }
        });
        vec <<= 1;
    });
}

pub fn BzpHuffmanGroupsReset(mut huffman: Ptr<BzpHuffmanGroups>, mut alphaSize: i32) -> i32 {
    if BZP_INVALID_ALPHA_SIZE!(alphaSize).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    huffman.alphaSize = alphaSize;
    huffman.block = NULL!();
    huffman.mtfFreq = NULL!();
    huffman.nSelect = 0;
    huffman.nGroups = 0;
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        BzpHuffmanInit(alphaSize.cast(), c_ref!(huffman.huffmanGroups[i]).cast());
    });
    return BZP_OK!();
}

pub fn BzpHuffmanGroupsInit(mut blockSize: i32) -> Ptr<BzpHuffmanGroups> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffmanGroups: Ptr<BzpHuffmanGroups> = c_malloc!(c_sizeof!(BzpHuffmanGroups));
    if (huffmanGroups == NULL!()).as_bool() {
        return NULL!();
    }
    huffmanGroups.select = NULL!();
    huffmanGroups.selectMTF = NULL!();
    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffmanGroups.select = c_malloc!(spaceSize * c_sizeof!(i32));
    huffmanGroups.selectMTF = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffmanGroups.select == NULL!()).as_bool() || (huffmanGroups.selectMTF == NULL!()).as_bool()
    {
        BzpHuffmanGroupsFinish(huffmanGroups.cast());
        return NULL!();
    }
    huffmanGroups.alphaSize = 0;
    huffmanGroups.block = NULL!();
    huffmanGroups.mtfFreq = NULL!();
    huffmanGroups.nSelect = 0;
    huffmanGroups.nGroups = 0;
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        BzpHuffmanInit(0, c_ref!(huffmanGroups.huffmanGroups[i]).cast());
    });
    return huffmanGroups.cast();
}

pub fn BzpHuffmanGroupsFinish(mut huffman: Ptr<BzpHuffmanGroups>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        if (huffman.selectMTF != NULL!()).as_bool() {
            c_free!(huffman.selectMTF);
            huffman.selectMTF = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}

pub fn BzpGetHuffmanGroups(mut nBlock: i32) -> i32 {
    let mut nGroups: i32 = 1;
    if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT0!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_0!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT1!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_1!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT2!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_2!();
    } else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT3!()).as_bool() {
        nGroups = BZP_NGROUPS_NUM_3!();
    } else {
        nGroups = BZP_NGROUPS_NUM_4!();
    }
    return nGroups.cast();
}

pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let nGroups: i32 = huffman.nGroups;
    let mut list: Vec<i32> = vec![0; nGroups as usize];
    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        list[i as usize] = i;
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = 0;
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            if (huffman.select[i as usize] == list[j as usize]) {
                pos = j;
                break;
            }
        });
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            list[j as usize] = list[(j - 1) as usize];
        });
        let tmp0 = 0;
        list[tmp0 as usize];
        huffman.selectMTF[i as usize] = pos;
    });
}

pub fn BzpInitLenArray(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups.cast();
    let mut npart: i32 = nGroups.cast();
    let mut AllFreqNum: i32 = huffman.nBlock.cast();
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();
    while (npart > 0).as_bool() {
        let mut NowFreqNum: i32 = 0;
        let mut FreqNumLimit: i32 = AllFreqNum / npart;
        ed = st - 1;
        while (ed < huffman.alphaSize - 1).as_bool() && (NowFreqNum < FreqNumLimit).as_bool() {
            ed += 1;
            NowFreqNum += huffman.mtfFreq[ed];
        }
        if (ed > st).as_bool()
            && (npart != nGroups).as_bool()
            && (npart != 1).as_bool()
            && (((nGroups - npart) & 1) != 0).as_bool()
        {
            NowFreqNum -= huffman.mtfFreq[ed];
            ed -= 1;
        }
        c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
            if (i >= st).as_bool() && (i <= ed).as_bool() {
                huffman.huffmanGroups[npart - 1].len[i] = 0;
            } else {
                huffman.huffmanGroups[npart - 1].len[i] = BZP_HUFFMAN_LEN_MAX_COST!();
            }
        });
        npart -= 1;
        st = ed + 1;
        AllFreqNum -= NowFreqNum;
    }
}

pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(
        huffman.cost,
        c_sizeofval!(huffman.cost),
        0,
        c_sizeofval!(huffman.cost)
    )
    .cast::<Void>();
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
        c_for!(let mut t: i32 = 0; t < nGroups; t.suffix_plus_plus(); {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]];
        });
    });
}

pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = 0; k < nGroups; k.suffix_plus_plus(); {
        let tmp0 = k;
        if (huffman.cost[tmp0]) {
            id = k;
        }
    });
    huffman.select[huffman.nSelect] = id;
    huffman.nSelect += 1;
    return id;
}

pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock);
    huffman.nGroups = nGroups;
    BzpInitLenArray(huffman);
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_MAX_ITER_NUM!(); i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight)).cast::<Void>();
        });
        st = 0;
        huffman.nSelect = 0;
        while (st < huffman.nBlock) {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!()) - 1;
            BzpCalculateCost(huffman, st, ed);
            let mut id: i32 = BzpSelectTree(huffman);
            c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
                let block_k = huffman.block[k];
                huffman.huffmanGroups[id].weight[block_k] += 1;
            });
            st = ed + 1;
        }
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]));
        });
    });
    BzpGenerateSelectMTF(huffman);
    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]));
    });
}

pub fn BzpBwtDecodeInit(mut blockSize: i32) -> Ptr<BzpBwtDecodeInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtDecodeInfo> = c_malloc!(c_sizeof!(BzpBwtDecodeInfo));
    if (bwt == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize;
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.deCode = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sorted = c_malloc!(spaceSize * c_sizeof!(i32));
    if (bwt.block == NULL!()).as_bool()
        || (bwt.sorted == NULL!()).as_bool()
        || (bwt.deCode == NULL!()).as_bool()
    {
        BzpBwtDecodeFinish(bwt.cast());
        return NULL!();
    }
    bwt.nBlock = 0;
    bwt.oriPtr = 0;
    return bwt.cast();
}

pub fn BzpBwtDecode(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    let mut ftab: Array<i32, 257> = Default::default();
    c_memset_s!(ftab, c_sizeofval!(ftab), 0, c_sizeofval!(ftab)).cast::<Void>();
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        ftab[bwt.block[i] + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= BZP_ASCII_SIZE!(); i.suffix_plus_plus(); {
        ftab[i] += ftab[i - 1];
    });
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        let mut ch: u8 = bwt.block[i].cast();
        bwt.sorted[ftab[ch]] = i.cast();
        ftab[ch] += 1;
    });
    let mut cnt: i32 = 0;
    let mut pos: i32 = bwt.oriPtr.cast();
    while (cnt < bwt.nBlock).as_bool() {
        pos = bwt.sorted[pos].cast();
        let mut ch: u8 = bwt.block[pos].cast();
        bwt.deCode[cnt] = ch.cast();
        cnt += 1;
    }
}

pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()).as_bool() {
            c_free!(bwt.deCode);
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()).as_bool() {
            c_free!(bwt.sorted);
            bwt.sorted = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}

pub fn BzpHuffmanDecodeInit(mut blockSize: i32) -> Ptr<BzpHuffmanDecode> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = c_malloc!(c_sizeof!(BzpHuffmanDecode));
    if (huffman == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffman.select = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffman.select == NULL!()).as_bool() {
        BzpHuffmanDecodeFinish(huffman.cast());
    }
    c_memset_s!(
        huffman.base,
        c_sizeofval!(huffman.base),
        0,
        c_sizeofval!(huffman.base)
    )
    .cast::<Void>();
    c_memset_s!(
        huffman.perm,
        c_sizeofval!(huffman.perm),
        0,
        c_sizeofval!(huffman.perm)
    )
    .cast::<Void>();
    c_memset_s!(
        huffman.limit,
        c_sizeofval!(huffman.limit),
        0,
        c_sizeofval!(huffman.limit)
    )
    .cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
    return huffman.cast();
}

pub fn BzpHuffmanDecodeReset(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_memset_s!(
        huffman.base,
        c_sizeofval!(huffman.base),
        0,
        c_sizeofval!(huffman.base)
    )
    .cast::<Void>();
    c_memset_s!(
        huffman.perm,
        c_sizeofval!(huffman.perm),
        0,
        c_sizeofval!(huffman.perm)
    )
    .cast::<Void>();
    c_memset_s!(
        huffman.limit,
        c_sizeofval!(huffman.limit),
        0,
        c_sizeofval!(huffman.limit)
    )
    .cast::<Void>();
    huffman.selectCnt = 0;
    huffman.deCodeNum = 0;
}

pub fn BzpGetOneTable(mut huffman: Ptr<BzpHuffmanDecode>, mut t: i32) {
    let mut vec: i32 = 0;
    let mut cnt: i32 = 0;
    let mut mi: i32 = huffman.len[t][0];
    let mut mx: i32 = huffman.len[t][0];
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[t][i]);
        mx = BZP_MAX_FUN!(mx, huffman.len[t][i]);
    });
    huffman.minLens[t] = mi;
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[t][j] == i) {
                huffman.perm[t][cnt] = j;
                cnt += 1;
            }
        });
    });
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        let len = huffman.len[t][i];
        huffman.base[t][len + 1] += 1;
    });
    c_for!(let mut i: i32 = 1; i <= mx + 1; i.suffix_plus_plus(); {
        let tmp0 = t;
        huffman.base[tmp0][i - 1];
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        vec += (huffman.base[t][i + 1] - huffman.base[t][i]);
        huffman.limit[t][i] = (vec - 1);
        vec <<= 1;
    });
    c_for!(let mut i: i32 = mi + 1; i <= mx; i.suffix_plus_plus(); {
        huffman.base[t][i] = (((huffman.limit[t][i - 1] + 1) << 1) - huffman.base[t][i]);
    });
}

pub fn BzpGenerateDecodeTable(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_for!(let mut t: i32 = 0; t < huffman.nGroups; t.suffix_plus_plus(); {
        BzpGetOneTable(huffman.cast(), t.cast());
    });
}

pub fn BzpHuffmanDecodeFinish(mut huffman: Ptr<BzpHuffmanDecode>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}
