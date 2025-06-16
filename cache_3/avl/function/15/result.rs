pub fn VosAvlSwapLeftMost(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstSubTree: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstSwapNode: Ptr<AVLBASE_NODE_S> = pstSubTree.cast();
    let mut pstSwapParent: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstSwapRight: Ptr<AVLBASE_NODE_S> = Default::default();

    FIND_LEFTMOST_NODE!(pstSwapNode);

    if (pstSwapNode.sLHeight != 0).as_bool() || (pstSwapNode.sRHeight > 1).as_bool() {
        return;
    }

    pstSwapParent = pstSwapNode.pstParent.cast();
    pstSwapRight = pstSwapNode.pstRight.cast();

    VosAvlUpdateSwapNode(pstTree.cast(), pstSwapNode.cast(), pstNode.cast());
    VosAvlMoveNodeToNewPos(pstNode.cast(), pstSwapParent.cast(), AVL_NULL_PTR!(), pstSwapRight.cast());

    pstNode.pstParent.pstLeft = pstNode.cast();

    return;
}
