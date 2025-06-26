macro_rules! VOS_AVL_FIND_NEXT { ($TREE:expr, $KEY:expr) => { VOS_AVL_Find_Or_Find_Next(&$TREE.cast(), $KEY.cast(), AVL_TRUE) } }
pub(crate) use VOS_AVL_FIND_NEXT;