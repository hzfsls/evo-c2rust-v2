pub fn VOS_AVL3_First(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> VoidPtr {
    let mut pstNode: Ptr<AVL3_NODE>;
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstFirst;
    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}
