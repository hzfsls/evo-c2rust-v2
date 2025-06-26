pub fn VOS_AVL3_Find(mut pstTree: Ptr<AVL3_TREE>, mut pstKey: Ptr<Void>, mut pstTreeInfo: Ptr<AVL3_TREE_INFO>) -> Ptr<Void> {
    let mut pstNode: Ptr<AVL3_NODE> = Default::default();
    let mut iResult: i32 = Default::default();
    let mut iKeyOffset: i32 = Default::default();

    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo).as_bool() {
        return AVL_NULL_PTR!();
    }

    pstNode = pstTree.pstRoot.cast();
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo).cast();

    while (pstNode != AVL_NULL_PTR!()).as_bool() {
        iResult = (pstTreeInfo.pfCompare)(pstKey.cast(), (pstNode.cast::<Ptr<u8>>() + iKeyOffset).cast::<Ptr<Void>>()).cast();
        if iResult > 0 {
            pstNode = pstNode.pstRight.cast();
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft.cast();
        } else {
            break;
        }
    }

    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}
