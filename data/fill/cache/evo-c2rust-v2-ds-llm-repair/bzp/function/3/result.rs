pub fn BzpInDeComdataFinish(mut inData: Ptr<InDeComdata>) {
    if (inData != NULL!()).as_bool() {
        c_free!(inData);
        inData = NULL!();
    }
}
