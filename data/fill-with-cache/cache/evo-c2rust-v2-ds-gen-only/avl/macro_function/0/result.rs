macro_rules! VOS_AVL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr) => {
        $TREE.pfnCompare = $COMPARE;
        $TREE.pstFirst = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstLast = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $TREE.pstRoot = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
    }
}
pub(crate) use VOS_AVL_INIT_TREE;
