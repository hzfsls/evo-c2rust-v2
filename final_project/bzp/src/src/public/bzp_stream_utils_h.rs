use crate::translation_utils::*;
pub use crate::src::public::bzp_stream_utils_c::BzpStreamFinish;
pub use crate::src::public::bzp_stream_utils_c::BzpStreamInit;
pub use crate::src::public::bzp_stream_utils_c::g_bzpCRC32Table;

#[repr(C)]
#[derive(Default)]
pub struct BzpStream {
    pub filePtr: FilePtr,
    pub nBuf: i32,
    pub pos: i32,
    pub buf: Array<u8, { BZP_BUF_SIZE!() }>,
}


macro_rules! BZP_STREAM_UTILS_H { () => {  } }
pub(crate) use BZP_STREAM_UTILS_H;


macro_rules! BZP_HDR_B { () => { 0x42 } }
pub(crate) use BZP_HDR_B;


macro_rules! BZP_HDR_Z { () => { 0x5a } }
pub(crate) use BZP_HDR_Z;


macro_rules! BZP_HDR_H { () => { 0x68 } }
pub(crate) use BZP_HDR_H;


macro_rules! BZP_HDR_0 { () => { 0x30 } }
pub(crate) use BZP_HDR_0;


macro_rules! BZP_BLOCK_HEAD_0 { () => { 0x31 } }
pub(crate) use BZP_BLOCK_HEAD_0;


macro_rules! BZP_BLOCK_HEAD_1 { () => { 0x41 } }
pub(crate) use BZP_BLOCK_HEAD_1;


macro_rules! BZP_BLOCK_HEAD_2 { () => { 0x59 } }
pub(crate) use BZP_BLOCK_HEAD_2;


macro_rules! BZP_BLOCK_HEAD_3 { () => { 0x26 } }
pub(crate) use BZP_BLOCK_HEAD_3;


macro_rules! BZP_BLOCK_HEAD_4 { () => { 0x53 } }
pub(crate) use BZP_BLOCK_HEAD_4;


macro_rules! BZP_BLOCK_HEAD_5 { () => { 0x59 } }
pub(crate) use BZP_BLOCK_HEAD_5;


macro_rules! BZP_FILE_END_0 { () => { 0x17 } }
pub(crate) use BZP_FILE_END_0;


macro_rules! BZP_FILE_END_1 { () => { 0x72 } }
pub(crate) use BZP_FILE_END_1;


macro_rules! BZP_FILE_END_2 { () => { 0x45 } }
pub(crate) use BZP_FILE_END_2;


macro_rules! BZP_FILE_END_3 { () => { 0x38 } }
pub(crate) use BZP_FILE_END_3;


macro_rules! BZP_FILE_END_4 { () => { 0x50 } }
pub(crate) use BZP_FILE_END_4;


macro_rules! BZP_FILE_END_5 { () => { 0x90 } }
pub(crate) use BZP_FILE_END_5;


macro_rules! BZP_BUF_SIZE { () => { 5000 } }
pub(crate) use BZP_BUF_SIZE;


macro_rules! BZP_EOF { () => { -1 } }
pub(crate) use BZP_EOF;


macro_rules! BZP_BIT { () => { 1 } }
pub(crate) use BZP_BIT;


macro_rules! BZP_BITS2 { () => { 2 } }
pub(crate) use BZP_BITS2;


macro_rules! BZP_BITS3 { () => { 3 } }
pub(crate) use BZP_BITS3;


macro_rules! BZP_BITS5 { () => { 5 } }
pub(crate) use BZP_BITS5;


macro_rules! BZP_BITS8 { () => { 8 } }
pub(crate) use BZP_BITS8;


macro_rules! BZP_BITS15 { () => { 15 } }
pub(crate) use BZP_BITS15;


macro_rules! BZP_BITS16 { () => { 16 } }
pub(crate) use BZP_BITS16;


macro_rules! BZP_BITS24 { () => { 24 } }
pub(crate) use BZP_BITS24;


macro_rules! BZP_BITS32 { () => { 32 } }
pub(crate) use BZP_BITS32;


macro_rules! BZP_RLC_NUM_1 { () => { 1 } }
pub(crate) use BZP_RLC_NUM_1;


macro_rules! BZP_RLC_NUM_2 { () => { 2 } }
pub(crate) use BZP_RLC_NUM_2;


macro_rules! BZP_RLC_NUM_3 { () => { 3 } }
pub(crate) use BZP_RLC_NUM_3;


macro_rules! BZP_RLC_NUM_4 { () => { 4 } }
pub(crate) use BZP_RLC_NUM_4;


macro_rules! BZP_RLC_NUM_LOWER_LIMIT { () => { 1 } }
pub(crate) use BZP_RLC_NUM_LOWER_LIMIT;


macro_rules! BZP_RLC_NUM_UPPER_LIMIT { () => { 255 } }
pub(crate) use BZP_RLC_NUM_UPPER_LIMIT;


macro_rules! BZP_GROUPS_ASCII { () => { 16 } }
pub(crate) use BZP_GROUPS_ASCII;


macro_rules! BZP_CHARS_PER_GROUP_ASCII { () => { 16 } }
pub(crate) use BZP_CHARS_PER_GROUP_ASCII;


macro_rules! BZP_CRC_MOVE_RIGHT_VAL { () => { 31 } }
pub(crate) use BZP_CRC_MOVE_RIGHT_VAL;


macro_rules! BZP_HUFFMAN_LEN_INCREASE { () => { 2 } }
pub(crate) use BZP_HUFFMAN_LEN_INCREASE;


macro_rules! BZP_HUFFMAN_LEN_REDUCED { () => { 3 } }
pub(crate) use BZP_HUFFMAN_LEN_REDUCED;


macro_rules! BZP_EXTRA_CHARS_NUM { () => { 2 } }
pub(crate) use BZP_EXTRA_CHARS_NUM;


macro_rules! BZP_BLOCK_FULL { ($bwt:expr) => { $bwt.nBlock >= $bwt.nBlockMax } }
pub(crate) use BZP_BLOCK_FULL;


macro_rules! BZP_BUFF_READ_EMPTY { ($bzpf:expr) => { $bzpf.input.pos >= $bzpf.input.nBuf } }
pub(crate) use BZP_BUFF_READ_EMPTY;


macro_rules! BZP_UPDATE_CRC { ($crcVar:expr, $cha:expr) => 
    {
        $crcVar = (($crcVar << 8) ^ g_bzpCRC32Table[(($crcVar >> 24) ^ ($cha as u8)) as usize]);
    }
}
pub(crate) use BZP_UPDATE_CRC;


