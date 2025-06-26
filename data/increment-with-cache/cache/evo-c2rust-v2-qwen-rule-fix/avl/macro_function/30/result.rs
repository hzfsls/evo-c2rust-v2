macro_rules! VOS_AVL_INSERT_OR_FIND { ($TREE:expr, $NODE:expr) => 
    {
        VOS_AVL_Insert_Or_Find(c_ref!($TREE), c_ref!($NODE))
    }
}
pub(crate) use VOS_AVL_INSERT_OR_FIND;