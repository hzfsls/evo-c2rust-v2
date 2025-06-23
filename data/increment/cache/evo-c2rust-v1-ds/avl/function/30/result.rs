pub fn VOS_AVL3_Last(mut pstTree: Ptr<AVL3_TREE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) {
        return AVL_NULL_PTR!();
    }

    pstNode = pstTree.pstLast.cast();

    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}
