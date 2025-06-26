pub fn VOS_AVL3_Delete(mut pstTree: Ptr<AVL3_TREE>, mut pstNode: Ptr<AVL3_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();
    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() {
        return;
    }
    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = pstTree.cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
}
