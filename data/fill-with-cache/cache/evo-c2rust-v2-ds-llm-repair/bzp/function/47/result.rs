pub fn BzpCompressStream(mut inName: Ptr<u8>, mut outName: Ptr<u8>, mut blockSize: i32) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut IsLastdata: bool = false;
    if (inName == NULL!()) || (outName == NULL!()) || BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return BZP_ERROR_PARAM!();
    }
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = BzpAlgorithmInfoInit(blockSize);
    if (bzpInfo == NULL!()) {
        return BZP_ERROR_MEMORY_OPER_FAILURE!();
    }
    ret = BzpOpenFile(bzpInfo, inName, outName);
    if (ret != BZP_OK!()) {
        return ret;
    }
    let mut inStream: Ptr<BzpStream> = bzpInfo.compressFile.input;
    while !IsLastdata {
        inStream.nBuf = c_fread!(inStream.buf, c_sizeof!(char), c_sizeofval!(inStream.buf), inStream.filePtr);
        inStream.pos = 0;
        IsLastdata = BzpFileEOF(inStream.filePtr);
        ret = BzpProcessData(bzpInfo, IsLastdata);
        if (ret != BZP_OK!()) {
            break;
        }
    }
    BzpCompressEnd(bzpInfo);
    if (ret != BZP_OK!()) {
        c_remove!(outName);
    }
    return ret;
}
