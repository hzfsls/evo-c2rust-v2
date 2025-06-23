use std::ptr;

fn vos_avl_swap_right_most(
    pst_tree: *mut AVLBASE_TREE_S,
    pst_sub_tree: *mut AVLBASE_NODE_S,
    pst_node: *mut AVLBASE_NODE_S,
) {
    let mut pst_swap_node = pst_sub_tree;
    let pst_swap_parent;
    let pst_swap_left;

    // FIND_RIGHTMOST_NODE macro equivalent
    unsafe {
        while !(*pst_swap_node).pstRight.is_null() {
            pst_swap_node = (*pst_swap_node).pstRight;
        }
    }

    unsafe {
        if (*pst_swap_node).sRHeight != 0 || (*pst_swap_node).sLHeight > 1 {
            return;
        }

        pst_swap_parent = (*pst_swap_node).pstParent;
        pst_swap_left = (*pst_swap_node).pstLeft;

        vos_avl_update_swap_node(pst_tree, pst_swap_node, pst_node);
        vos_avl_move_node_to_new_pos(
            pst_node,
            pst_swap_parent,
            pst_swap_left,
            ptr::null_mut(),
        );

        (*pst_node).pstParent.pstRight = pst_node;
    }
}
