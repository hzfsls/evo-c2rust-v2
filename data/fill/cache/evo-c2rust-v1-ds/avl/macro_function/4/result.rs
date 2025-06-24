macro_rules! VOS_AVL_FIND { ($TREE:expr, $KEY:expr) => { VOS_AVL_Find(c_ref!($TREE), $KEY) } }
pub(crate) use VOS_AVL_FIND;
