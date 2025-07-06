pub fn VOS_AVL_Prev(mut pstNode: Ptr<AVL_NODE>) -> VoidPtr {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode;
    if pstNodeTmp == AVL_NULL_PTR!() || !VOS_AVL_IN_TREE!(pstNodeTmp) {
        return AVL_NULL_PTR!();
    }
    if pstNodeTmp.pstLeft != AVL_NULL_PTR!() {
        pstNodeTmp = pstNodeTmp.pstLeft;
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while pstNodeTmp != AVL_NULL_PTR!() {
            if pstNodeTmp.pstParent == AVL_NULL_PTR!()
                || pstNodeTmp.pstParent.pstRight == pstNodeTmp
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