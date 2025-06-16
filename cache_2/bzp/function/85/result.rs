pub fn BzpBuffToStream(mut bzpf: Ptr<BzpFile>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos: i32 = 0;
    while (pos < outData.num) {
        bzpf.output.nBuf = 0;
        while (pos < outData.num) && (bzpf.output.nBuf < BZP_BUF_SIZE!()) {
            let tmp0 = bzpf.output.nBuf;
            bzpf.output.buf[tmp0] = outData.out[pos];
            bzpf.output.nBuf += 1;
            pos += 1;
        }
        let mut n2: i32 = c_fwrite!(bzpf.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), bzpf.output.nBuf, bzpf.output.filePtr);
        if (n2 != bzpf.output.nBuf) {
            return BZP_ERROR_IO!();
        }
    }
    return BZP_OK!();
}
