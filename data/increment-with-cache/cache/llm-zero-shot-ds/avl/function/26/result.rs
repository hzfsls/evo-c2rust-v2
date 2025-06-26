use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in the Rust codebase:
// - AVL3_TREE
// - AVL3_NODE
// - AVL3_TREE_INFO
// - AVLBASE_TREE_S
// - AVLBASE_NODE_S
// And the following constants:
// - AVL_NULL_PTR (null_mut())
// And the following functions:
// - TREE_OR_TREEINFO_IS_NULL
// - GET_KEYOFFSET
// - VosAvlNodeRightInsert
// - VosAvlNodeLeftInsert
// - VosAvlBalanceTree

pub unsafe extern "C" fn VOS_AVL3_Insert_Or_Find(
    pstTree: *mut AVL3_TREE,
    pstNode: *mut AVL3_NODE,
    pstTreeInfo: *mut AVL3_TREE_INFO,
) -> *mut c_void {
    if TREE_OR_TREEINFO_IS_NULL(pstTree, pstTreeInfo) || pstNode.is_null() {
        return null_mut();
    }

    (*pstNode).sRHeight = 0;
    (*pstNode).sLHeight = 0;

    if (*pstTree).pstRoot.is_null() {
        (*pstTree).pstRoot = pstNode;
        (*pstTree).pstFirst = pstNode;
        (*pstTree).pstLast = pstNode;
        return null_mut();
    }

    let mut pstParentNode = (*pstTree).pstRoot;
    let iKeyOffset = GET_KEYOFFSET(pstTreeInfo);

    while !pstParentNode.is_null() {
        let node_key = (pstNode as *mut u8).add(iKeyOffset) as *mut c_void;
        let parent_key = (pstParentNode as *mut u8).add(iKeyOffset) as *mut c_void;
        let iResult = ((*pstTreeInfo).pfCompare)(node_key, parent_key);

        if iResult > 0 {
            if !(*pstParentNode).pstRight.is_null() {
                pstParentNode = (*pstParentNode).pstRight;
                continue;
            }
            VosAvlNodeRightInsert(pstTree as *mut AVLBASE_TREE_S, pstParentNode as *mut AVLBASE_NODE_S, pstNode as *mut AVLBASE_NODE_S);
        } else if iResult < 0 {
            if !(*pstParentNode).pstLeft.is_null() {
                pstParentNode = (*pstParentNode).pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert(pstTree as *mut AVLBASE_TREE_S, pstParentNode as *mut AVLBASE_NODE_S, pstNode as *mut AVLBASE_NODE_S);
        } else {
            (*pstNode).sRHeight = -1;
            (*pstNode).sLHeight = -1;
            return (pstParentNode as *mut u8).sub((*pstTreeInfo).usNodeOffset) as *mut c_void;
        }

        break;
    }

    VosAvlBalanceTree(pstTree as *mut AVLBASE_TREE_S, pstParentNode as *mut AVLBASE_NODE_S);
    null_mut()
}
