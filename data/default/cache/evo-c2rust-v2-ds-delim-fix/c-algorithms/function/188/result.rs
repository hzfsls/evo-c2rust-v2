pub fn rb_tree_insert(mut tree: Ptr<RBTree>, mut key: RBTreeKey, mut value: RBTreeValue) -> Ptr<RBTreeNode> {
    let mut node: Ptr<RBTreeNode>;
    let mut rover: Ptr<Ptr<RBTreeNode>>;
    let mut parent: Ptr<RBTreeNode>;
    let mut side: RBTreeNodeSide;

    node = c_malloc!(c_sizeof!(RBTreeNode));

    if (node == NULL!()).as_bool() {
        return NULL!();
    }

    node.key = key.cast();
    node.value = value.cast();
    node.color = RB_TREE_NODE_RED!();
    node.children[RB_TREE_NODE_LEFT!()] = NULL!();
    node.children[RB_TREE_NODE_RIGHT!()] = NULL!();

    parent = NULL!();
    rover = c_ref!(tree.root_node).cast();

    while (*rover != NULL!()).as_bool() {
        parent = *rover.cast();

        if (tree.compare_func(value.cast(), (*rover).value.cast()) < 0).as_bool() {
            side = RB_TREE_NODE_LEFT!();
        } else {
            side = RB_TREE_NODE_RIGHT!();
        }

        rover = c_ref!((*rover).children[side]).cast();
    }

    *rover = node.cast();
    node.parent = parent.cast();

    rb_tree_insert_case1(tree.cast(), node.cast());

    tree.num_nodes.prefix_plus_plus();

    return node.cast();
}
