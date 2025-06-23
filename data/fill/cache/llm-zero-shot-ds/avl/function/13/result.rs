use std::ptr;

// Assuming AVLBASE_NODE_S is defined somewhere with these fields
struct AVLBASE_NODE_S {
    pstParent: *mut AVLBASE_NODE_S,
    pstLeft: *mut AVLBASE_NODE_S,
    pstRight: *mut AVLBASE_NODE_S,
    sLHeight: i32,
    sRHeight: i32,
}

const AVL_NULL_PTR: *mut AVLBASE_NODE_S = ptr::null_mut();

fn vos_avl_move_node_to_new_pos(
    pst_node: *mut AVLBASE_NODE_S,
    pst_new_parent: *mut AVLBASE_NODE_S,
    pst_new_left_son: *mut AVLBASE_NODE_S,
    pst_new_right_son: *mut AVLBASE_NODE_S,
) {
    unsafe {
        (*pst_node).pstParent = pst_new_parent;
        (*pst_node).pstLeft = pst_new_left_son;
        (*pst_node).pstRight = pst_new_right_son;
        (*pst_node).sLHeight = 0;
        (*pst_node).sRHeight = 0;
        
        if !pst_new_left_son.is_null() {
            (*pst_new_left_son).pstParent = pst_node;
            (*pst_node).sLHeight = 1;
        }
        
        if !pst_new_right_son.is_null() {
            (*pst_new_right_son).pstParent = pst_node;
            (*pst_node).sRHeight = 1;
        }
    }
}
