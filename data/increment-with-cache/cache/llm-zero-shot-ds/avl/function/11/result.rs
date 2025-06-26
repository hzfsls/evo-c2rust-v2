use std::ptr;

// Assuming AVLBASE_NODE_S and AVL_NULL_PTR are defined somewhere in the Rust code.
// For the sake of this translation, let's define them as follows:
// type AVLBASE_NODE_S = ...;
// const AVL_NULL_PTR: *mut AVLBASE_NODE_S = ptr::null_mut();

pub fn vos_avl_rotate_left(ppst_sub_tree: &mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pst_right_son = (*ppst_sub_tree).pst_right;

        (*ppst_sub_tree).pst_right = pst_right_son.pst_left;
        if (*ppst_sub_tree).pst_right != AVL_NULL_PTR {
            (*ppst_sub_tree).pst_right.pst_parent = *ppst_sub_tree;
        }

        (*ppst_sub_tree).s_r_height = pst_right_son.s_l_height;
        pst_right_son.pst_parent = (*ppst_sub_tree).pst_parent;
        pst_right_son.pst_left = *ppst_sub_tree;
        pst_right_son.pst_left.pst_parent = pst_right_son;
        pst_right_son.s_l_height = 1 + vos_v2_avl_max((*ppst_sub_tree).s_r_height, (*ppst_sub_tree).s_l_height);

        *ppst_sub_tree = pst_right_son;
    }
}

// Helper function (assuming it's defined elsewhere)
fn vos_v2_avl_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
