static fn CmptLzGetAilgnProb(probsMatrix: *mut CmptLzDecProb) -> *mut CmptLzDecProb {
    unsafe { probsMatrix.offset(CMPTLZ_ALIGN as isize) }
}
