pub fn VOS_AVL3_Insert_Or_Find(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstParentNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() {
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
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();
    while (pstParentNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)((pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>(), (pstParentNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            if (pstParentNode.pstRight != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstRight.cast();
                continue;
            }
            VosAvlNodeRightInsert(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
        } else if iResult < 0 {
            if (pstParentNode.pstLeft != AVL_NULL_PTR!()).as_bool() {
                pstParentNode = pstParentNode.pstLeft.cast();
                continue;
            }
            VosAvlNodeLeftInsert(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>(), pstNode.cast::<Ptr<AVLBASE_NODE_S>>());
        } else {
            pstNode.sRHeight = -1;
            pstNode.sLHeight = -1;
            return (pstParentNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
        }
        break;
    }
    VosAvlBalanceTree(pstTree.cast::<Ptr<AVLBASE_TREE_S>>(), pstParentNode.cast::<Ptr<AVLBASE_NODE_S>>());
    return AVL_NULL_PTR!();
}
