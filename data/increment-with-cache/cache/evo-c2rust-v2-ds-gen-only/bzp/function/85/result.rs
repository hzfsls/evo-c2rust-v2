pub fn BzpBuffToStream(mut bzpf: Ptr<BzpFile>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    bzpf.output.pos = 0;
    let mut pos: i32 = 0;
    while (pos < outData.num).as_bool() {
        bzpf.output.nBuf = 0;
        while (pos < outData.num).as_bool() && (bzpf.output.nBuf < BZP_BUF_SIZE!()).as_bool() {
            bzpf.output.buf[bzpf.output.nBuf] = outData.out[pos].cast();
            bzpf.output.nBuf += 1;
            pos += 1;
        }
        let mut n2: i32 = c_fwrite!(bzpf.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), bzpf.output.nBuf, bzpf.output.filePtr);
        if (n2 != bzpf.output.nBuf).as_bool() {
            return BZP_ERROR_IO!();
        }
    }
    return BZP_OK!();
}
