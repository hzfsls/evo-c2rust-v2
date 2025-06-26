pub fn AVL3_Find_Or_Find_Next(
    mut pstTree: Ptr<AVL3_TREE>,
    mut pKey: VoidPtr,
    mut bFlag: u32,
    mut pstTreeInfo: Ptr<AVL3_TREE_INFO>,
) -> VoidPtr {
    let mut pstNode: Ptr<AVL3_NODE>;
    let mut pFoundNode: VoidPtr = AVL_NULL_PTR!();
    let mut iResult: i32;
    let mut iKeyOffset: i32;
    if TREE_OR_TREEINFO_IS_NULL!(pstTree, pstTreeInfo) {
        return AVL_NULL_PTR!();
    }
    pstNode = pstTree.pstRoot;
    if pstNode == AVL_NULL_PTR!() {
        return AVL_NULL_PTR!();
    }
    iKeyOffset = GET_KEYOFFSET!(pstTreeInfo);
    loop {
        iResult = (pstTreeInfo.pfCompare)(pKey, pstNode.cast::<VoidPtr>() + iKeyOffset);
        if iResult > 0 {
            if pstNode.pstRight == AVL_NULL_PTR!() {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
                break;
            }
            pstNode = pstNode.pstRight;
        } else if iResult < 0 {
            if pstNode.pstLeft == AVL_NULL_PTR!() {
                pFoundNode = (pstNode - pstTreeInfo.usNodeOffset).cast();
                break;
            }
            pstNode = pstNode.pstLeft;
        } else {
            if bFlag != 0 {
                pFoundNode = VOS_AVL3_Next(pstNode, pstTreeInfo);
            } else {
                pFoundNode = (pstNode - pstTreeInfo.usNodeOffset).cast();
            }
            break;
        }
    }
    return pFoundNode;
}