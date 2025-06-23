pub fn VOS_AVL_Next(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if pstNodeTmp == AVL_NULL_PTR!() || !VOS_AVL_IN_TREE!(*pstNodeTmp) {
        return AVL_NULL_PTR!();
    }

    if pstNodeTmp.pstRight != AVL_NULL_PTR!() {
        pstNodeTmp = pstNodeTmp.pstRight.cast();
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while pstNodeTmp != AVL_NULL_PTR!() {
            if pstNodeTmp.pstParent == AVL_NULL_PTR!() || pstNodeTmp.pstParent.pstLeft == pstNodeTmp {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }

            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }

    return if pstNodeTmp != AVL_NULL_PTR!() { pstNodeTmp.pSelf.cast() } else { AVL_NULL_PTR!() };
}
