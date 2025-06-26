macro_rules! VOS_AVL_LAST { ($TREE:expr) => {
    {
        if (&$TREE).pstLast != AVL_NULL_PTR!() {
            (&$TREE).pstLast.pSelf
        } else {
            AVL_NULL_PTR!()
        }
    }
} }
pub(crate) use VOS_AVL_LAST;