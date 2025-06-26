use std::ptr::null_mut;

// Assuming AVL3_NODE and AVL3_TREE_INFO are defined elsewhere with the same fields as in C.
// Also assuming AVL_NULL_PTR is represented as a null pointer in Rust.

pub unsafe fn VOS_AVL3_Next(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut c_void {
    if pstNode.is_null() || pstTreeInfo.is_null() {
        return null_mut();
    }

    let mut pstNodeTmp = pstNode;
    
    if !(*pstNodeTmp).pstRight.is_null() {
        pstNodeTmp = (*pstNodeTmp).pstRight;
        // FIND_LEFTMOST_NODE implementation
        while !(*pstNodeTmp).pstLeft.is_null() {
            pstNodeTmp = (*pstNodeTmp).pstLeft;
        }
    } else {
        while !pstNodeTmp.is_null() {
            let parent = (*pstNodeTmp).pstParent;
            if parent.is_null() || (*parent).pstLeft == pstNodeTmp {
                pstNodeTmp = parent;
                break;
            }
            pstNodeTmp = parent;
        }
    }

    if pstNodeTmp.is_null() {
        return null_mut();
    }
    
    // GET_NODE_START_ADDRESS implementation
    let node_offset = (*pstTreeInfo).usNodeOffset;
    (pstNodeTmp as *mut u8).offset(-(node_offset as isize)) as *mut c_void
}
