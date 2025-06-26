use std::ptr::null_mut;

// Assuming AVL_NULL_PTR is represented as a null pointer in Rust
const AVL_NULL_PTR: *mut AVL3_NODE = null_mut();

// Helper macro to find the rightmost node
macro_rules! FIND_RIGHTMOST_NODE {
    ($node:expr) => {
        while (*$node).pstRight != AVL_NULL_PTR {
            $node = (*$node).pstRight;
        }
    };
}

// Helper function to get the node start address
unsafe fn GET_NODE_START_ADDRESS(node: *mut AVL3_NODE, offset: usize) -> *mut () {
    if node.is_null() {
        return null_mut();
    }
    (node as *mut u8).sub(offset) as *mut ()
}

#[repr(C)]
struct AVL3_NODE {
    pstLeft: *mut AVL3_NODE,
    pstRight: *mut AVL3_NODE,
    pstParent: *mut AVL3_NODE,
    // Other fields...
}

#[repr(C)]
struct AVL3_TREE_INFO {
    usNodeOffset: usize,
    // Other fields...
}

unsafe fn VOS_AVL3_Prev(pstNode: *mut AVL3_NODE, pstTreeInfo: *mut AVL3_TREE_INFO) -> *mut () {
    let mut pstNodeTmp = pstNode;
    if pstNodeTmp.is_null() || pstTreeInfo.is_null() {
        return null_mut();
    }

    if (*pstNodeTmp).pstLeft != AVL_NULL_PTR {
        pstNodeTmp = (*pstNodeTmp).pstLeft;
        FIND_RIGHTMOST_NODE!(pstNodeTmp);
    } else {
        while !pstNodeTmp.is_null() {
            if (*pstNodeTmp).pstParent.is_null() || (*((*pstNodeTmp).pstParent)).pstRight == pstNodeTmp {
                pstNodeTmp = (*pstNodeTmp).pstParent;
                break;
            }
            pstNodeTmp = (*pstNodeTmp).pstParent;
        }
    }

    GET_NODE_START_ADDRESS(pstNodeTmp, (*pstTreeInfo).usNodeOffset)
}
