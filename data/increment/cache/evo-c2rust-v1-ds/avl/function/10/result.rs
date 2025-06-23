pub fn VosAvlRotateRight(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstLeftSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstLeft.cast();

    (*ppstSubTree).pstLeft = pstLeftSon.pstRight.cast();
    if (*ppstSubTree).pstLeft != AVL_NULL_PTR!() {
        (*ppstSubTree).pstLeft.pstParent = (*ppstSubTree).cast();
    }

    (*ppstSubTree).sLHeight = pstLeftSon.sRHeight.cast();
    pstLeftSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstLeftSon.pstRight = (*ppstSubTree).cast();
    pstLeftSon.pstRight.pstParent = pstLeftSon.cast();
    pstLeftSon.sRHeight = (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();

    *ppstSubTree = pstLeftSon.cast();

    return;
}
