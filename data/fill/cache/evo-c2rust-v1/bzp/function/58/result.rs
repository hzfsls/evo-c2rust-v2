pub fn BzpBwtFinish(mut bwt: Ptr<BzpBwtInfo>) {
    if bwt != NULL!() {
        if bwt.block != NULL!() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if bwt.sortBlock != NULL!() {
            c_free!(bwt.sortBlock);
            bwt.sortBlock = NULL!();
        }
        if bwt.idx != NULL!() {
            c_free!(bwt.idx);
            bwt.idx = NULL!();
        }
        if bwt.isStartPos != NULL!() {
            c_free!(bwt.isStartPos);
            bwt.isStartPos = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}
