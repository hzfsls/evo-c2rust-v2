pub fn BzpBlockSortMain(mut bwt: Ptr<BzpBwtInfo>) {
    BzpBinaryLiftingSort(bwt);
    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        if bwt.sortBlock[i] == 0 {
            bwt.oriPtr = i;
            break;
        }
    });
}