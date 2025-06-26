use std::ptr;

unsafe fn vos_avl_node_right_insert(
    pst_tree: *mut AVLBASE_TREE_S,
    pst_parent_node: *mut AVLBASE_NODE_S,
    pst_node: *mut AVLBASE_NODE_S,
) {
    (*pst_node).pst_parent = pst_parent_node;
    (*pst_parent_node).pst_right = pst_node;
    (*pst_parent_node).s_r_height = 1;
    
    if pst_parent_node == (*pst_tree).pst_last {
        (*pst_tree).pst_last = pst_node;
    }
}
