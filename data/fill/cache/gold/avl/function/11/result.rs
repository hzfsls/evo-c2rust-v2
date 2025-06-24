pub fn VosAvlRotateLeft(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstRightSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstRight;
    (*ppstSubTree).pstRight = pstRightSon.pstLeft;
    if (*ppstSubTree).pstRight != AVL_NULL_PTR!() {
        (*ppstSubTree).pstRight.pstParent = *ppstSubTree;
    }
    (*ppstSubTree).sRHeight = pstRightSon.sLHeight;
    pstRightSon.pstParent = (*ppstSubTree).pstParent;
    pstRightSon.pstLeft = *ppstSubTree;
    pstRightSon.pstLeft.pstParent = pstRightSon;
    pstRightSon.sLHeight = 1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight);
    *ppstSubTree = pstRightSon;
    return;
}