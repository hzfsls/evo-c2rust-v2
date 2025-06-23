use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in the Rust code:
// type AVL_TREE = ...;
// type AVL_NODE = ...;
// type AVLBASE_NODE_S = ...;
// type AVLBASE_TREE_S = ...;
// const AVL_NULL_PTR: *mut T = null_mut();

pub unsafe fn VOS_AVL_Delete(pstTree: *mut AVL_TREE, pstNode: *mut AVL_NODE) {
    if pstTree.is_null() || pstNode.is_null() || !VOS_AVL_IN_TREE(*pstNode) {
        return;
    }

    let pstBaseNode = pstNode as *mut AVLBASE_NODE_S;
    let pstBaseTree = &mut (*pstTree).pstRoot as *mut _ as *mut AVLBASE_TREE_S;
    VosAvlDelete(pstBaseNode, pstBaseTree);
}
