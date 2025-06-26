pub fn BzpBuffToStream(mut bzpf: Ptr<BzpFile>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos: i32 = 0;
    while pos < outData.num {
        bzpf.output.nBuf = 0;
        while pos < outData.num && bzpf.output.nBuf < BZP_BUF_SIZE!() {
            // bzpf.output.buf[bzpf.output.nBuf] = outData.out[pos];
            index!(bzpf.output.buf, bzpf.output.nBuf, outData.out[pos]);
            bzpf.output.nBuf += 1;
            pos += 1;
        }
        let mut n2: usize = c_fwrite!(bzpf.output.buf, c_sizeof!(u8), bzpf.output.nBuf as usize, bzpf.output.filePtr);
        if n2 != bzpf.output.nBuf as usize {
            return BZP_ERROR_IO!();
        }
    }
    return BZP_OK!();
}