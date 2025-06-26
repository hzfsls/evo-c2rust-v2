pub fn BzpWriteChar(mut ch: u8, mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inData.output.nBuf >= BZP_BUF_SIZE!()).as_bool() {
        let mut n2: i32 = c_fwrite!(inData.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), inData.output.nBuf, inData.output.filePtr);
        if (n2 != inData.output.nBuf).as_bool() {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }
    inData.output.buf[inData.output.nBuf.suffix_plus_plus()] = ch.cast();
    return ret.cast();
}
