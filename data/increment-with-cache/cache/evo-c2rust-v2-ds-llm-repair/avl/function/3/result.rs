pub fn VOS_AVL_Delete(mut pstTree: Ptr<AVL_TREE>, mut pstNode: Ptr<AVL_NODE>) {
    let mut pstBaseNode: Ptr<AVLBASE_NODE_S> = Default::default();
    let mut pstBaseTree: Ptr<AVLBASE_TREE_S> = Default::default();

    if (pstTree == AVL_NULL_PTR!()).as_bool() || (pstNode == AVL_NULL_PTR!()).as_bool() || (!VOS_AVL_IN_TREE!(*pstNode)).as_bool() {
        return;
    }

    pstBaseNode = pstNode.cast::<Ptr<AVLBASE_NODE_S>>();
    pstBaseTree = c_ref!(pstTree.pstRoot).cast::<Ptr<Void>>().cast::<Ptr<AVLBASE_TREE_S>>();
    VosAvlDelete(pstBaseNode.cast(), pstBaseTree.cast());
    return;
}
