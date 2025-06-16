pub fn VosAvlUpdateSwapNode(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSwapNode: Ptr<AVLBASE_NODE_S>, mut pstBaseNode: Ptr<AVLBASE_NODE_S>) {
    pstSwapNode.pstParent = pstBaseNode.pstParent.cast();
    pstSwapNode.pstRight = pstBaseNode.pstRight.cast();
    pstSwapNode.pstLeft = pstBaseNode.pstLeft.cast();
    pstSwapNode.sRHeight = pstBaseNode.sRHeight.cast();
    pstSwapNode.sLHeight = pstBaseNode.sLHeight.cast();
    pstSwapNode.pstRight.pstParent = pstSwapNode.cast();
    pstSwapNode.pstLeft.pstParent = pstSwapNode.cast();
    if (pstBaseNode.pstParent == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstSwapNode.cast();
    } else if (pstBaseNode.pstParent.pstRight == pstBaseNode).as_bool() {
        pstSwapNode.pstParent.pstRight = pstSwapNode.cast();
    } else {
        pstSwapNode.pstParent.pstLeft = pstSwapNode.cast();
    }
}
