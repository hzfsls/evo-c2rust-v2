pub fn VOS_AVL3_Insert_Or_Find(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstNode: Ptr<AVL3_NODE>,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> VoidPtr {
    let mut pstParentNode: Ptr<AVL3_NODE>;
    let mut iResult: i32;
    let mut iKeyOffset: i32;
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) || pstNode == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }
    pstNode.sRHeight = 0;
    pstNode.sLHeight = 0;
    if pstTree.pstRoot == AVL_NULL_PTR!() {
        pstTree.pstRoot = pstNode;
        pstTree.pstFirst = pstNode;
        pstTree.pstLast = pstNode;
        return AVL_NULL_PTR!();
    }
    pstParentNode = pstTree.pstRoot;
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo);
    while pstParentNode != AVL_NULL_PTR!() {
        iResult = (pstTreeInfo.pfCompare)(
            pstNode.cast::<VoidPtr>() + iKeyOffset,
            pstParentNode.cast::<VoidPtr>() + iKeyOffset,
        );
        if iResult > 0 {
            if pstParentNode.pstRight != AVL_NULL_PTR!() {
                pstParentNode = pstParentNode.pstRight;
                continue;
            }
            VosAvlNodeRightInsert(pstTree.cast(), pstParentNode.cast(), pstNode.cast());
        } else if iResult < 0 {
            if pstParentNode.pstLeft != AVL_NULL_PTR!() {
                pstParentNode = pstParentNode.pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert(pstTree.cast(), pstParentNode.cast(), pstNode.cast());
        } else {
            pstNode.sRHeight = -1;
            pstNode.sLHeight = -1;
            return (pstParentNode - pstTreeInfo.usNodeOffset).cast();
        }
        break;
    }
    VosAvlBalanceTree(pstTree.cast(), pstParentNode.cast());
    return AVL_NULL_PTR!();
}