macro_rules! VOS_AVL3_INSERT_OR_FIND { ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => 
    {
        VOS_AVL3_Insert_Or_Find(&$TREE.cast(), &$NODE.cast(), &$TREE_INFO.cast())
    }
}
pub(crate) use VOS_AVL3_INSERT_OR_FIND;