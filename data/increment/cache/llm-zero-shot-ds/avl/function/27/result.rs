use std::ptr::null_mut;

// Assuming AVL3_TREE, AVL3_NODE, AVLBASE_NODE_S, AVLBASE_TREE_S are defined elsewhere
// and VosAvlDelete is an external function that takes these types.

pub unsafe fn VOS_AVL3_Delete(pstTree: *mut AVL3_TREE, pstNode: *mut AVL3_NODE) {
    if pstTree.is_null() || pstNode.is_null() {
        return;
    }

    let pstBaseNode = pstNode as *mut AVLBASE_NODE_S;
    let pstBaseTree = pstTree as *mut AVLBASE_TREE_S;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}
