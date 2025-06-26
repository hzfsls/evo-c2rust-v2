macro_rules! VOS_AVL_DELETE { ($TREE:expr, $NODE:expr) => { VOS_AVL_Delete(c_ref!($TREE), c_ref!($NODE)) } }
pub(crate) use VOS_AVL_DELETE;
