macro_rules! VOS_AVL3_PREV { ($NODE:expr, $TREE_INFO:expr) => { VOS_AVL3_Prev(c_ref!($NODE), c_ref!($TREE_INFO)) } }
pub(crate) use VOS_AVL3_PREV;