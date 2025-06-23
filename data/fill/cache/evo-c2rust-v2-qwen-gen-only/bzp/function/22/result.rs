pub fn BzpOpenFile(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut inName: Ptr<u8>, mut outName: Ptr<u8>) -> i32 {
    if (bzpInfo == NULL!()).as_bool() {
        return BZP_ERROR_PARAM!();
    }
    bzpInfo.compressFile.input.filePtr = c_fopen!(inName.cast(), cstr!("rb")).cast();
    bzpInfo.compressFile.output.filePtr = c_fopen!(outName.cast(), cstr!("wb")).cast();
    if (bzpInfo.compressFile.input.filePtr == NULL!()).as_bool() || (bzpInfo.compressFile.output.filePtr == NULL!()).as_bool() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        c_remove!(outName.cast());
        return BZP_ERROR_IO!();
    }
    return BZP_OK!();
}