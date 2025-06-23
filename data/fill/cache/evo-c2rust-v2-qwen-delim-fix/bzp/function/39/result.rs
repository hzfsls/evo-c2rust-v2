pub fn BzpFlushbuf(mut outData: Ptr<BzpOutComdata>) {
    while (outData.nBuf > 0).as_bool() {
        outData.out[outData.num.suffix_plus_plus()] = (outData.buf >> BZP_BITS24!()).cast::<u8>();
        outData.nBuf -= BZP_BITS8!();
        outData.buf <<= BZP_BITS8!();
    }
}