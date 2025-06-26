use std::ptr::null_mut;

// Assuming the following types are defined elsewhere in Rust:
// type AVL3_TREE = ...;
// type AVL3_NODE = ...;
// type AVL3_TREE_INFO = ...;
// type AVLBASE_TREE_S = ...;
// type AVLBASE_NODE_S = ...;

// Constants
const AVL_NULL_PTR: *mut () = null_mut();

// Helper macros converted to functions
fn tree_or_treeinfo_is_null(tree: *const AVL3_TREE, tree_info: *const AVL3_TREE_INFO) -> bool {
    tree.is_null() || tree_info.is_null()
}

fn get_keyoffset(tree_info: *const AVL3_TREE_INFO) -> usize {
    unsafe { (*tree_info).usKeyOffset }
}

// External functions (assuming they exist)
extern "C" {
    fn VosAvlNodeRightInsert(tree: *mut AVLBASE_TREE_S, parent: *mut AVLBASE_NODE_S, node: *mut AVLBASE_NODE_S);
    fn VosAvlNodeLeftInsert(tree: *mut AVLBASE_TREE_S, parent: *mut AVLBASE_NODE_S, node: *mut AVLBASE_NODE_S);
    fn VosAvlBalanceTree(tree: *mut AVLBASE_TREE_S, node: *mut AVLBASE_NODE_S);
}

pub unsafe fn vos_avl3_insert_or_find(
    pst_tree: *mut AVL3_TREE,
    pst_node: *mut AVL3_NODE,
    pst_tree_info: *mut AVL3_TREE_INFO,
) -> *mut () {
    if tree_or_treeinfo_is_null(pst_tree, pst_tree_info) || pst_node.is_null() {
        return AVL_NULL_PTR;
    }

    (*pst_node).sRHeight = 0;
    (*pst_node).sLHeight = 0;

    if (*pst_tree).pstRoot.is_null() {
        (*pst_tree).pstRoot = pst_node;
        (*pst_tree).pstFirst = pst_node;
        (*pst_tree).pstLast = pst_node;
        return AVL_NULL_PTR;
    }

    let mut pst_parent_node = (*pst_tree).pstRoot;
    let i_key_offset = get_keyoffset(pst_tree_info);

    while !pst_parent_node.is_null() {
        let i_result = ((*pst_tree_info).pfCompare)(
            ((pst_node as *mut u8).add(i_key_offset)) as *mut _,
            ((pst_parent_node as *mut u8).add(i_key_offset)) as *mut _,
        );

        if i_result > 0 {
            if !(*pst_parent_node).pstRight.is_null() {
                pst_parent_node = (*pst_parent_node).pstRight;
                continue;
            }
            VosAvlNodeRightInsert(
                pst_tree as *mut AVLBASE_TREE_S,
                pst_parent_node as *mut AVLBASE_NODE_S,
                pst_node as *mut AVLBASE_NODE_S,
            );
        } else if i_result < 0 {
            if !(*pst_parent_node).pstLeft.is_null() {
                pst_parent_node = (*pst_parent_node).pstLeft;
                continue;
            }
            VosAvlNodeLeftInsert(
                pst_tree as *mut AVLBASE_TREE_S,
                pst_parent_node as *mut AVLBASE_NODE_S,
                pst_node as *mut AVLBASE_NODE_S,
            );
        } else {
            (*pst_node).sRHeight = -1;
            (*pst_node).sLHeight = -1;
            return ((pst_parent_node as *mut u8).offset(-((*pst_tree_info).usNodeOffset as isize))) as *mut _;
        }
        break;
    }

    VosAvlBalanceTree(
        pst_tree as *mut AVLBASE_TREE_S,
        pst_parent_node as *mut AVLBASE_NODE_S,
    );
    AVL_NULL_PTR
}
