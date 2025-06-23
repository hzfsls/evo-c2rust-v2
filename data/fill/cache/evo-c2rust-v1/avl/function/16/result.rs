pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32;
    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();
    if iMoment > 1 {
        if (*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }
        VosAvlRotateLeft(ppstSubTree.cast());
    } else if iMoment < -1 {
        if (*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }
        VosAvlRotateRight(ppstSubTree.cast());
    }
    return;
}
