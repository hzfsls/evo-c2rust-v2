pub fn BzpOutComDataFinish(mut data: Ptr<BzpOutComdata>) {
    if data != NULL!() {
        if data.out != NULL!() {
            c_free!(data.out);
            data.out = NULL!();
        }
        c_free!(data);
        data = NULL!();
    }
}
