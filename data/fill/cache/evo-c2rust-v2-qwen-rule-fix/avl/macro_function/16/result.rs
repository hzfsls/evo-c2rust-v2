macro_rules! VOS_AVL3_FIRST { ($TREE:expr, $TREE_INFO:expr) => { VOS_AVL3_First(&$TREE.cast(), &$TREE_INFO.cast()) } }
pub(crate) use VOS_AVL3_FIRST;