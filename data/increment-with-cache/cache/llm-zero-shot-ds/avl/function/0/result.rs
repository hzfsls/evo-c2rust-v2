use std::ptr;

unsafe fn vos_avl_node_right_insert(
    pst_tree: *mut AVLBASE_TREE_S,
    pst_parent_node: *mut AVLBASE_NODE_S,
    pst_node: *mut AVLBASE_NODE_S,
) {
    (*pst_node).pstParent = pst_parent_node;
    (*pst_parent_node).pstRight = pst_node;
    (*pst_parent_node).sRHeight = 1;
    
    if pst_parent_node == (*pst_tree).pstLast {
        (*pst_tree).pstLast = pst_node;
    }
}
