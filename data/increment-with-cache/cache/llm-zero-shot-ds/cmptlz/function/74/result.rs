pub unsafe fn CmptLzGetProbsMatrix(decCtx: *mut CmptLzDecCtx) -> *mut CmptLzDecProb {
    (*decCtx).probsPlus1664
}
