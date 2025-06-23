pub fn BzpCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>, mut blockSize: i32) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut IsLastdata: bool = false;
    if (inName == NULL!()).as_bool() || (outName == NULL!()).as_bool() || BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = BzpAlgorithmInfoInit(blockSize.cast());
    if (bzpInfo == NULL!()).as_bool() {
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    ret = BzpOpenFile(bzpInfo.cast(), inName.cast(), outName.cast()).cast();
    if (ret != BZP_OK!()).as_bool() {
        return ret;
    }
    let mut inStream: Ptr<BzpStream> = bzpInfo.compressFile.input.cast();
    while !IsLastdata.as_bool() {
        inStream.nBuf = c_fread!(inStream.buf, c_sizeof!(char), c_sizeofval!(inStream.buf), inStream.filePtr).cast();
        inStream.pos = 0;
        IsLastdata = BzpFileEOF(inStream.filePtr.cast()).cast();
        ret = BzpProcessData(bzpInfo.cast(), IsLastdata.cast()).cast();
        if (ret != BZP_OK!()).as_bool() {
            break;
        }
    }
    BzpCompressEnd(bzpInfo.cast());
    if (ret != BZP_OK!()).as_bool() {
        c_remove!(outName);
    }
    return ret.cast();
}
