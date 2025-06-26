macro_rules! VOS_AVL_FIRST {
    ($TREE:expr) => {
        if c_ref!($TREE).pstFirst != AVL_NULL_PTR!() {
            c_ref!($TREE).pstFirst.pSelf
        } else {
            AVL_NULL_PTR!()
        }
    }
}
pub(crate) use VOS_AVL_FIRST;
