pub fn VOS_AVL_Find_Or_Find_Next(mut pstTree: Ptr<AVL_TREE>, mut pKey: Ptr<Void>, mut bValue: u32) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();

    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return pFoundNode.cast();
    }

    loop {
        iResult = (pstTree.pfnCompare)(pKey.cast(), pstNode.pKey.cast()).cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
                break;
            }

            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = pstNode.pSelf.cast();
                break;
            }

            pstNode = pstNode.pstLeft.cast();
        } else {
            if (bValue != 0).as_bool() {
                pFoundNode = VOS_AVL_Next(pstNode.cast()).cast();
            } else {
                pFoundNode = pstNode.pSelf.cast();
            }
            break;
        }
    }

    return pFoundNode.cast();
}
