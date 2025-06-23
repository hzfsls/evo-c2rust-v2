use std::ptr::null_mut;

// Assuming AVLBASE_NODE_S and AVLBASE_TREE_S are defined elsewhere with the same fields as in C.
// Also assuming AVL_NULL_PTR is represented as null_mut() in Rust.

pub fn vos_avl_delete_check(pst_tree: *mut AVLBASE_TREE_S, pst_node: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pst_replace_node: *mut AVLBASE_NODE_S = null_mut();
    
    unsafe {
        if (*pst_node).pstLeft.is_null() && (*pst_node).pstRight.is_null() {
            pst_replace_node = null_mut();
            if (*pst_tree).pstFirst == pst_node {
                (*pst_tree).pstFirst = (*pst_node).pstParent;
            }
            if (*pst_tree).pstLast == pst_node {
                (*pst_tree).pstLast = (*pst_node).pstParent;
            }
        } else if (*pst_node).pstLeft.is_null() {
            pst_replace_node = (*pst_node).pstRight;
            if (*pst_tree).pstFirst == pst_node {
                (*pst_tree).pstFirst = pst_replace_node;
            }
        } else if (*pst_node).pstRight.is_null() {
            pst_replace_node = (*pst_node).pstLeft;
            if (*pst_tree).pstLast == pst_node {
                (*pst_tree).pstLast = pst_replace_node;
            }
        } else {
            pst_replace_node = vos_avl_search_replace_node(pst_tree, pst_node);
        }
    }
    
    pst_replace_node
}
