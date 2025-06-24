pub fn VosAvlSearchReplaceNodeInLTree(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S>;
    if pstNode.pstLeft.pstRight == AVL_NULL_PTR!() {
        pstReplaceNode = pstNode.pstLeft;
        pstReplaceNode.pstRight = pstNode.pstRight;
        pstReplaceNode.pstRight.pstParent = pstReplaceNode;
        pstReplaceNode.sRHeight = pstNode.sRHeight;
    } else {
        VosAvlSwapRightMost(pstTree, pstNode.pstLeft, pstNode);
        pstReplaceNode = pstNode.pstLeft;
    }
    return pstReplaceNode;
}