pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()).as_bool() {
        if (bwt.block != NULL!()).as_bool() {
            c_free!(bwt.block.cast()).cast::<Void>();
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()).as_bool() {
            c_free!(bwt.deCode.cast()).cast::<Void>();
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()).as_bool() {
            c_free!(bwt.sorted.cast()).cast::<Void>();
            bwt.sorted = NULL!();
        }
        c_free!(bwt.cast()).cast::<Void>();
        bwt = NULL!();
    }
}