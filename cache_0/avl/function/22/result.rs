pub fn VosAvlDelete(mut pstBaseNode: Ptr<AVLBASE_NODE_S>, mut pstBaseTree: Ptr<AVLBASE_TREE_S>) {
    let mut pstReplaceNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstParentNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut sNewHeight: i16 = 0;

    pstReplaceNode = VosAvlDeleteCheck(pstBaseTree.cast(), pstBaseNode.cast()).cast();

    pstParentNode = pstBaseNode.pstParent.cast();

    pstBaseNode.pstParent = AVL_NULL_PTR!();
    pstBaseNode.pstRight = AVL_NULL_PTR!();
    pstBaseNode.pstLeft = AVL_NULL_PTR!();
    pstBaseNode.sRHeight = -1;
    pstBaseNode.sLHeight = -1;

    if (pstReplaceNode != AVL_NULL_PTR!()).as_bool() {
        pstReplaceNode.pstParent = pstParentNode.cast();
        sNewHeight = (1 + VOS_V2_AVL_MAX!(pstReplaceNode.sLHeight, pstReplaceNode.sRHeight)).cast();
    }

    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        if (pstParentNode.pstRight == pstBaseNode).as_bool() {
            pstParentNode.pstRight = pstReplaceNode.cast();
            pstParentNode.sRHeight = sNewHeight.cast();
        } else {
            pstParentNode.pstLeft = pstReplaceNode.cast();
            pstParentNode.sLHeight = sNewHeight.cast();
        }

        VosAvlBalanceTree(pstBaseTree.cast(), pstParentNode.cast());
    } else {
        pstBaseTree.pstRoot = pstReplaceNode.cast();
    }

    return;
}
