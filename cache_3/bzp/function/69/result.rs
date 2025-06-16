pub fn BzpFileFinish(mut bzpF: Ptr<BzpFile>) {
    if (bzpF != NULL!()).as_bool() {
        BzpStreamFinish(bzpF.input.cast());
        BzpStreamFinish(bzpF.output.cast());
        c_free!(bzpF);
        bzpF = NULL!();
    }
}
