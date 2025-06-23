pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() || (VOS_AVL_IN_TREE!(*pstNode).as_bool()) {
        return AVL_NULL_PTR!();
    }
    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;
    if (pstTree.pstRoot == AVL_NULL_PTR!()).as_bool() {
        pstTree.pstRoot = pstNode.cast();
        pstTree.pstFirst = pstNode.cast();
        pstTree.pstLast = pstNode.cast();
        return AVL_NULL_PTR!();
    }
    c_for!(pstParentNode = pstTree.pstRoot.cast(); pstParentNode != AVL_NULL_PTR!(); {
        iResult = (pstTree.pfnCompare)(pstNode.pKey.cast(), pstParentNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }
            VosAvlNodeRightInsert((AVLBASE_TREE_S!() as Ptr<Void>).cast(), pstParentNode.cast(), pstNode.cast());
            break;
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }
            VosAvlNodeLeftInsert((AVLBASE_TREE_S!() as Ptr<Void>).cast(), pstParentNode.cast(), pstNode.cast());
            break;
        }
        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf.cast();
    });
    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        VosAvlBalanceTree((AVLBASE_TREE_S!() as Ptr<Void>).cast(), pstParentNode.cast());
    }
    return AVL_NULL_PTR!();
}