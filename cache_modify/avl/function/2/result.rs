pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() || VOS_AVL_IN_TREE!(*pstNode).as_bool() {
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

    pstParentNode = pstTree.pstRoot.cast();
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pstNode.pKey.cast(), pstParentNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }

            VosAvlNodeRightInsert(c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());

            break;
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }

            VosAvlNodeLeftInsert(c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());

            break;
        }

        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf.cast();
    }

    if (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        VosAvlBalanceTree(c_ref!(pstTree.pstRoot).cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>());
    }

    return AVL_NULL_PTR!();
}
