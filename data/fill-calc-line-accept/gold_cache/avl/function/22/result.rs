pub fn VosAvlDelete(mut pstBaseNode: Ptr<AVLBASE_NODE_S>, mut pstBaseTree: Ptr<AVLBASE_TREE_S>) {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S>;
    let mut pstParentNode: Ptr<AVLBASE_NODE_S>;
    let mut sNewHeight: i16 = 0;
    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree, pstBaseNode);
    pstParentNode = pstBaseNode.pstParent;
    pstBaseNode.pstParent = AVL_NULL_PTR!();
    pstBaseNode.pstRight = AVL_NULL_PTR!();
    pstBaseNode.pstLeft = AVL_NULL_PTR!();
    pstBaseNode.sRHeight = -1;
    pstBaseNode.sLHeight = -1;
    if pstReplaceNode != AVL_NULL_PTR!() {
        pstReplaceNode.pstParent = pstParentNode;
        sNewHeight = 1 + VOS_V2_AVL_MAX!(pstReplaceNode.sLHeight, pstReplaceNode.sRHeight);
    }
    if pstParentNode != AVL_NULL_PTR!() {
        if pstParentNode.pstRight == pstBaseNode {
            pstParentNode.pstRight = pstReplaceNode;
            pstParentNode.sRHeight = sNewHeight;
        } else {
            pstParentNode.pstLeft = pstReplaceNode;
            pstParentNode.sLHeight = sNewHeight;
        }
        VosAvlBalanceTree(pstBaseTree, pstParentNode);
    } else {
        pstBaseTree.pstRoot = pstReplaceNode;
    }
    return;
}