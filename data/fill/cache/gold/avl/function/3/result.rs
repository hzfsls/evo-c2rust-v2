pub fn VOS_AVL3_Find(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pstKey: VoidPtr,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> VoidPtr {
    let mut pstNode: Ptr<AVL3_NODE>;
    let mut iResult: i32;
    let mut iKeyOffset: i32;
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot;
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo);
    while pstNode != AVL_NULL_PTR!() {
        iResult = (pstTreeInfo.pfCompare)(pstKey, pstNode.cast::<VoidPtr>() + iKeyOffset);
        if iResult > 0 {
            pstNode = pstNode.pstRight;
        } else if iResult < 0 {
            pstNode = pstNode.pstLeft;
        } else {
            break;
        }
    }
    return GET_NODE_START_ADDRESS!(pstNode, pstTreeInfo.usNodeOffset);
}