pub fn BzpOutComDataFinish(mut data: Ptr<BzpOutComdata>) {
    if (data != NULL!()).as_bool() {
        if (data.out != NULL!()).as_bool() {
            c_free!(data.out);
            data.out = NULL!();
        }
        c_free!(data);
        data = NULL!();
    }
}
