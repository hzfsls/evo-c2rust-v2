pub fn VosAvlBalanceTree(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    let mut pstNodeTmp: Ptr<AVLBASE_NODE_S> = pstNode;
    while pstNodeTmp.pstParent != AVL_NULL_PTR!() {
        if pstNodeTmp.pstParent.pstRight == pstNodeTmp {
            pstNodeTmp = pstNodeTmp.pstParent;
            VosAvlRebalance(c_ref!(pstNodeTmp.pstRight));
            pstNodeTmp.sRHeight =
                1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstRight.sRHeight, pstNodeTmp.pstRight.sLHeight);
        } else {
            pstNodeTmp = pstNodeTmp.pstParent;
            VosAvlRebalance(c_ref!(pstNodeTmp.pstLeft));
            pstNodeTmp.sLHeight =
                1 + VOS_V2_AVL_MAX!(pstNodeTmp.pstLeft.sRHeight, pstNodeTmp.pstLeft.sLHeight);
        }
    }
    if pstNodeTmp.sLHeight != pstNodeTmp.sRHeight {
        VosAvlRebalance(c_ref!(pstTree.pstRoot));
    }
    return;
}