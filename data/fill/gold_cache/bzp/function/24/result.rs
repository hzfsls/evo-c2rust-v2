pub fn BzpFileInit() -> Ptr<BzpFile> {
    let mut compressFile: Ptr<BzpFile> = c_malloc!(c_sizeof!(BzpFile));
    let mut inStream: Ptr<BzpStream> = BzpStreamInit();
    let mut outStream: Ptr<BzpStream> = BzpStreamInit();
    if compressFile == NULL!() || inStream == NULL!() || outStream == NULL!() {
        BzpStreamFinish(inStream);
        BzpStreamFinish(outStream);
        BzpFileFinish(compressFile);
        return NULL!();
    }
    compressFile.input = inStream;
    compressFile.output = outStream;
    compressFile.input.pos = 0;
    compressFile.output.pos = 0;
    compressFile.num = 0;
    compressFile.lasChar = BZP_ASCII_SIZE!();
    compressFile.state = BZP_INPUT_COMPRESS!();
    return compressFile;
}