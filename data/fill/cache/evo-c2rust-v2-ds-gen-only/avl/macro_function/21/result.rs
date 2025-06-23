macro_rules! TREE_OR_TREEINFO_IS_NULL { ($pstTree:expr, $pstTreeInfo:expr) => { ($pstTree == AVL_NULL_PTR!()) || ($pstTreeInfo == AVL_NULL_PTR!()) } }
pub(crate) use TREE_OR_TREEINFO_IS_NULL;
