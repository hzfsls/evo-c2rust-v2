macro_rules! VOS_AVL3_DELETE { ($TREE:expr, $NODE:expr) => { VOS_AVL3_Delete(c_ref!($TREE), c_ref!($NODE)) } }
pub(crate) use VOS_AVL3_DELETE;
