pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if bwt != NULL!() {
        if bwt.block != NULL!() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if bwt.deCode != NULL!() {
            c_free!(bwt.deCode);
            bwt.deCode = NULL!();
        }
        if bwt.sorted != NULL!() {
            c_free!(bwt.sorted);
            bwt.sorted = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}
