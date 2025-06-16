pub fn VosAvlBalanceTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstNodeTmp: Ptr<AVLBASE_NODE_S> = pstNode.cast();
    while (pstNodeTmp.pstParent != AVL_NULL_PTR!()).as_bool() {
        if (pstNodeTmp.pstParent.pstRight == pstNodeTmp).as_bool() {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstRight).cast());
            pstNodeTmp.sRHeight = (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstRight.sRHeight, pstNodeTmp.pstRight.sLHeight)).cast();
        } else {
            pstNodeTmp = pstNodeTmp.pstParent.cast();
            VosAvlRebalance(c_ref!(pstNodeTmp.pstLeft).cast());
            pstNodeTmp.sLHeight = (1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstLeft.sRHeight, pstNodeTmp.pstLeft.sLHeight)).cast();
        }
    }
    if (pstNodeTmp.sLHeight != pstNodeTmp.sRHeight).as_bool() {
        VosAvlRebalance(c_ref!(pstTree.pstRoot).cast());
    }
    return;
}
