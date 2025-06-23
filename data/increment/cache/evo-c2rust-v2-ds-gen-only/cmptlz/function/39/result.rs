pub fn CmptlzEncode(mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut src: Ptr<u8>, mut srcLen: usize, mut props: Ptr<CmptlzEncParam>, mut propsEncoded: Ptr<u8>, mut propsSize: Ptr<usize>, mut writeEndMark: i32, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    if (alloc == NULL!()).as_bool() || (alloc.CmptLzAlloc == NULL!()).as_bool() || (alloc.CmptLzFree == NULL!()).as_bool() {
        CMPTLZ_LOG!(CMPT_ENC_ERROR_PARAM!(), cstr!("Cmptlz input wrong param!"));
        return CMPT_ENC_ERROR_PARAM!();
    }
    let mut encCtx: Ptr<CmptLzEncCtx> = CmptInitCctx(alloc.cast(), writeEndMark.cast()).cast::<Ptr<CmptLzEncCtx>>();
    if (encCtx == NULL!()).as_bool() {
        CMPTLZ_LOG!(CMPT_ENC_CTX_INIT_FAIL!(), cstr!("CmptInitCctx Fail!"));
        return CMPT_ENC_CTX_INIT_FAIL!();
    }
    CmptlzSetParam(encCtx.cast(), props.cast());
    res = CmptHeadWrite(encCtx.cast(), propsEncoded.cast(), propsSize.cast()).cast();
    if (res != 0).as_bool() {
        alloc.CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE!(), encCtx.cast());
        CMPTLZ_LOG!(res, cstr!("CmptHeadWrite Fail!"));
        return res;
    }
    res = CmptlzEncodeIO(encCtx.cast(), dest.cast(), destLen.cast(), src.cast(), srcLen.cast(), alloc.cast()).cast();
    if (res != 0).as_bool() {
        CMPTLZ_LOG!(res, cstr!("CmptlzEncode I / O Fail!"));
    }
    return res.cast();
}
