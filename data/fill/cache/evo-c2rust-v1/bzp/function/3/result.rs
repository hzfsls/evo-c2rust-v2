pub fn BzpInDeComdataFinish(mut inData: Ptr<InDeComdata>) {
    if inData != NULL!() {
        c_free!(inData);
        inData = NULL!();
    }
}
