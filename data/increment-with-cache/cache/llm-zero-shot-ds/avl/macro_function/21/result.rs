macro_rules! VOS_AVL_FIRST {
    ($tree:expr) => {
        if (&$tree).pstFirst != AVL_NODE::AVL_NULL_PTR {
            (&$tree).pstFirst.pSelf
        } else {
            AVL_NODE::AVL_NULL_PTR
        }
    };
}

pub(crate) use VOS_AVL_FIRST;
