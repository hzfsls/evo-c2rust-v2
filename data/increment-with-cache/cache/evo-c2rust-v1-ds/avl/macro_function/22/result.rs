macro_rules! VOS_AVL_LAST { ($TREE:expr) => { if c_ref!($TREE).pstLast != AVL_NULL_PTR!() { c_ref!($TREE).pstLast.pSelf } else { AVL_NULL_PTR!() } } }
pub(crate) use VOS_AVL_LAST;
