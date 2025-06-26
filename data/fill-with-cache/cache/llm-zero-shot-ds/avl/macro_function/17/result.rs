macro_rules! VOS_AVL3_LAST {
    ($TREE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Last(&($TREE), &($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_LAST;
