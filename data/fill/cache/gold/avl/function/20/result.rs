pub fn VosAvlSearchReplaceNode(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S>;
    if pstNode.sRHeight > pstNode.sLHeight {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree, pstNode);
    } else {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree, pstNode);
    }
    return pstReplaceNode;
}