pub fn BzpDeCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    if (inName == NULL!()).as_bool() || (outName == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }

    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if (inStream == NULL!()).as_bool() || (outStream == NULL!()).as_bool() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inStream.filePtr = c_fopen!(inName, cstr!("rb"));
    outStream.filePtr = c_fopen!(outName, cstr!("wb"));
    if (inStream.filePtr == NULL!()).as_bool() || (outStream.filePtr == NULL!()).as_bool() {
        c_free!(inStream);
        inStream = NULL!();
        c_free!(outStream);
        outStream = NULL!();
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    let mut inData: Ptr<InDeComdata> = BzpInDeComdataInit();
    if (inData == NULL!()).as_bool() {
        BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
        c_remove!(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    inData.input = inStream.cast();
    inData.output = outStream.cast();

    ret = BZPDeCompressData(inData.cast()).cast();

    if (inData.output.nBuf > 0).as_bool() {
        let mut n2: i32 = c_fwrite!(inData.output.buf.cast::<Ptr<Void>>(), c_sizeof!(u8), inData.output.nBuf, inData.output.filePtr);
        if (n2 != inData.output.nBuf).as_bool() {
            ret = BZP_ERROR_IO!();
        }
        inData.output.nBuf = 0;
    }

    BzpDeComStreamFinish(inData.cast(), inStream.cast(), outStream.cast());
    if (ret != BZP_OK!()).as_bool() {
        c_remove!(outName);
    }
    return ret.cast();
}
