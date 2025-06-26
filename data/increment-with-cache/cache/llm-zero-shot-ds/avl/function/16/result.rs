use std::ptr;

// Assuming the following struct definitions are available:
// #[repr(C)]
// struct AVLBASE_NODE_S {
//     sRHeight: i32,
//     sLHeight: i32,
//     pstRight: *mut AVLBASE_NODE_S,
//     pstLeft: *mut AVLBASE_NODE_S,
// }

// Also assuming these functions are defined elsewhere:
// fn VosAvlRotateLeft(ppstSubTree: *mut *mut AVLBASE_NODE_S);
// fn VosAvlRotateRight(ppstSubTree: *mut *mut AVLBASE_NODE_S);

pub unsafe fn VosAvlRebalance(ppstSubTree: *mut *mut AVLBASE_NODE_S) {
    let iMoment = (**ppstSubTree).sRHeight - (**ppstSubTree).sLHeight;

    if iMoment > 1 {
        if (**ppstSubTree).pstRight.as_ref().unwrap().sLHeight > 
           (**ppstSubTree).pstRight.as_ref().unwrap().sRHeight {
            VosAvlRotateRight(&mut (**ppstSubTree).pstRight);
        }
        VosAvlRotateLeft(ppstSubTree);
    } else if iMoment < -1 {
        if (**ppstSubTree).pstLeft.as_ref().unwrap().sRHeight > 
           (**ppstSubTree).pstLeft.as_ref().unwrap().sLHeight {
            VosAvlRotateLeft(&mut (**ppstSubTree).pstLeft);
        }
        VosAvlRotateRight(ppstSubTree);
    }
}
