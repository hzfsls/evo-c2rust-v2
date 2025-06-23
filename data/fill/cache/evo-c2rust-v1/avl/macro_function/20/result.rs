macro_rules! VOS_AVL3_FIND_OR_FIND_NEXT { ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => { AVL3_Find_Or_Find_Next(c_ref!($TREE), $KEY.cast(), AVL_FALSE!().cast(), c_ref!($TREE_INFO)) } }
pub(crate) use VOS_AVL3_FIND_OR_FIND_NEXT;
