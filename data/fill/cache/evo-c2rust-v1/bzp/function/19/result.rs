pub fn BzpDeComStreamFinish(mut inData: Ptr<InDeComdata>, mut inStream: Ptr<BzpStream>, mut outStream: Ptr<BzpStream>) {
    if inStream.filePtr != NULL!() {
        c_fclose!(inStream.filePtr);
        inStream.filePtr = NULL!();
    }
    if outStream.filePtr != NULL!() {
        c_fclose!(outStream.filePtr);
        outStream.filePtr = NULL!();
    }
    BzpStreamFinish(inStream.cast());
    BzpStreamFinish(outStream.cast());
    BzpInDeComdataFinish(inData.cast());
}
