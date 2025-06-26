use std::ptr;

// Assuming the following types are defined elsewhere in Rust:
// type AVL_TREE = ...;
// type AVL_NODE = ...;
// const AVL_NULL_PTR: *mut AVL_NODE = ptr::null_mut();

pub unsafe fn VOS_AVL_Find(pstTree: *mut AVL_TREE, pKey: *const ()) -> *mut () {
    if pstTree.is_null() {
        return ptr::null_mut();
    }

    let mut pstNode = (*pstTree).pstRoot;
    while !pstNode.is_null() {
        let iResult = ((*pstTree).pfnCompare)(pKey, (*pstNode).pKey);
        if iResult > 0 {
            pstNode = (*pstNode).pstRight;
        } else if iResult < 0 {
            pstNode = (*pstNode).pstLeft;
        } else {
            break;
        }
    }

    if !pstNode.is_null() {
        (*pstNode).pSelf
    } else {
        ptr::null_mut()
    }
}
