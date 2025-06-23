pub fn BzpReadBits(mut nBit: i32, mut inData: Ptr<InDeComdata>) -> u32 {
    let mut res: u32 = 0;
    while inData.nBuf < nBit {
        if inData.input.nBuf == inData.input.pos {
            inData.input.nBuf = c_fread!(inData.input.buf, c_sizeof!(char), c_sizeofval!(inData.input.buf), inData.input.filePtr).cast();
            inData.input.pos = 0;
        }
        let mut data: i32 = inData.input.buf[inData.input.pos].cast::<u32>().cast();
        inData.buf = (inData.buf << BZP_BITS8!()) | data.cast::<u32>();
        inData.input.pos += 1;
        inData.nBuf += BZP_BITS8!();
    }
    res = (inData.buf >> (inData.nBuf - nBit)).cast();
    res = (res & ((1 << nBit) - 1)).cast();
    inData.nBuf -= nBit;
    return res.cast();
}
