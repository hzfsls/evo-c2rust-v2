pub fn VosAVLSearchReplaceNodeInRTree(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S>;
    if pstNode.pstRight.pstLeft == AVL_NULL_PTR!() {
        pstReplaceNode = pstNode.pstRight;
        pstReplaceNode.pstLeft = pstNode.pstLeft;
        pstReplaceNode.pstLeft.pstParent = pstReplaceNode;
        pstReplaceNode.sLHeight = pstNode.sLHeight;
    } else {
        VosAvlSwapLeftMost(pstTree, pstNode.pstRight, pstNode);
        pstReplaceNode = pstNode.pstRight;
    }
    return pstReplaceNode;
}