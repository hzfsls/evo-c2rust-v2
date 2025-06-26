pub fn rb_tree_node_sibling(mut node: Ptr<RBTreeNode>) -> Ptr<RBTreeNode> {
    let mut side: RBTreeNodeSide;
    side = rb_tree_node_side(node.cast()).cast();
    return node.parent.children[1 - side].cast();
}
