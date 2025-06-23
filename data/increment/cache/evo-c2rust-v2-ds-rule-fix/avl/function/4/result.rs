pub fn VOS_AVL_Find(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();

    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }

    return if pstNode != AVL_NULL_PTR!() { pstNode.pSelf.cast() } else { AVL_NULL_PTR!() };
}
