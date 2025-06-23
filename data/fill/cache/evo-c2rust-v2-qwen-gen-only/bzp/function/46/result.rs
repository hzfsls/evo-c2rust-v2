pub fn BzpCompressEnd(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo.compressFile.input.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.input.filePtr.cast::<FilePtr>());
    }
    if (bzpInfo.compressFile.output.filePtr != NULL!()).as_bool() {
        c_fclose!(bzpInfo.compressFile.output.filePtr.cast::<FilePtr>());
    }
    BzpAlgorithmInfoFinish(bzpInfo.cast());
}