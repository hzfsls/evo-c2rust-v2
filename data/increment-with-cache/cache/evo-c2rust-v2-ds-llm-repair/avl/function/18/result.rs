pub fn VosAVLSearchReplaceNodeInRTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstRight.pstLeft == AVL_NULL_PTR!()).as_bool() {

        pstReplaceNode = pstNode.pstRight.cast();
        pstReplaceNode.pstLeft = pstNode.pstLeft.cast();
        pstReplaceNode.pstLeft.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sLHeight = pstNode.sLHeight.cast();
    } else {

        VosAvlSwapLeftMost(pstTree.cast(), pstNode.pstRight.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstRight.cast();
    }

    return pstReplaceNode.cast();
}
