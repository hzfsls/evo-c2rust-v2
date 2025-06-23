macro_rules! VOS_AVL3_DELETE {
    ($TREE:expr, $NODE:expr) => {
        VOS_AVL3_Delete(&mut ($TREE), &($NODE))
    };
}
pub(crate) use VOS_AVL3_DELETE;
