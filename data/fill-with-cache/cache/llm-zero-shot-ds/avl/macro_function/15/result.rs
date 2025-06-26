macro_rules! VOS_AVL3_PREV {
    ($NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Prev(&($NODE), &($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_PREV;
