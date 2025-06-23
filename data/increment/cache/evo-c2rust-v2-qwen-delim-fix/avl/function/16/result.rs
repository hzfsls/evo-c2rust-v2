pub fn VOS_AVL3_Next(mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNodeTmp: Ptr<AVL3_NODE> = pstNode.cast();
    if (pstNodeTmp == AVL_NULL_PTR!()).as_bool() || (pstTreeInfo == AVL_NULL_PTR!()).as_bool() {
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

    return GET_NODE_START_ADDRESS!(pstNodeTmp, pstTreeInfo.usNodeOffset).cast();
}