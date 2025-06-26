pub fn BzpWriteChar(mut ch: u8, mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inData.output.nBuf >= BZP_BUF_SIZE!()).as_bool() {
        let mut n2: i32 = f_fwrite!(
            inData.output.buf.cast(),
            c_sizeof!(u8).cast(),
            inData.output.nBuf.cast(),
            inData.output.filePtr.cast()
        );
        if (n2 != inData.output.nBuf).as_bool() {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }
    inData.output.buf[inData.output.nBuf.suffix_plus_plus()] = ch;
    return ret.cast();
}