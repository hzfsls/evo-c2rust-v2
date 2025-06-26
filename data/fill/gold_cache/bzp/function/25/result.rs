pub fn BzpFileFinish(mut bzpF: Ptr<BzpFile>) {
    if bzpF != NULL!() {
        BzpStreamFinish(bzpF.input);
        BzpStreamFinish(bzpF.output);
        c_free!(bzpF);
        bzpF = NULL!();
    }
}