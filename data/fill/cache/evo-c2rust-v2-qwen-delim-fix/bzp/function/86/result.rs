pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block.cast());
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()).as_bool() {
            c_free!(bwt.deCode.cast());
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()).as_bool() {
            c_free!(bwt.sorted.cast());
            bwt.sorted = NULL!();
        }
        c_free!(bwt.cast());
        bwt = NULL!();
    }
}