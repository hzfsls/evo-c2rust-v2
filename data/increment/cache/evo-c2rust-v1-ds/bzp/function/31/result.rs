pub fn BzpMtfReSet(mut mtf: Ptr<BzpMtfInfo>) {
    mtf.nUse = 0;
    mtf.nMtf = 0;
    mtf.block = NULL!();
    mtf.map = NULL!();
    mtf.inUse = NULL!();
}
