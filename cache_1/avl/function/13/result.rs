pub fn VosAvlMoveNodeToNewPos(mut pstNode: Ptr<AVLBASE_NODE_S>, mut pstNewParent: Ptr<AVLBASE_NODE_S>, mut pstNewLeftSon: Ptr<AVLBASE_NODE_S>, mut pstNewRightSon: Ptr<AVLBASE_NODE_S>) {
    pstNode.pstParent = pstNewParent.cast();
    pstNode.pstLeft = pstNewLeftSon.cast();
    pstNode.pstRight = pstNewRightSon.cast();
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;

    if (pstNewLeftSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstLeft.pstParent = pstNode.cast();
        pstNode.sLHeight = 1;
    }

    if (pstNewRightSon != AVL_NULL_PTR!()).as_bool() {
        pstNode.pstRight.pstParent = pstNode.cast();
        pstNode.sRHeight = 1;
    }
}
