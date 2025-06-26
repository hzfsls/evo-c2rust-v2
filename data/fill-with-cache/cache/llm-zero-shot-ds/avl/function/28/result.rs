use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in the Rust code:
// type AVL_TREE = ...;
// type AVL_NODE = ...;
// const AVL_NULL_PTR: *mut () = null_mut();

pub unsafe extern "C" fn VOS_AVL_Find_Or_Find_Next(
    pstTree: *mut AVL_TREE,
    pKey: *const (),
    bValue: u32,
) -> *mut () {
    if pstTree.is_null() {
        return AVL_NULL_PTR;
    }

    let mut pstNode = (*pstTree).pstRoot;
    if pstNode.is_null() {
        return AVL_NULL_PTR;
    }

    let mut pFoundNode = AVL_NULL_PTR;

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
