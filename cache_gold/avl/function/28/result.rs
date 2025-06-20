pub fn VOS_AVL_Find_Or_Find_Next(
    mut pstTree: Ptr<AVL_TREE>,
    mut pKey: VoidPtr,
    mut bValue: u32,
) -> VoidPtr {
    let mut pstNode: Ptr<AVL_NODE>;
    let mut pFoundNode: VoidPtr = AVL_NULL_PTR!();
    let mut iResult: i32;
    if pstTree == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot;
    if pstNode == AVL_NULL_PTR!() {
        return pFoundNode;
    }
    loop {
        iResult = (pstTree.pfnCompare)(pKey, pstNode.pKey);
        if iResult > 0 {
            if pstNode.pstRight == AVL_NULL_PTR!() {
                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            }
            pstNode = pstNode.pstRight;
        } else if iResult < 0 {
            if pstNode.pstLeft == AVL_NULL_PTR!() {
                pFoundNode = pstNode.pSelf;
                break;
            }
            pstNode = pstNode.pstLeft;
        } else {
            if bValue != 0 {
                pFoundNode = VOS_AVL_Next(pstNode);
            } else {
                pFoundNode = pstNode.pSelf;
            }
            break;
        }
    }
    return pFoundNode;
}