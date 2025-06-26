pub fn VosAvlSearchReplaceNodeInLTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstLeft.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();
        pstReplaceNode.pstRight = pstNode.pstRight.cast();
        pstReplaceNode.pstRight.pstParent = pstReplaceNode.cast();
        pstReplaceNode.sRHeight = pstNode.sRHeight.cast();
    } else {
        VosAvlSwapRightMost(pstTree.cast(), pstNode.pstLeft.cast(), pstNode.cast());
        pstReplaceNode = pstNode.pstLeft.cast();
    }

    return pstReplaceNode.cast();
}
