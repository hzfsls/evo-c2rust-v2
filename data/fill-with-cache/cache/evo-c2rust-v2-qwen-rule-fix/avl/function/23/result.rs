pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()) || (pstNode == AVL_NULL_PTR!()) || (VOS_AVL_IN_TREE!(*pstNode)) {
        return AVL_NULL_PTR!();
    }
    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;
    if (pstTree.pstRoot == AVL_NULL_PTR!()) {
        pstTree.pstRoot = pstNode;
        pstTree.pstFirst = pstNode;
        pstTree.pstLast = pstNode;
        return AVL_NULL_PTR!();
    }
    c_for!(pstParentNode = pstTree.pstRoot; pstParentNode != AVL_NULL_PTR!(); {
        iResult = (pstTree.pfnCompare)(pstNode.pKey, pstParentNode.pKey);
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()) {
                pstParentNode = pstParentNode.pstRight;
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S!() as Ptr<Void>), pstParentNode, pstNode);
            break;
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()) {
                pstParentNode = pstParentNode.pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert((AVLBASE_TREE_S!() as Ptr<Void>), pstParentNode, pstNode);
            break;
        }
        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf;
    });
    if (pstParentNode != AVL_NULL_PTR!()) {
        VosAvlBalanceTree((AVLBASE_TREE_S!() as Ptr<Void>), pstParentNode.cast());
    }
    return AVL_NULL_PTR!();
}