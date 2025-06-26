use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in the Rust code:
// type AVLBASE_NODE_S = ...;
// type AVLBASE_TREE_S = ...;
// const AVL_NULL_PTR: *mut AVLBASE_NODE_S = null_mut();

pub fn vos_avl_delete(pst_base_node: *mut AVLBASE_NODE_S, pst_base_tree: *mut AVLBASE_TREE_S) {
    unsafe {
        let mut pst_replace_node = vos_avl_delete_check(pst_base_tree, pst_base_node);
        let pst_parent_node = (*pst_base_node).pst_parent;
        
        // Reset the node being deleted
        (*pst_base_node).pst_parent = AVL_NULL_PTR;
        (*pst_base_node).pst_right = AVL_NULL_PTR;
        (*pst_base_node).pst_left = AVL_NULL_PTR;
        (*pst_base_node).s_r_height = -1;
        (*pst_base_node).s_l_height = -1;
        
        let mut s_new_height = 0;
        
        if !pst_replace_node.is_null() {
            (*pst_replace_node).pst_parent = pst_parent_node;
            s_new_height = 1 + std::cmp::max(
                (*pst_replace_node).s_l_height,
                (*pst_replace_node).s_r_height
            );
        }
        
        if !pst_parent_node.is_null() {
            if (*pst_parent_node).pst_right == pst_base_node {
                (*pst_parent_node).pst_right = pst_replace_node;
                (*pst_parent_node).s_r_height = s_new_height;
            } else {
                (*pst_parent_node).pst_left = pst_replace_node;
                (*pst_parent_node).s_l_height = s_new_height;
            }
            vos_avl_balance_tree(pst_base_tree, pst_parent_node);
        } else {
            (*pst_base_tree).pst_root = pst_replace_node;
        }
    }
}
