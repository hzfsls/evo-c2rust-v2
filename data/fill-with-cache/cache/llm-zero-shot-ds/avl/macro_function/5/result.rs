macro_rules! VOS_AVL_IN_TREE {
    ($node:expr) => {
        ($node.sLHeight != -1) && ($node.sRHeight != -1)
    };
}
pub(crate) use VOS_AVL_IN_TREE;
