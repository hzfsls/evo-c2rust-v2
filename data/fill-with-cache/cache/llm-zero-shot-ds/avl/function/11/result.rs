use std::ptr;

// Assuming the following types and constants are defined elsewhere in the Rust code:
// type AVLBASE_NODE_S = ...;
// const AVL_NULL_PTR: *mut AVLBASE_NODE_S = ptr::null_mut();
// fn VOS_V2_AVL_MAX(a: i32, b: i32) -> i32 { ... }

pub unsafe fn vos_avl_rotate_left(ppst_sub_tree: *mut *mut AVLBASE_NODE_S) {
    let pst_right_son = (*ppst_sub_tree).pstRight;
    (*ppst_sub_tree).pstRight = pst_right_son.pstLeft;
    
    if (*ppst_sub_tree).pstRight != AVL_NULL_PTR {
        (*ppst_sub_tree).pstRight.pstParent = *ppst_sub_tree;
    }
    
    (*ppst_sub_tree).sRHeight = pst_right_son.sLHeight;
    pst_right_son.pstParent = (*ppst_sub_tree).pstParent;
    pst_right_son.pstLeft = *ppst_sub_tree;
    pst_right_son.pstLeft.pstParent = pst_right_son;
    pst_right_son.sLHeight = 1 + VOS_V2_AVL_MAX((*ppst_sub_tree).sRHeight, (*ppst_sub_tree).sLHeight);
    
    *ppst_sub_tree = pst_right_son;
}
