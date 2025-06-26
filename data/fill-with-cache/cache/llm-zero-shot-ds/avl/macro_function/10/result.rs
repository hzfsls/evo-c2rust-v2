macro_rules! VOS_AVL3_INSERT {
    ($TREE:expr, $NODE:expr, $TREE_INFO:expr) => {
        AVL_NULL_PTR == VOS_AVL3_Insert_Or_Find(&($TREE), &($NODE), &($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_INSERT;
