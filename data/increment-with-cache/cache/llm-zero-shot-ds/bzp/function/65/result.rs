use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

pub struct BzpAlgorithmInfo {
    bwt: *mut BzpBlockSort,
    mtf: *mut BzpMtf,
    huffman: *mut BzpHuffmanGroups,
    outData: *mut BzpOutComData,
    compressFile: *mut BzpFile,
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
