macro_rules! VOS_AVL_NEXT {
    ($NODE:expr) => {
        VOS_AVL_Next(&($NODE))
    };
}
pub(crate) use VOS_AVL_NEXT;
