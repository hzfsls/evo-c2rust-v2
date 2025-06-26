pub fn BzpBwtFinish(mut bwt: Ptr<BzpBwtInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block.cast());
            bwt.block = NULL!();
        }
        if (bwt.sortBlock != NULL!()).as_bool() {
            c_free!(bwt.sortBlock.cast());
            bwt.sortBlock = NULL!();
        }
        if (bwt.idx != NULL!()).as_bool() {
            c_free!(bwt.idx.cast());
            bwt.idx = NULL!();
        }
        if (bwt.isStartPos != NULL!()).as_bool() {
            c_free!(bwt.isStartPos.cast());
            bwt.isStartPos = NULL!();
        }
        c_free!(bwt.cast());
        bwt = NULL!();
    }
}