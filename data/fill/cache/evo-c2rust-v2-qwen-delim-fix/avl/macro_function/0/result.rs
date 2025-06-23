macro_rules! VOS_AVL_INIT_TREE { ($tree:expr, $compare:expr) => 
    {
        $tree.pfnCompare = $compare.cast();
        $tree.pstFirst = AVL_NULL_PTR!();
        $tree.pstLast = AVL_NULL_PTR!();
        $tree.pstRoot = AVL_NULL_PTR!();
    }
}
pub(crate) use VOS_AVL_INIT_TREE;