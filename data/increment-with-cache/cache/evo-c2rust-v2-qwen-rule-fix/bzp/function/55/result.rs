pub fn BzpBwtDecodeFinish(mut bwt: Ptr<BzpBwtDecodeInfo>) {
    if (bwt != NULL!()) {
        if (bwt.block != NULL!()) {
            c_free!(bwt.block.cast()).cast::<Void>();
            bwt.block = NULL!();
        }
        if (bwt.deCode != NULL!()) {
            c_free!(bwt.deCode.cast()).cast::<Void>();
            bwt.deCode = NULL!();
        }
        if (bwt.sorted != NULL!()) {
            c_free!(bwt.sorted.cast()).cast::<Void>();
            bwt.sorted = NULL!();
        }
        c_free!(bwt.cast()).cast::<Void>();
        bwt = NULL!();
    }
}