pub fn BzpOpenFile(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    if (bzpInfo == NULL!()) {
        return BZP_ERROR_PARAM!();
    }
    bzpInfo.compressFile.input.filePtr = c_fopen!(inName, cstr!("rb"));
    bzpInfo.compressFile.output.filePtr = c_fopen!(outName, cstr!("wb"));
    if (bzpInfo.compressFile.input.filePtr == NULL!()) || (bzpInfo.compressFile.output.filePtr == NULL!()) {
        BzpAlgorithmInfoFinish(bzpInfo);
        c_remove!(outName);
        return BZP_ERROR_IO!();
    }
    return BZP_OK!();
}