macro_rules! VOS_AVLL_INSERT_OR_FIND { ($TREE:expr, $NODE:expr) => 
    {
        VOS_AVL3_INSERT_OR_FIND!($TREE.stTree, $NODE, $TREE.stTreeInfo)
    }
}
pub(crate) use VOS_AVLL_INSERT_OR_FIND;