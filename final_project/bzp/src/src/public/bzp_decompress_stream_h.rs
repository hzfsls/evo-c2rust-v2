use crate::translation_utils::*;
pub use crate::src::decompress::bzp_bwt_decode_h::*;
pub use crate::src::decompress::bzp_huffman_decode_h::*;
pub use crate::src::public::bzp_stream_utils_h::*;
pub use crate::include::bzp_type_h::*;
pub use crate::src::public::bzp_decompress_stream_c::BzpGetDictionaryList;
pub use crate::src::public::bzp_decompress_stream_c::BzpInDeComdataInit;
pub use crate::src::public::bzp_decompress_stream_c::BZPDeCompressData;
pub use crate::src::public::bzp_decompress_stream_c::BzpReadBits;
pub use crate::src::public::bzp_decompress_stream_c::BzpReadUInt32;
pub use crate::src::public::bzp_decompress_stream_c::BzpHuffmanDecodeStep;
pub use crate::src::public::bzp_decompress_stream_c::BzpCheckFileHead;
pub use crate::src::public::bzp_decompress_stream_c::BzpMTFDeCode;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeCompressOneBlock;
pub use crate::src::public::bzp_decompress_stream_c::BzpReadUInt24;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeCompressStream;
pub use crate::src::public::bzp_decompress_stream_c::BzpWriteChar;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeHuffmanSelect;
pub use crate::src::public::bzp_decompress_stream_c::BzpInDeComdataFinish;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeCodeToStream;
pub use crate::src::public::bzp_decompress_stream_c::BZPReadFileEnd;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeHuffmanLen;
pub use crate::src::public::bzp_decompress_stream_c::BzpDeComStreamFinish;

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


macro_rules! BZP_DECOM_STREAM_H { () => { } }
pub(crate) use BZP_DECOM_STREAM_H;


