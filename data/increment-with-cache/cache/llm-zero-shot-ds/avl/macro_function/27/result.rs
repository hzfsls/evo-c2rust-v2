macro_rules! VOS_AVL3_INIT_TREE {
    ($TREE:expr, $TREE_INFO:expr) => {
        $TREE.pstFirst = core::ptr::null_mut();
        $TREE.pstLast = core::ptr::null_mut();
        $TREE.pstRoot = core::ptr::null_mut();
    };
}

pub(crate) use VOS_AVL3_INIT_TREE;
