macro_rules! VOS_AVL3_INSERT { ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
    VOS_AVL3_Insert_Or_Find(&$TREE.cast(), &$NODE.cast(), &$TREE_INFO.cast()) == AVL_NULL_PTR!()
} }
pub(crate) use VOS_AVL3_INSERT;