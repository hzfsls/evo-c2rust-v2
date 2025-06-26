pub fn VosAvlSwapLeftMost(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSubTree: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree;
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S>;
    let mut pstSwapRight: Ptr<AVLBASE_NODE_S>;
    FIND_LEFTMOST_NODE!(pstSwapNode);
    if pstSwapNode.sLHeight != 0 || pstSwapNode.sRHeight > 1 {
        return;
    }
    pstSwapParent = pstSwapNode.pstParent;
    pstSwapRight = pstSwapNode.pstRight;
    VosAvlUpdateSwapNode(pstTree, pstSwapNode, pstNode);
    VosAvlMoveNodeToNewPos(pstNode, pstSwapParent, AVL_NULL_PTR!(), pstSwapRight);
    pstNode.pstParent.pstLeft = pstNode;
    return;
}