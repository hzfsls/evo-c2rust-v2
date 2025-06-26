macro_rules! VOS_AVL_FIRST { ($TREE:expr) => {
    {
        if (&$TREE).pstFirst != AVL_NULL_PTR!() {
            (&$TREE).pstFirst.pSelf
        } else {
            AVL_NULL_PTR!()
        }
    }
} }
pub(crate) use VOS_AVL_FIRST;