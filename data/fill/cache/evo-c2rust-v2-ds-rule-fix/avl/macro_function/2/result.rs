macro_rules! VOS_AVL_INSERT { ($TREE:expr, $NODE:expr) => { VOS_AVL_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE)) == AVL_NULL_PTR!() } }
pub(crate) use VOS_AVL_INSERT;
