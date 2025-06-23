macro_rules! VOS_AVL3_NEXT {
    ($NODE:expr, $TREE_INFO:expr) => {
        VOS_AVL3_Next(&($NODE), &($TREE_INFO))
    };
}
pub(crate) use VOS_AVL3_NEXT;
