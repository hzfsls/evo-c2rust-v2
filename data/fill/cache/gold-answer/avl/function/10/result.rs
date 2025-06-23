pub fn VosAvlRotateRight(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstLeftSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstLeft;
    (*ppstSubTree).pstLeft = pstLeftSon.pstRight;
    if (*ppstSubTree).pstLeft != AVL_NULL_PTR!() {
        (*ppstSubTree).pstLeft.pstParent = *ppstSubTree;
    }
    (*ppstSubTree).sLHeight = pstLeftSon.sRHeight;
    pstLeftSon.pstParent = (*ppstSubTree).pstParent;
    pstLeftSon.pstRight = *ppstSubTree;
    pstLeftSon.pstRight.pstParent = pstLeftSon;
    pstLeftSon.sRHeight = 1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight);
    *ppstSubTree = pstLeftSon;
    return;
}