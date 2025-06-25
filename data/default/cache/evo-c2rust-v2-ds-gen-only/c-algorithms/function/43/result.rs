pub fn avl_tree_remove_node(mut tree: Ptr<AVLTree>, mut node: Ptr<AVLTreeNode>) {
    let mut swap_node: Ptr<AVLTreeNode> = Default::default();
    let mut balance_startpoint: Ptr<AVLTreeNode> = Default::default();
    let mut i: i32 = Default::default();

    swap_node = avl_tree_node_get_replacement(tree.cast(), node.cast()).cast();

    if (swap_node == NULL!()).as_bool() {
        avl_tree_node_replace(tree.cast(), node.cast(), NULL!().cast());

        balance_startpoint = node.parent.cast();
    } else {
        if (swap_node.parent == node).as_bool() {
            balance_startpoint = swap_node.cast();
        } else {
            balance_startpoint = swap_node.parent.cast();
        }

        c_for!(i = 0; i < 2; i.prefix_plus_plus(); {
            swap_node.children[i] = node.children[i].cast();

            if (swap_node.children[i] != NULL!()).as_bool() {
                swap_node.children[i].parent = swap_node.cast();
            }
        });

        swap_node.height = node.height.cast();

        avl_tree_node_replace(tree.cast(), node.cast(), swap_node.cast());
    }

    c_free!(node);

    tree.num_nodes -= 1;

    avl_tree_balance_to_root(tree.cast(), balance_startpoint.cast());
}
