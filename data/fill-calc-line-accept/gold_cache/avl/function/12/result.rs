pub fn VosAvlUpdateSwapNode(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstSwapNode: Ptr<AVLBASE_NODE_S>,
    mut pstBaseNode: Ptr<AVLBASE_NODE_S>,
) {
    pstSwapNode.pstParent = pstBaseNode.pstParent;
    pstSwapNode.pstRight = pstBaseNode.pstRight;
    pstSwapNode.pstLeft = pstBaseNode.pstLeft;
    pstSwapNode.sRHeight = pstBaseNode.sRHeight;
    pstSwapNode.sLHeight = pstBaseNode.sLHeight;
    pstSwapNode.pstRight.pstParent = pstSwapNode;
    pstSwapNode.pstLeft.pstParent = pstSwapNode;
    if pstBaseNode.pstParent == AVL_NULL_PTR!() {
        pstTree.pstRoot = pstSwapNode;
    } else if pstBaseNode.pstParent.pstRight == pstBaseNode {
        pstSwapNode.pstParent.pstRight = pstSwapNode;
    } else {
        pstSwapNode.pstParent.pstLeft = pstSwapNode;
    }
}