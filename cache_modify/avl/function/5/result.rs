pub fn VOS_AVL_Next(mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNodeTmp)).as_bool() {
        return AVL_NULL_PTR!();
    }

    if (pstNodeTmp.pstRight != AVL_NULL_PTR!()).as_bool() {
        pstNodeTmp = pstNodeTmp.pstRight.cast();
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()).as_bool() {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()).as_bool() || (pstNodeTmp.pstParent.pstLeft == pstNodeTmp).as_bool() {
                pstNodeTmp = pstNodeTmp.pstParent.cast();
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent.cast();
        }
    }

    return if pstNodeTmp != AVL_NULL_PTR!() { pstNodeTmp.pSelf.cast() } else { AVL_NULL_PTR!() };
}
