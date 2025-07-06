pub fn VosAvlMoveNodeToNewPos(
    mut pstNode: Ptr<AVLBASE_NODE_S>,
    mut pstNewParent: Ptr<AVLBASE_NODE_S>,
    mut pstNewLeftSon: Ptr<AVLBASE_NODE_S>,
    mut pstNewRightSon: Ptr<AVLBASE_NODE_S>,
) {
    pstNode.pstParent = pstNewParent;
    pstNode.pstLeft = pstNewLeftSon;
    pstNode.pstRight = pstNewRightSon;
    pstNode.sLHeight = 0;
    pstNode.sRHeight = 0;
    if pstNewLeftSon != AVL_NULL_PTR!() {
        pstNode.pstLeft.pstParent = pstNode;
        pstNode.sLHeight = 1;
    }
    if pstNewRightSon != AVL_NULL_PTR!() {
        pstNode.pstRight.pstParent = pstNode;
        pstNode.sRHeight = 1;
    }
}