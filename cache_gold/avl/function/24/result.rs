pub fn VOS_AVL_Delete(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S>;
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S>;
    if pstTree == AVL_NULL_PTR!() || pstNode == AVL_NULL_PTR!() || !VOS_AVL_IN_TREE!(pstNode) {
        return;
    }
    pstBaseNode = pstNode.cast();
    pstBaseTree = c_ref!(pstTree.pstRoot).cast();
    VosAvlDelete(pstBaseNode, pstBaseTree);
    return;
}