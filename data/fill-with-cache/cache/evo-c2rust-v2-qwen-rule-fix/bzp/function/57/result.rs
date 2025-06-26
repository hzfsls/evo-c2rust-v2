pub fn BzpBlockSortMain(mut bwt: Ptr<BzpBwtInfo>) {
    BzpBinaryLiftingSort(bwt.cast());
    c_for!(let mut i: i32 = 0; i < bwt.nBlock.cast(); i.suffix_plus_plus(); {
        if bwt.sortBlock[i] == 0 {
            bwt.oriPtr = i.cast();
            break;
        }
    });
}