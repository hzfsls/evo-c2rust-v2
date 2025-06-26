use std::ptr::copy_nonoverlapping;

#[repr(C)]
pub struct CmptLzDecCtx {
    dict: *mut u8,
    dictBufSize: usize,
    dictPos: usize,
}

#[repr(C)]
pub struct CmptLzDecIn {
    pSrcIn: *const u8,
    strInLen: usize,
    strInCostLen: usize,
}

#[repr(C)]
pub struct CmptLzDecOut {
    pDestOut: *mut u8,
    destOutLen: usize,
    destOutFillLen: usize,
}

#[repr(C)]
pub enum EnCmptLzFinMode {
    CMPTLZ_FINISH_ANY,
    // Add other variants as needed
}

#[repr(C)]
pub enum EnCmptLzStatus {
    // Define status variants as needed
}

pub const CMPT_OK: i32 = 0;
pub const CMPT_ERROR_MEM: i32 = -1;

extern "C" {
    fn CmptLzDecDecodeToDic(
        decCtx: *mut CmptLzDecCtx,
        dictPosLimit: usize,
        pSrcIn: *const u8,
        srcCostSize: *mut usize,
        finMode: EnCmptLzFinMode,
        finStatus: *mut EnCmptLzStatus,
    ) -> i32;
}

pub unsafe fn CmptLzDecDecodeToBuf(
    decCtx: *mut CmptLzDecCtx,
    pDecIn: *mut CmptLzDecIn,
    pDecOut: *mut CmptLzDecOut,
    finMode: EnCmptLzFinMode,
    finStatus: *mut EnCmptLzStatus,
) -> i32 {
    let mut res = CMPT_OK;
    let mut dictPos;
    let mut dictPosLimit;
    let mut srcCostSize;

    let mut leftSrcSize = (*pDecIn).strInLen;
    let mut leftDestSize = (*pDecOut).destOutLen;
    let mut pSrcIn = (*pDecIn).pSrcIn;
    let mut pDestOut = (*pDecOut).pDestOut;
    let mut tmpFinMode;

    loop {
        if (*decCtx).dictPos == (*decCtx).dictBufSize {
            (*decCtx).dictPos = 0;
        }
        dictPos = (*decCtx).dictPos;
        if leftDestSize > (*decCtx).dictBufSize - dictPos {
            dictPosLimit = (*decCtx).dictBufSize;
            tmpFinMode = EnCmptLzFinMode::CMPTLZ_FINISH_ANY;
        } else {
            dictPosLimit = dictPos + leftDestSize;
            tmpFinMode = finMode;
        }
        srcCostSize = leftSrcSize;
        res = CmptLzDecDecodeToDic(
            decCtx,
            dictPosLimit,
            pSrcIn,
            &mut srcCostSize,
            tmpFinMode,
            finStatus,
        );
        if res != CMPT_OK {
            break;
        }
        pSrcIn = pSrcIn.add(srcCostSize);
        leftSrcSize -= srcCostSize;
        dictPosLimit = (*decCtx).dictPos - dictPos;
        leftDestSize -= dictPosLimit;
        if dictPosLimit == 0 {
            break;
        }
        copy_nonoverlapping(
            (*decCtx).dict.add(dictPos),
            pDestOut,
            dictPosLimit,
        );
        pDestOut = pDestOut.add(dictPosLimit);
        if leftDestSize == 0 {
            break;
        }
    }

    (*pDecIn).strInCostLen = (*pDecIn).strInLen - leftSrcSize;
    (*pDecOut).destOutFillLen = (*pDecOut).destOutLen - leftDestSize;

    res
}
