pub fn BzpWriteToArray(mut val: i32, mut n: i32, mut data: Ptr<BzpOutComdata>) {
    while (data.nBuf >= BZP_BITS8!()) {
        let tmp0 = data.num;
        data.out[tmp0] = (data.buf >> BZP_BITS24!()).cast::<u8>();
        data.num += 1;
        data.nBuf -= BZP_BITS8!();
        data.buf <<= BZP_BITS8!();
    }
    data.buf |= (val << (BZP_BITS32!() - n - data.nBuf)).cast::<u32>();
    data.nBuf += n;
}
