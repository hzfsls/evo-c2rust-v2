pub fn VOS_AVL_Next(mut pstNode: Ptr<AVL_NODE>) -> VoidPtr {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode;
    if pstNodeTmp == AVL_NULL_PTR!() || !VOS_AVL_IN_TREE!(pstNodeTmp) {
        return AVL_NULL_PTR!();
    }
    if pstNodeTmp.pstRight != AVL_NULL_PTR!() {
        pstNodeTmp = pstNodeTmp.pstRight;
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        loop {
            if pstNodeTmp == AVL_NULL_PTR!() {
                break;
            }
            if pstNodeTmp.pstParent == AVL_NULL_PTR!() || pstNodeTmp.pstParent.pstLeft == pstNodeTmp
            {
                pstNodeTmp = pstNodeTmp.pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent;
        }
    }
    return if pstNodeTmp != AVL_NULL_PTR!() {
        pstNodeTmp.pSelf
    } else {
        AVL_NULL_PTR!()
    };
}