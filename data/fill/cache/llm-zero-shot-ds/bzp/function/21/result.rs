use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

#[repr(C)]
pub struct BzpAlgorithmInfo {
    bwt: *mut BzpBlockSort,
    mtf: *mut BzpMtf,
    huffman: *mut BzpHuffmanGroups,
    outData: *mut BzpOutComData,
    compressFile: *mut BzpFile,
}

extern "C" {
    fn BzpBlockSortInit(blockSize: i32) -> *mut BzpBlockSort;
    fn BzpMtfInit(blockSize: i32) -> *mut BzpMtf;
    fn BzpHuffmanGroupsInit(blockSize: i32) -> *mut BzpHuffmanGroups;
    fn BzpOutComDataInit(blockSize: i32) -> *mut BzpOutComData;
    fn BzpFileInit() -> *mut BzpFile;
    fn BzpAlgorithmInfoFinish(bzpInfo: *mut BzpAlgorithmInfo);
}

pub unsafe fn BzpAlgorithmInfoInit(blockSize: i32) -> *mut BzpAlgorithmInfo {
    let layout = Layout::new::<BzpAlgorithmInfo>();
    let bzpInfo = alloc(layout) as *mut BzpAlgorithmInfo;
    
    if bzpInfo.is_null() {
        return ptr::null_mut();
    }
    
    (*bzpInfo).bwt = BzpBlockSortInit(blockSize);
    (*bzpInfo).mtf = BzpMtfInit(blockSize);
    (*bzpInfo).huffman = BzpHuffmanGroupsInit(blockSize);
    (*bzpInfo).outData = BzpOutComDataInit(blockSize);
    (*bzpInfo).compressFile = BzpFileInit();
    
    if (*bzpInfo).bwt.is_null() || (*bzpInfo).outData.is_null() || (*bzpInfo).compressFile.is_null() || 
       (*bzpInfo).mtf.is_null() || (*bzpInfo).huffman.is_null() {
        BzpAlgorithmInfoFinish(bzpInfo);
        return ptr::null_mut();
    }
    
    bzpInfo
}
