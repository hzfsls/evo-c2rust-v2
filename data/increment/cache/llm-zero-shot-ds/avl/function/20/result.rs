use super::*; // Assuming necessary imports are in the parent module

pub fn vos_avl_search_replace_node(
    pst_tree: &mut AVLBASE_TREE_S,
    pst_node: &mut AVLBASE_NODE_S,
) -> *mut AVLBASE_NODE_S {
    let mut pst_replace_node: *mut AVLBASE_NODE_S;

    if pst_node.sRHeight > pst_node.sLHeight {
        pst_replace_node = vos_avl_search_replace_node_in_r_tree(pst_tree, pst_node);
    } else {
        pst_replace_node = vos_avl_search_replace_node_in_l_tree(pst_tree, pst_node);
    }

    pst_replace_node
}
