pub fn BzpWriteToArray(mut val: i32, mut n: i32, mut data: Ptr<BzpOutComdata>) {
    while data.nBuf >= BZP_BITS8!() {
        index!(data.out, data.num, (data.buf >> BZP_BITS24!()).cast());
        data.num += 1;
        data.nBuf -= BZP_BITS8!();
        data.buf <<= BZP_BITS8!();
    }
    data.buf |= (val << (BZP_BITS32!() - n - data.nBuf)) as u32;
    data.nBuf += n;
}