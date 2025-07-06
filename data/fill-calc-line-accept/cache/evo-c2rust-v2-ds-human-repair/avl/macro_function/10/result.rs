macro_rules! VOS_AVL3_INSERT { ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {  VOS_AVL3_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE), c_ref!($TREE_INFO)) == AVL_NULL_PTR!() } }
pub(crate) use VOS_AVL3_INSERT;
