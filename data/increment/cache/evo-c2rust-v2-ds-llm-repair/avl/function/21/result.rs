pub fn VosAvlDeleteCheck(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() && (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = AVL_NULL_PTR!();

        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstNode.pstParent.cast();
        }

        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstNode.pstParent.cast();
        }
    } else if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstRight.cast();

        if (pstTree.pstFirst == pstNode).as_bool() {
            pstTree.pstFirst = pstReplaceNode.cast();
        }
    } else if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode = pstNode.pstLeft.cast();

        if (pstTree.pstLast == pstNode).as_bool() {
            pstTree.pstLast = pstReplaceNode.cast();
        }
    } else {
        pstReplaceNode = VosAvlSearchReplaceNode(pstTree.cast(), pstNode.cast()).cast();
    }
    return pstReplaceNode.cast();
}
