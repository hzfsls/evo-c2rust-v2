pub fn VOS_AVL_Insert_Or_Find(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) -> VoidPtr {
    let mut pstParentNode: Ptr<AVL_NODE>;
    let mut iResult: i32;
    if pstTree == AVL_NULL_PTR!() || pstNode == AVL_NULL_PTR!() || VOS_AVL_IN_TREE!(pstNode) {
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
    c_for!(pstParentNode = pstTree.pstRoot; pstParentNode != AVL_NULL_PTR!();;{
        iResult = (pstTree.pfnCompare)(pstNode.pKey, pstParentNode.pKey);
        if iResult > 0 {
            if pstParentNode.pstRight != AVL_NULL_PTR!() {
                pstParentNode = pstParentNode.pstRight;
                continue;
            }
            VosAvlNodeRightInsert(c_ref!(pstTree.pstRoot).cast(), pstParentNode.cast(), pstNode.cast());
            break;
        } else if iResult < 0 {
            if pstParentNode.pstLeft != AVL_NULL_PTR!() {
                pstParentNode = pstParentNode.pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert(c_ref!(pstTree.pstRoot).cast(), pstParentNode.cast(), pstNode.cast());
            break;
        }
        pstNode.sRHeight = -1;
        pstNode.sLHeight = -1;
        return pstParentNode.pSelf;
    });
    if pstParentNode != AVL_NULL_PTR!() {
        VosAvlBalanceTree(c_ref!(pstTree.pstRoot).cast(), pstParentNode.cast());
    }
    return AVL_NULL_PTR!();
}