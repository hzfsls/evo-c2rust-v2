pub fn VosAvlRotateLeft(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut pstRightSon: Ptr<AVLBASE_NODE_S> = (*ppstSubTree).pstRight.cast();

    (*ppstSubTree).pstRight = pstRightSon.pstLeft.cast();
    if ((*ppstSubTree).pstRight != AVL_NULL_PTR!()).as_bool() {
        (*ppstSubTree).pstRight.pstParent = (*ppstSubTree).cast();
    }

    (*ppstSubTree).sRHeight = pstRightSon.sLHeight.cast();
    pstRightSon.pstParent = (*ppstSubTree).pstParent.cast();
    pstRightSon.pstLeft = (*ppstSubTree).cast();
    pstRightSon.pstLeft.pstParent = pstRightSon.cast();
    pstRightSon.sLHeight = (1 + VOS_V2_AVL_MAX!((*ppstSubTree).sRHeight, (*ppstSubTree).sLHeight)).cast();

    *ppstSubTree = pstRightSon.cast();

    return;
}
