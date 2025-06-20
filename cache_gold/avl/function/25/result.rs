pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: VoidPtr) -> VoidPtr {
    let mut pstNode: Ptr<AVL_NODE>;
    let mut iResult: i32;
    if pstTree == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot;
    while pstNode != AVL_NULL_PTR!() {
        iResult = (pstTree.pfnCompare)(pKey, pstNode.pKey);
        if iResult > 0 {
            pstNode = pstNode.pstRight;
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft;
        } else {
            break;
        }
    }
    return if pstNode != AVL_NULL_PTR!() {
        pstNode.pSelf
    } else {
        AVL_NULL_PTR!()
    };
}