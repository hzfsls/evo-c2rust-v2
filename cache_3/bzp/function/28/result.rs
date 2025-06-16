pub fn BzpBlockSortMain(mut bwt: Ptr<BzpBwtInfo>) {
    BzpBinaryLiftingSort(bwt.cast());

    c_for!(let mut i: i32 = 0; i < bwt.nBlock; i.suffix_plus_plus(); {
        if (bwt.sortBlock[i] == 0).as_bool() {
            bwt.oriPtr = i.cast();
            break;
        }
    });
}
