use std::ptr;

// Assuming the following type definitions:
// type AVLBASE_NODE_S = ...;
// const AVL_NULL_PTR: *mut AVLBASE_NODE_S = ptr::null_mut();

pub fn vos_avl_rotate_right(ppst_sub_tree: &mut *mut AVLBASE_NODE_S) {
    unsafe {
        let pst_left_son = (**ppst_sub_tree).pstLeft;
        (**ppst_sub_tree).pstLeft = (*pst_left_son).pstRight;
        
        if !(**ppst_sub_tree).pstLeft.is_null() {
            (*(**ppst_sub_tree).pstLeft).pstParent = *ppst_sub_tree;
        }
        
        (**ppst_sub_tree).sLHeight = (*pst_left_son).sRHeight;
        (*pst_left_son).pstParent = (**ppst_sub_tree).pstParent;
        (*pst_left_son).pstRight = *ppst_sub_tree;
        (*(*pst_left_son).pstRight).pstParent = pst_left_son;
        
        (*pst_left_son).sRHeight = 1 + std::cmp::max(
            (**ppst_sub_tree).sRHeight,
            (**ppst_sub_tree).sLHeight
        );
        
        *ppst_sub_tree = pst_left_son;
    }
}
