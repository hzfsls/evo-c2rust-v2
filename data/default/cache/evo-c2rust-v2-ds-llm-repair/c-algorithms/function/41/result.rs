pub fn avl_tree_insert(mut tree: Ptr<AVLTree>, mut key: AVLTreeKey, mut value: AVLTreeValue) -> Ptr<AVLTreeNode> {
    let mut rover: Ptr<Ptr<AVLTreeNode>>;
    let mut new_node: Ptr<AVLTreeNode>;
    let mut previous_node: Ptr<AVLTreeNode>;

    rover = c_ref!(tree.root_node);
    previous_node = NULL!();

    while (*rover != NULL!()) {
        previous_node = *rover;
        if ((tree.compare_func)(key, (*rover).key) < 0) {
            rover = c_ref!((*rover).children[AVL_TREE_NODE_LEFT!()]);
        } else {
            rover = c_ref!((*rover).children[AVL_TREE_NODE_RIGHT!()]);
        }
    }

    new_node = c_malloc!(c_sizeof!(AVLTreeNode));

    if (new_node == NULL!()) {
        return NULL!();
    }

    new_node.children[AVL_TREE_NODE_LEFT!()] = NULL!();
    new_node.children[AVL_TREE_NODE_RIGHT!()] = NULL!();
    new_node.parent = previous_node;
    new_node.key = key;
    new_node.value = value;
    new_node.height = 1;

    *rover = new_node;

    avl_tree_balance_to_root(tree, previous_node);

    tree.num_nodes.prefix_plus_plus();

    return new_node;
}
