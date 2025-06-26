pub fn VOS_AVL3_Next(mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode;
    if (pstNodeTmp == AVL_NULL_PTR!()) || (pstTreeInfo == AVL_NULL_PTR!()) {
        return AVL_NULL_PTR!();
    }

    if (pstNodeTmp.pstRight != AVL_NULL_PTR!()) {
        pstNodeTmp = pstNodeTmp.pstRight;
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while (pstNodeTmp != AVL_NULL_PTR!()) {
            if (pstNodeTmp.pstParent == AVL_NULL_PTR!()) || (pstNodeTmp.pstParent.pstLeft == pstNodeTmp) {
                pstNodeTmp = pstNodeTmp.pstParent;
                break;
            }
            pstNodeTmp = pstNodeTmp.pstParent;
        }
    }

    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset);
}