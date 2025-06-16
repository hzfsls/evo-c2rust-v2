pub fn AVL3_Find_Or_Find_Next(mut pstTree: Ptr<AVL3_TREE>, mut pKey: Ptr<Void>, mut bFlag: u32, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut pFoundNode: Ptr<Void> = AVL_NULL_PTR!();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot.cast();
    if (pstNode == AVL_NULL_PTR!()).as_bool() {
        return AVL_NULL_PTR!();
    }

    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();

    loop {
        iResult = (pstTreeInfo.pfCompare)(pKey.cast(), (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            if (pstNode.pstRight == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
                break;
            }
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            if (pstNode.pstLeft == AVL_NULL_PTR!()).as_bool() {
                pFoundNode = (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
                break;
            }
            pstNode = pstNode.pstLeft.cast();
        } else {
            if (bFlag != 0).as_bool() {
                pFoundNode = VOS_AVL3_Next(pstNode.cast(), pstTreeInfo.cast()).cast();
            } else {
                pFoundNode = (pstNode.cast::<Ptr<u8>>() - pstTreeInfo.usNodeOffset).cast::<Ptr<Void>>();
            }
            break;
        }
    }

    return pFoundNode.cast();
}
