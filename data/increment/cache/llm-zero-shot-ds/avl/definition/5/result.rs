pub struct AvlTree {
    pfn_compare: AVL_V2_COMPARE_FUNC,
    pst_root: *mut AVL_NODE,
    pst_first: *mut AVL_NODE,
    pst_last: *mut AVL_NODE,
}
