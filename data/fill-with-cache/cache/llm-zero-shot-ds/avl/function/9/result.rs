use std::ptr;

fn vos_avl_node_left_insert(
    pst_tree: &mut AVLBASE_TREE_S,
    pst_parent_node: &mut AVLBASE_NODE_S,
    pst_node: &mut AVLBASE_NODE_S,
) {
    pst_node.pstParent = pst_parent_node as *mut AVLBASE_NODE_S;
    pst_parent_node.pstLeft = pst_node as *mut AVLBASE_NODE_S;
    pst_parent_node.sLHeight = 1;
    
    if ptr::eq(pst_parent_node, pst_tree.pstFirst) {
        pst_tree.pstFirst = pst_node as *mut AVLBASE_NODE_S;
    }
}
