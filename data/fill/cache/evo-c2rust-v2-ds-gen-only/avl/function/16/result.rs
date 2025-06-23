pub fn VosAvlRebalance(mut ppstSubTree: Ptr<Ptr<AVLBASE_NODE_S>>) {
    let mut iMoment: i32;
    iMoment = ((*ppstSubTree).sRHeight - (*ppstSubTree).sLHeight).cast();
    if (iMoment > 1).as_bool() {
        if ((*ppstSubTree).pstRight.sLHeight > (*ppstSubTree).pstRight.sRHeight).as_bool() {
            VosAvlRotateRight(c_ref!((*ppstSubTree).pstRight).cast());
        }
        VosAvlRotateLeft(ppstSubTree.cast());
    } else if (iMoment < -1).as_bool() {
        if ((*ppstSubTree).pstLeft.sRHeight > (*ppstSubTree).pstLeft.sLHeight).as_bool() {
            VosAvlRotateLeft(c_ref!((*ppstSubTree).pstLeft).cast());
        }
        VosAvlRotateRight(ppstSubTree.cast());
    }
    return;
}
