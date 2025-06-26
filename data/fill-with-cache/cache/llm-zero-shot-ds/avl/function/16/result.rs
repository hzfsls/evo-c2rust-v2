use std::ptr;

// Assuming the AVLBASE_NODE_S struct is defined somewhere like this:
// #[repr(C)]
// struct AVLBASE_NODE_S {
//     sLHeight: i32,
//     sRHeight: i32,
//     pstLeft: *mut AVLBASE_NODE_S,
//     pstRight: *mut AVLBASE_NODE_S,
// }

pub unsafe fn vos_avl_rebalance(ppst_subtree: *mut *mut AVLBASE_NODE_S) {
    let subtree = *ppst_subtree;
    let i_moment = (*subtree).sRHeight - (*subtree).sLHeight;
    
    if i_moment > 1 {
        let right = (*subtree).pstRight;
        if (*right).sLHeight > (*right).sRHeight {
            vos_avl_rotate_right(&mut (*subtree).pstRight);
        }
        vos_avl_rotate_left(ppst_subtree);
    } else if i_moment < -1 {
        let left = (*subtree).pstLeft;
        if (*left).sRHeight > (*left).sLHeight {
            vos_avl_rotate_left(&mut (*subtree).pstLeft);
        }
        vos_avl_rotate_right(ppst_subtree);
    }
}
