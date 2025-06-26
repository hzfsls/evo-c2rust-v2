pub fn VOS_AVL3_Delete(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S>;
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S>;
    if pstTree == AVL_NULL_PTR!() || pstNode == AVL_NULL_PTR!() {
        return;
    }
    pstBaseNode = pstNode.cast();
    pstBaseTree = pstTree.cast();
    VosAvlDelete(pstBaseNode, pstBaseTree);
}