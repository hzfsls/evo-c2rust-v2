macro_rules! VOS_AVL_LAST {
    ($tree:expr) => {
        if (&$tree).pstLast != AVL_NODE::AVL_NULL_PTR {
            (&$tree).pstLast.pSelf
        } else {
            AVL_NODE::AVL_NULL_PTR
        }
    };
}
pub(crate) use VOS_AVL_LAST;
