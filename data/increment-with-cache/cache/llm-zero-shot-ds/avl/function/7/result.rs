use std::ptr;

// Assuming the following types are defined elsewhere in the Rust code:
// type AVL_TREE = ...;
// type AVL_NODE = ...;
// const AVL_NULL_PTR: *mut c_void = ptr::null_mut();

pub unsafe fn VOS_AVL_Find_Or_Find_Next(
    pstTree: *mut AVL_TREE,
    pKey: *const c_void,
    bValue: u32,
) -> *mut c_void {
    if pstTree.is_null() {
        return ptr::null_mut();
    }

    let mut pstNode = (*pstTree).pstRoot;
    let mut pFoundNode: *mut c_void = ptr::null_mut();

    if pstNode.is_null() {
        return pFoundNode;
    }

    loop {
        let iResult = ((*pstTree).pfnCompare)(pKey, (*pstNode).pKey);

        if iResult > 0 {
            if (*pstNode).pstRight.is_null() {
                pFoundNode = VOS_AVL_Next(pstNode);
                break;
            }
            pstNode = (*pstNode).pstRight;
        } else if iResult < 0 {
            if (*pstNode).pstLeft.is_null() {
                pFoundNode = (*pstNode).pSelf;
                break;
            }
            pstNode = (*pstNode).pstLeft;
        } else {
            if bValue != 0 {
                pFoundNode = VOS_AVL_Next(pstNode);
            } else {
                pFoundNode = (*pstNode).pSelf;
            }
            break;
        }
    }

    pFoundNode
}
