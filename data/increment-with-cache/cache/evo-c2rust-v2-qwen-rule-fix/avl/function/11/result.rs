pub fn VOS_AVL3_Insert_Or_Find(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) || (pstNode == AVL_NULL_PTR!()) {
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

    pstParentNode = pstTree.pstRoot;

    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo);
    while (pstParentNode != AVL_NULL_PTR!()) {
        iResult = (pstTreeInfo.pfCompare)((c_ref!((pstNode.cast::<Ptr<u8>>() + iKeyOffset)).cast::<Ptr<Void>>()), (c_ref!((pstParentNode.cast::<Ptr<u8>>() + iKeyOffset)).cast::<Ptr<Void>>()));
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()) {
                pstParentNode = pstParentNode.pstRight;
                continue;
            }

            VosAvlNodeRightInsert(pstTree.cast(), pstParentNode.cast(), pstNode.cast());
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()) {
                pstParentNode = pstParentNode.pstLeft;
                continue;
            }

            VosAvlNodeLeftInsert(pstTree.cast(), pstParentNode.cast(), pstNode.cast());
        } else {
            pstNode.sRHeight = -1;
            pstNode.sLHeight = -1;
            return (c_ref!((pstParentNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset)).cast::<Ptr<Void>>());
        }

        break;
    }

    VosAvlBalanceTree(pstTree.cast(), pstParentNode.cast());

    return AVL_NULL_PTR!();
}