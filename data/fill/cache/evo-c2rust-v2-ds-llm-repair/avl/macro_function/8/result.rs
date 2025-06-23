macro_rules! VOS_AVL3_INIT_TREE {
    ($TREE:expr, $TREE_INFO:expr) => {
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
    }
}
pub(crate) use VOS_AVL3_INIT_TREE;
