macro_rules! VOS_AVL_FIND_OR_FIND_NEXT { ($TREE:expr, $KEY:expr) => { VOS_AVL_Find_Or_Find_Next(c_ref!($TREE), $KEY, AVL_FALSE!()) } }
pub(crate) use VOS_AVL_FIND_OR_FIND_NEXT;
