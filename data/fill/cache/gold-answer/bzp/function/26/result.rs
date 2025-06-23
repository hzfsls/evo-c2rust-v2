pub fn BzpOutComDataInit(blockSize: i32) -> Ptr<BzpOutComdata> {
    let mut outData: Ptr<BzpOutComdata> = c_malloc!(c_sizeof!(BzpOutComdata));
    if outData == NULL!() {
        return NULL!();
    }
    outData.out = NULL!();
    outData.out = c_malloc!(blockSize as u32 * BZP_BASE_BLOCK_SIZE!() * c_sizeof!(u32));
    if outData.out == NULL!() {
        c_free!(outData);
        return NULL!();
    }
    outData.nBuf = 0;
    outData.buf = 0;
    outData.num = 0;
    outData.blockSize = blockSize;
    return outData;
}