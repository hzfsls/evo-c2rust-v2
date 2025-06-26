use std::ptr;

fn vos_avl_node_left_insert(
    pst_tree: &mut AVLBASE_TREE_S,
    pst_parent_node: &mut AVLBASE_NODE_S,
    pst_node: &mut AVLBASE_NODE_S,
) {
    pst_node.pst_parent = pst_parent_node;
    pst_parent_node.pst_left = pst_node;
    pst_parent_node.s_l_height = 1;
    
    if ptr::eq(pst_parent_node, pst_tree.pst_first) {
        pst_tree.pst_first = pst_node;
    }
}
