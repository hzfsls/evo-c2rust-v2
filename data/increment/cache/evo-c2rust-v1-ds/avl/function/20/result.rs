pub fn VosAvlSearchReplaceNode(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if pstNode.sRHeight > pstNode.sLHeight {
        pstReplaceNode = VosAVLSearchReplaceNodeInRTree(pstTree.cast(), pstNode.cast()).cast();
    } else {
        pstReplaceNode = VosAvlSearchReplaceNodeInLTree(pstTree.cast(), pstNode.cast()).cast();
    }

    return pstReplaceNode.cast();
}
