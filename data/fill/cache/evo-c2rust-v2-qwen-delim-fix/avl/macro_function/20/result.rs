macro_rules! VOS_AVL3_FIND_OR_FIND_NEXT { ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
    AVL3_Find_Or_Find_Next(&$TREE.cast(), $KEY.cast(), AVL_FALSE!(), &$TREE_INFO.cast())
} }
pub(crate) use VOS_AVL3_FIND_OR_FIND_NEXT;