macro_rules! VOS_AVL3_FIND_NEXT {
    ($TREE:expr, $KEY:expr, $TREE_INFO:expr) => {
        AVL3_Find_Or_Find_Next(&($TREE), $KEY, AVL_TRUE, &($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_FIND_NEXT;
