pub fn CmptlzEncode(mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut src: Ptr<u8>, mut srcLen: usize, mut props: Ptr<CmptlzEncParam>,
                    mut propsEncoded: Ptr<u8>, mut propsSize: Ptr<usize>, mut writeEndMark: i32, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    if alloc == NULL!() || alloc.CmptLzAlloc == NULL!() || alloc.CmptLzFree == NULL!() {
        CMPTLZ_LOG!(CMPT_ENC_ERROR_PARAM!(), cstr!("Cmptlz input wrong param!"));
        return CMPT_ENC_ERROR_PARAM!();
    }
    let mut encCtx: Ptr<CmptLzEncCtx> = CmptInitCctx(alloc, writeEndMark).cast();
    if encCtx == NULL!() {
        CMPTLZ_LOG!(CMPT_ENC_CTX_INIT_FAIL!(), cstr!("CmptInitCctx Fail!"));
        return CMPT_ENC_CTX_INIT_FAIL!();
    }
    CmptlzSetParam(encCtx, props);
    res = CmptHeadWrite(encCtx, propsEncoded, propsSize);
    if res != 0 {
        (alloc.CmptLzFree)(CMPTLZ_ENC_CCTX_HANDLE!(), encCtx.cast());
        CMPTLZ_LOG!(res, cstr!("CmptHeadWrite Fail!"));
        return res;
    }
    res = CmptlzEncodeIO(encCtx, dest, destLen, src, srcLen, alloc);
    if res != 0 {
        CMPTLZ_LOG!(res, cstr!("CmptlzEncode I / O Fail!"));
    }
    return res;
}