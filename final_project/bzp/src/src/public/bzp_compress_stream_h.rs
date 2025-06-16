use crate::translation_utils::*;
pub use crate::src::compress::bzp_bwt_encode_h::*;
pub use crate::src::compress::bzp_huffman_encode_h::*;
pub use crate::src::compress::bzp_mtf_encode_h::*;
pub use crate::src::public::bzp_stream_utils_h::*;
pub use crate::include::bzp_type_h::*;
pub use crate::src::public::bzp_compress_stream_c::BzpOutComDataInit;
pub use crate::src::public::bzp_compress_stream_c::BzpBuffToBlockRLC;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteLen;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteFileHead;
pub use crate::src::public::bzp_compress_stream_c::BzpBuffToStream;
pub use crate::src::public::bzp_compress_stream_c::BzpCompressEnd;
pub use crate::src::public::bzp_compress_stream_c::BzpFileFinish;
pub use crate::src::public::bzp_compress_stream_c::BzpAlgorithmInfoFinish;
pub use crate::src::public::bzp_compress_stream_c::BzpOutComDataFinish;
pub use crate::src::public::bzp_compress_stream_c::BzpAddCharToBlock;
pub use crate::src::public::bzp_compress_stream_c::BzpProcessData;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteInputEncode;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteSelect;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteInt32;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteValidASCII;
pub use crate::src::public::bzp_compress_stream_c::BzpFileInit;
pub use crate::src::public::bzp_compress_stream_c::BzpResetCompress;
pub use crate::src::public::bzp_compress_stream_c::BzpOpenFile;
pub use crate::src::public::bzp_compress_stream_c::BzpFileEOF;
pub use crate::src::public::bzp_compress_stream_c::BzpCompressOneBlock;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteToArray;
pub use crate::src::public::bzp_compress_stream_c::BzpCompressStream;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteBlockHead;
pub use crate::src::public::bzp_compress_stream_c::BzpAlgorithmInfoInit;
pub use crate::src::public::bzp_compress_stream_c::BzpWriteFileEnd;
pub use crate::src::public::bzp_compress_stream_c::BzpFlushbuf;
pub use crate::src::public::bzp_compress_stream_c::BzpCalculateCRC;

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


macro_rules! BZP_COM_STREAM_H { () => {  } }
pub(crate) use BZP_COM_STREAM_H;


macro_rules! BZP_INPUT_COMPRESS { () => { 0 } }
pub(crate) use BZP_INPUT_COMPRESS;


macro_rules! BZP_OUTPUT_COMPRESS { () => { 1 } }
pub(crate) use BZP_OUTPUT_COMPRESS;


macro_rules! BZP_RETUEN_COMPRESS { () => { 2 } }
pub(crate) use BZP_RETUEN_COMPRESS;


