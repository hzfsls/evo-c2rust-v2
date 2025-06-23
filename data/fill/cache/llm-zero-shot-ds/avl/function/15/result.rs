use std::ptr::null_mut;

pub fn vos_avl_swap_left_most(
    pst_tree: *mut AVLBASE_TREE_S,
    pst_sub_tree: *mut AVLBASE_NODE_S,
    pst_node: *mut AVLBASE_NODE_S,
) {
    let mut pst_swap_node = pst_sub_tree;
    
    // FIND_LEFTMOST_NODE macro equivalent
    unsafe {
        while !(*pst_swap_node).pst_left.is_null() {
            pst_swap_node = (*pst_swap_node).pst_left;
        }
    }

    unsafe {
        if (*pst_swap_node).s_l_height != 0 || (*pst_swap_node).s_r_height > 1 {
            return;
        }

        let pst_swap_parent = (*pst_swap_node).pst_parent;
        let pst_swap_right = (*pst_swap_node).pst_right;

        vos_avl_update_swap_node(pst_tree, pst_swap_node, pst_node);
        vos_avl_move_node_to_new_pos(pst_node, pst_swap_parent, null_mut(), pst_swap_right);
        
        if !(*pst_node).pst_parent.is_null() {
            (*(*pst_node).pst_parent).pst_left = pst_node;
        }
    }
}
