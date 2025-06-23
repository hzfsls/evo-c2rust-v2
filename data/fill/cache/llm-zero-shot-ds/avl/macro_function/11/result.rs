macro_rules! VOS_AVL3_INSERT_OR_FIND {
    ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Insert_Or_Find(&($TREE), &($NODE), &($TREE_INFO))
    };
}

pub(crate) use VOS_AVL3_INSERT_OR_FIND;
