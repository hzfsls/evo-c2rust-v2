pub fn VosAvlDeleteCheck(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) -> Ptr<AVLBASE_NODE_S> {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();

    if pstNode.pstLeft == AVL_NULL_PTR!() && pstNode.pstRight == AVL_NULL_PTR!() {

        pstReplaceNode = AVL_NULL_PTR!();

        if pstTree.pstFirst == pstNode {

            pstTree.pstFirst = pstNode.pstParent.cast();
        }

        if pstTree.pstLast == pstNode {

            pstTree.pstLast = pstNode.pstParent.cast();
        }
    } else if pstNode.pstLeft == AVL_NULL_PTR!() {

        pstReplaceNode = pstNode.pstRight.cast();

        if pstTree.pstFirst == pstNode {

            pstTree.pstFirst = pstReplaceNode.cast();
        }
    } else if pstNode.pstRight == AVL_NULL_PTR!() {

        pstReplaceNode = pstNode.pstLeft.cast();

        if pstTree.pstLast == pstNode {

            pstTree.pstLast = pstReplaceNode.cast();
        }
    } else {

        pstReplaceNode = VosAvlSearchReplaceNode(pstTree.cast(), pstNode.cast()).cast();
    }
    return pstReplaceNode.cast();
}
