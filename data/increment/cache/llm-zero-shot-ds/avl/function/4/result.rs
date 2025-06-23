use std::ptr;

pub unsafe fn vos_avl_find<T>(pst_tree: *const AVLTree<T>, p_key: *const T) -> *mut T {
    if pst_tree.is_null() {
        return ptr::null_mut();
    }

    let mut pst_node = (*pst_tree).pst_root;

    while !pst_node.is_null() {
        let i_result = ((*pst_tree).pfn_compare)(p_key, (*pst_node).p_key);

        if i_result > 0 {
            pst_node = (*pst_node).pst_right;
        } else if i_result < 0 {
            pst_node = (*pst_node).pst_left;
        } else {
            break;
        }
    }

    if !pst_node.is_null() {
        (*pst_node).p_self
    } else {
        ptr::null_mut()
    }
}
