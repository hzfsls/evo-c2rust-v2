macro_rules! VOS_AVL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr) => {
        $TREE.pfnCompare = $COMPARE;
        $TREE.pstFirst = AVL_NULL_PTR!();
        $TREE.pstLast = AVL_NULL_PTR!();
        $TREE.pstRoot = AVL_NULL_PTR!();
    };
}
pub(crate) use VOS_AVL_INIT_TREE;