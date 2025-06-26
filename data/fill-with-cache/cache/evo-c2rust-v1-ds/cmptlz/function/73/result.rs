pub fn CmptInitCctx(mut alloc: Ptr<CmptLzMemHook>, mut writeEndMark: i32) -> Ptr<Void> {
    let mut handle: Ptr<Void> = alloc.CmptLzAlloc(CMPTLZ_ENC_CCTX_HANDLE!(), c_sizeof!(CmptLzEncCtx));
    if handle == NULL!() {
        return NULL!();
    }
    c_memset_s!(handle, c_sizeofval!(handle), 0, c_sizeofval!(handle)).cast::<Void>();
    let mut encCtx: Ptr<CmptLzEncCtx> = handle.cast();
    encCtx.endMarker = writeEndMark.cast();
    encCtx.rcCtx = NULL!();
    encCtx.mfCtx = NULL!();
    return encCtx.cast();
}
