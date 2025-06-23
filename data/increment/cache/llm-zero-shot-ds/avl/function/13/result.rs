use std::ptr;

// Assuming AVLBASE_NODE_S is defined somewhere with these fields
struct AVLBASE_NODE_S {
    pstParent: *mut AVLBASE_NODE_S,
    pstLeft: *mut AVLBASE_NODE_S,
    pstRight: *mut AVLBASE_NODE_S,
    sLHeight: i32,
    sRHeight: i32,
}

// Assuming AVL_NULL_PTR is defined as a null pointer
const AVL_NULL_PTR: *mut AVLBASE_NODE_S = ptr::null_mut();

unsafe fn VosAvlMoveNodeToNewPos(
    pstNode: *mut AVLBASE_NODE_S,
    pstNewParent: *mut AVLBASE_NODE_S,
    pstNewLeftSon: *mut AVLBASE_NODE_S,
    pstNewRightSon: *mut AVLBASE_NODE_S,
) {
    (*pstNode).pstParent = pstNewParent;
    (*pstNode).pstLeft = pstNewLeftSon;
    (*pstNode).pstRight = pstNewRightSon;
    (*pstNode).sLHeight = 0;
    (*pstNode).sRHeight = 0;

    if pstNewLeftSon != AVL_NULL_PTR {
        (*pstNewLeftSon).pstParent = pstNode;
        (*pstNode).sLHeight = 1;
    }

    if pstNewRightSon != AVL_NULL_PTR {
        (*pstNewRightSon).pstParent = pstNode;
        (*pstNode).sRHeight = 1;
    }
}
