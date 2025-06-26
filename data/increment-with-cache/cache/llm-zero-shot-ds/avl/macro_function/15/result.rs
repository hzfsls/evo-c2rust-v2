macro_rules! VOS_AVL_INSERT {
    ($TREE:expr, $NODE:expr) => {
        (VOS_AVL_Insert_Or_Find(&mut ($TREE), &($NODE)) == AVL_NULL_PTR)
    };
}
pub(crate) use VOS_AVL_INSERT;
