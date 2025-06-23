use std::ptr;

// Assuming the following types and constants are defined elsewhere in the Rust code:
// type AVL_NODE = ...;
// const AVL_NULL_PTR: *mut AVL_NODE = ptr::null_mut();
// fn VOS_AVL_IN_TREE(node: AVL_NODE) -> bool { ... }
// macro_rules! FIND_LEFTMOST_NODE { ($node:expr) => { ... } }

pub unsafe fn VOS_AVL_Next(pstNode: *mut AVL_NODE) -> *mut AVL_NODE {
    let mut pstNodeTmp = pstNode;
    
    if pstNodeTmp.is_null() || !VOS_AVL_IN_TREE(*pstNodeTmp) {
        return ptr::null_mut();
    }
    
    if (*pstNodeTmp).pstRight != ptr::null_mut() {
        pstNodeTmp = (*pstNodeTmp).pstRight;
        FIND_LEFTMOST_NODE!(pstNodeTmp);
    } else {
        while !pstNodeTmp.is_null() {
            if (*pstNodeTmp).pstParent.is_null() || (*pstNodeTmp).pstParent.pstLeft == pstNodeTmp {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                break;
            }
            pstNodeTmp = (*pstNodeTmp).pstParent;
        }
    }
    
    if pstNodeTmp.is_null() {
        ptr::null_mut()
    } else {
        (*pstNodeTmp).pSelf
    }
}
