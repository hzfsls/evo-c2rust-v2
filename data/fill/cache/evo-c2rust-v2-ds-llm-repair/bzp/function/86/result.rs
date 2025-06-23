pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()).as_bool() {
            c_free!(bwt.deCode);
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()).as_bool() {
            c_free!(bwt.sorted);
            bwt.sorted = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}
