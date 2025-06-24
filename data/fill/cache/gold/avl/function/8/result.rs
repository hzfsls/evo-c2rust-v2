pub fn VosAvlNodeRightInsert(
    mut pstTree: Ptr<AVLBASE_TREE_S>,
    mut pstParentNode: Ptr<AVLBASE_NODE_S>,
    mut pstNode: Ptr<AVLBASE_NODE_S>,
) {
    pstNode.pstParent = pstParentNode;
    pstParentNode.pstRight = pstNode;
    pstParentNode.sRHeight = 1;
    if pstParentNode == pstTree.pstLast {
        pstTree.pstLast = pstNode;
    }
}