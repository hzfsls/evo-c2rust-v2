pub fn VosAvlNodeLeftInsert(mut pstTree: Ptr<AVLBASE_TREE_S>, mut pstParentNode: Ptr<AVLBASE_NODE_S>, mut pstNode: Ptr<AVLBASE_NODE_S>) {
    pstNode.pstParent = pstParentNode.cast();
    pstParentNode.pstLeft = pstNode.cast();
    pstParentNode.sLHeight = 1;
    if (pstParentNode == pstTree.pstFirst).as_bool() {
        pstTree.pstFirst = pstNode.cast();
    }
}
