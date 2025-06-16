pub fn VosAvlSwapRightMost(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSubTree: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapLeft: Ptr<AVLBASE_NODE_S> = Default::default();

    FIND_RIGHTMOST_NODE!(pstSwapNode);

    if (pstSwapNode.sRHeight != 0).as_bool() || (pstSwapNode.sLHeight > 1).as_bool() {
        return;
    }

    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapLeft = pstSwapNode.pstLeft.cast();

    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(pstNode.cast(), pstSwapParent.cast(), pstSwapLeft.cast(), AVL_NULL_PTR!());

    pstNode.pstParent.pstRight = pstNode.cast();

    return;
}
