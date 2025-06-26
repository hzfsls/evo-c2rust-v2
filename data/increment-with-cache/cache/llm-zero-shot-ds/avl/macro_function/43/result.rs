macro_rules! TREE_OR_TREEINFO_IS_NULL {
    ($pstTree:expr, $pstTreeInfo:expr) => {
        ($pstTree.is_null() || $pstTreeInfo.is_null())
    };
}

pub(crate) use TREE_OR_TREEINFO_IS_NULL;
