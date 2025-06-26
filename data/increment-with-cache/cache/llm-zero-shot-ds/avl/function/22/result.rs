use std::ptr::null_mut;

// Assuming AVLBASE_NODE_S and AVLBASE_TREE_S are defined elsewhere with the necessary fields.
// Also assuming VosAvlDeleteCheck and VosAvlBalanceTree are defined elsewhere.

pub fn vos_avl_delete(pst_base_node: *mut AVLBASE_NODE_S, pst_base_tree: *mut AVLBASE_TREE_S) {
    let mut pst_replace_node: *mut AVLBASE_NODE_S;
    let mut pst_parent_node: *mut AVLBASE_NODE_S;
    let mut s_new_height: i16 = 0;

    unsafe {
        pst_replace_node = vos_avl_delete_check(pst_base_tree, pst_base_node);

        pst_parent_node = (*pst_base_node).pstParent;

        (*pst_base_node).pstParent = null_mut();
        (*pst_base_node).pstRight = null_mut();
        (*pst_base_node).pstLeft = null_mut();
        (*pst_base_node).sRHeight = -1;
        (*pst_base_node).sLHeight = -1;

        if !pst_replace_node.is_null() {
            (*pst_replace_node).pstParent = pst_parent_node;
            s_new_height = 1 + std::cmp::max((*pst_replace_node).sLHeight, (*pst_replace_node).sRHeight);
        }

        if !pst_parent_node.is_null() {
            if (*pst_parent_node).pstRight == pst_base_node {
                (*pst_parent_node).pstRight = pst_replace_node;
                (*pst_parent_node).sRHeight = s_new_height;
            } else {
                (*pst_parent_node).pstLeft = pst_replace_node;
                (*pst_parent_node).sLHeight = s_new_height;
            }

            vos_avl_balance_tree(pst_base_tree, pst_parent_node);
        } else {
            (*pst_base_tree).pstRoot = pst_replace_node;
        }
    }
}
