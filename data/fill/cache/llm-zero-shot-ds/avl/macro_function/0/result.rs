macro_rules! VOS_AVL_INIT_TREE {
    ($TREE:expr, $COMPARE:expr) => {
        $TREE.pfnCompare = $COMPARE;
        $TREE.pstFirst = core::ptr::null_mut();
        $TREE.pstLast = core::ptr::null_mut();
        $TREE.pstRoot = core::ptr::null_mut();
    };
}

pub(crate) use VOS_AVL_INIT_TREE;
