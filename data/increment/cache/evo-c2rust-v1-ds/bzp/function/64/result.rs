pub fn BzpDeCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if inName == NULL!() || outName == NULL!() {
        return BZP_ERROR_PARAM!();
    }

    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if inStream == NULL!() || outStream == NULL!() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inStream.filePtr = c_fopen!(inName, cstr!("rb"));
    outStream.filePtr = c_fopen!(outName, cstr!("wb"));
    if inStream.filePtr == NULL!() || outStream.filePtr == NULL!() {
        c_free!(inStream);
        inStream = NULL!();
        c_free!(outStream);
        outStream = NULL!();
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    let mut inData: Ptr<InDeComdata> = BzpInDeComdataInit();
    if inData == NULL!() {
        BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
        c_remove!(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inData.input = inStream.cast();
    inData.output = outStream.cast();

    ret = BZPDeCompressData(inData.cast()).cast();

    if inData.output.nBuf > 0 {
        let mut n2: i32 = c_fwrite!(inData.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), inData.output.nBuf, inData.output.filePtr);
        if n2 != inData.output.nBuf {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }

    BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
    if ret != BZP_OK!() {
        c_remove!(outName);
    }
    return ret.cast();
}
