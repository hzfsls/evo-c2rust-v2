use std::ptr::null_mut;

pub struct AVLBASE_NODE_S {
    pstLeft: *mut AVLBASE_NODE_S,
    pstRight: *mut AVLBASE_NODE_S,
    pstParent: *mut AVLBASE_NODE_S,
}

pub struct AVLBASE_TREE_S {
    pstFirst: *mut AVLBASE_NODE_S,
    pstLast: *mut AVLBASE_NODE_S,
}

const AVL_NULL_PTR: *mut AVLBASE_NODE_S = null_mut();

pub fn VosAvlDeleteCheck(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S {
    let mut pstReplaceNode: *mut AVLBASE_NODE_S;

    unsafe {
        if (*pstNode).pstLeft == AVL_NULL_PTR && (*pstNode).pstRight == AVL_NULL_PTR {
            pstReplaceNode = AVL_NULL_PTR;

            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = (*pstNode).pstParent;
            }

            if (*pstTree).pstLast == pstNode {
                (*pstTree).pstLast = (*pstNode).pstParent;
            }
        } else if (*pstNode).pstLeft == AVL_NULL_PTR {
            pstReplaceNode = (*pstNode).pstRight;

            if (*pstTree).pstFirst == pstNode {
                (*pstTree).pstFirst = pstReplaceNode;
            }
        } else if (*pstNode).pstRight == AVL_NULL_PTR {
            pstReplaceNode = (*pstNode).pstLeft;

            if (*pstTree).pstLast == pstNode {
                (*pstTree).pstLast = pstReplaceNode;
            }
        } else {
            pstReplaceNode = VosAvlSearchReplaceNode(pstTree, pstNode);
        }
    }

    pstReplaceNode
}

// Assuming VosAvlSearchReplaceNode is defined elsewhere
extern "C" {
    fn VosAvlSearchReplaceNode(pstTree: *mut AVLBASE_TREE_S, pstNode: *mut AVLBASE_NODE_S) -> *mut AVLBASE_NODE_S;
}
