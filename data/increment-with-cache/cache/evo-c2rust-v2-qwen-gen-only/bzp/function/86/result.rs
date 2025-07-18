pub fn BzpMtfInit(mut blockSize: i32) -> Ptr<BzpMtfInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut mtf: Ptr<BzpMtfInfo> = c_malloc!(c_sizeof!(BzpMtfInfo));
    if (mtf == NULL!()).as_bool() {
        return NULL!();
    }
    mtf.mtfV = NULL!();
    mtf.mtfV = c_malloc!(blockSize * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(i32));
    if (mtf.mtfV == NULL!()).as_bool() {
        c_free!(mtf.cast());
        mtf = NULL!();
        return NULL!();
    }
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
    return mtf.cast();
}