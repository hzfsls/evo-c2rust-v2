pub fn BzpFileInit() -> Ptr<BzpFile> {
    let mut compressFile: Ptr<BzpFile> = c_malloc!(c_sizeof!(BzpFile));
    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if compressFile == NULL!() || inStream == NULL!() || outStream == NULL!() {
        BzpStreamFinish(inStream.cast());
        BzpStreamFinish(outStream.cast());
        BzpFileFinish(compressFile.cast());
        return NULL!();
    }
    compressFile.input = inStream.cast();
    compressFile.output = outStream.cast();
    compressFile.input.pos = 0;
    compressFile.output.pos = 0;
    compressFile.num = 0;
    compressFile.lasChar = BZP_ASCII_SIZE!();
    compressFile.state = BZP_INPUT_COMPRESS!();
    return compressFile.cast();
}
