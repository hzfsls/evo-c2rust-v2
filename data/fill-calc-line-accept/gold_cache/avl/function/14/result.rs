pub fn VosAvlSwapRightMost(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSubTree: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree;
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S>;
    let mut pstSwapLeft: Ptr<AVLBASE_NODE_S>;
    FIND_RIGHTMOST_NODE!(pstSwapNode);
    if pstSwapNode.sRHeight != 0 || pstSwapNode.sLHeight > 1 {
        return;
    }
    pstSwapParent = pstSwapNode.pstParent;
    pstSwapLeft = pstSwapNode.pstLeft;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, pstSwapLeft, AVL_NULL_PTR!());
    pstNode.pstParent.pstRight = pstNode;
    return;
}