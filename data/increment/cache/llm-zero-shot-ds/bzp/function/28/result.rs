pub fn bzp_block_sort_main(bwt: &mut BzpBwtInfo) {
    bzp_binary_lifting_sort(bwt);

    for i in 0..bwt.n_block {
        if bwt.sort_block[i] == 0 {
            bwt.ori_ptr = i;
            break;
        }
    }
}
