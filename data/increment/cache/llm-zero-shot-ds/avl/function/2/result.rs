use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in Rust:
// type AVL_TREE = ...;
// type AVL_NODE = ...;
// type AVLBASE_TREE_S = ...;
// type AVLBASE_NODE_S = ...;
// const AVL_NULL_PTR: *mut AVL_NODE = null_mut();

// Helper functions (assuming they are defined elsewhere):
// fn VOS_AVL_IN_TREE(node: AVL_NODE) -> bool { ... }
// fn VosAvlNodeRightInsert(tree: *mut AVLBASE_TREE_S, parent: *mut AVLBASE_NODE_S, node: *mut AVLBASE_NODE_S) { ... }
// fn VosAvlNodeLeftInsert(tree: *mut AVLBASE_TREE_S, parent: *mut AVLBASE_NODE_S, node: *mut AVLBASE_NODE_S) { ... }
// fn VosAvlBalanceTree(tree: *mut AVLBASE_TREE_S, node: *mut AVLBASE_NODE_S) { ... }

pub unsafe fn VOS_AVL_Insert_Or_Find(pstTree: *mut AVL_TREE, pstNode: *mut AVL_NODE) -> *mut AVL_NODE {
    if pstTree.is_null() || pstNode.is_null() || VOS_AVL_IN_TREE(*pstNode) {
        return AVL_NULL_PTR;
    }

    (*pstNode).sRHeight = 0;
    (*pstNode).sLHeight = 0;

    if (*pstTree).pstRoot.is_null() {
        (*pstTree).pstRoot = pstNode;
        (*pstTree).pstFirst = pstNode;
        (*pstTree).pstLast = pstNode;
        return AVL_NULL_PTR;
    }

    let mut pstParentNode = (*pstTree).pstRoot;
    while !pstParentNode.is_null() {
        let iResult = ((*pstTree).pfnCompare)((*pstNode).pKey, (*pstParentNode).pKey);
        
        if iResult > 0 {
            if !(*pstParentNode).pstRight.is_null() {
                pstParentNode = (*pstParentNode).pstRight;
                continue;
            }

            VosAvlNodeRightInsert(
                &mut (*pstTree).pstRoot as *mut _ as *mut AVLBASE_TREE_S,
                pstParentNode as *mut AVLBASE_NODE_S,
                pstNode as *mut AVLBASE_NODE_S,
            );
            break;
        } else if iResult < 0 {
            if !(*pstParentNode).pstLeft.is_null() {
                pstParentNode = (*pstParentNode).pstLeft;
                continue;
            }

            VosAvlNodeLeftInsert(
                &mut (*pstTree).pstRoot as *mut _ as *mut AVLBASE_TREE_S,
                pstParentNode as *mut AVLBASE_NODE_S,
                pstNode as *mut AVLBASE_NODE_S,
            );
            break;
        } else {
            (*pstNode).sRHeight = -1;
            (*pstNode).sLHeight = -1;
            return (*pstParentNode).pSelf;
        }
    }

    if !pstParentNode.is_null() {
        VosAvlBalanceTree(
            &mut (*pstTree).pstRoot as *mut _ as *mut AVLBASE_TREE_S,
            pstParentNode as *mut AVLBASE_NODE_S,
        );
    }

    AVL_NULL_PTR
}
