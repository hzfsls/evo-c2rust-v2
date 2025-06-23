pub fn CmptMfPrepare(mut encCtx: Ptr<CmptLzEncCtx>, mut src: Ptr<u8>, mut srcLen: usize, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut mf: Ptr<CmptMfCtx> = alloc.CmptLzAlloc(CMPTLZ_MF_CCTX_HANDLE.cast(), c_sizeof!(CmptMfCtx).cast()).cast();
    if (mf == NULL!()).as_bool() {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    c_memset_s!(mf.cast(), c_sizeof!(CmptMfCtx).cast(), 0, c_sizeof!(CmptMfCtx).cast()).cast::<Void>();
    encCtx.mfCtx = mf.cast();
    mf.cycleSize = (encCtx.dicSize + 1).cast();
    let mut hashMask: u32 = (encCtx.dicSize - 1).cast();
    CMPT_HASH_MASK_CALC!(hashMask);
    mf.hashMask = hashMask;
    hashMask += 1;
    hashMask += CMPTLZ_HASH_2_SIZE!();
    hashMask += CMPTLZ_HASH_3_SIZE!();
    mf.hashCount = hashMask;
    mf.sonsCount = mf.cycleSize * 2;
    mf.hash = NULL!();
    mf.son = NULL!();
    mf.hash = alloc.CmptLzAlloc(CMPTLZ_MF_HASH_HANDLE.cast(), (mf.hashCount * c_sizeof!(u32)).cast()).cast();
    c_memset_s!(mf.hash.cast(), (mf.hashCount * c_sizeof!(u32)).cast(), 0, (mf.hashCount * c_sizeof!(u32)).cast()).cast::<Void>();
    if (mf.hash == NULL!()).as_bool() {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    mf.son = alloc.CmptLzAlloc(CMPTLZ_MF_SON_HANDLE.cast(), (mf.sonsCount * c_sizeof!(u32)).cast()).cast();
    c_memset_s!(mf.son.cast(), (mf.sonsCount * c_sizeof!(u32)).cast(), 0, (mf.sonsCount * c_sizeof!(u32)).cast()).cast::<Void>();
    if (mf.son == NULL!()).as_bool() {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    CmptlzMfGenHashTable(mf.cast());
    mf.srcStart = src.cast();
    mf.srcLen = srcLen.cast();
    mf.offset = mf.cycleSize;
    mf.niceLen = encCtx.numFastBytes;
    mf.depth = (CMPT_MF_BASE_DEPTH!() + mf.niceLen / 2).cast();
    return 0;
}