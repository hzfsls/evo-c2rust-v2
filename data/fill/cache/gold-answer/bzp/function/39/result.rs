pub fn BzpFlushbuf(mut outData: Ptr<BzpOutComdata>) {
    loop {
        if outData.nBuf <= 0 {
            break;
        }
        // outData.out[outData.num] = (outData.buf >> BZP_BITS24!()).cast();
        index!(outData.out, outData.num, (outData.buf >> BZP_BITS24!()).cast());
        outData.num += 1;
        outData.nBuf -= BZP_BITS8!();
        outData.buf <<= BZP_BITS8!();
    }
}