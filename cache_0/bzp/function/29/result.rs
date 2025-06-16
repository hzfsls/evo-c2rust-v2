pub fn BzpBwtFinish(mut bwt: Ptr<BzpBwtInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block);
            bwt.block = NULL!();
        }
        if (bwt.sortBlock != NULL!()).as_bool() {
            c_free!(bwt.sortBlock);
            bwt.sortBlock = NULL!();
        }
        if (bwt.idx != NULL!()).as_bool() {
            c_free!(bwt.idx);
            bwt.idx = NULL!();
        }
        if (bwt.isStartPos != NULL!()).as_bool() {
            c_free!(bwt.isStartPos);
            bwt.isStartPos = NULL!();
        }
        c_free!(bwt);
        bwt = NULL!();
    }
}
