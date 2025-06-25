pub fn binomial_tree_merge(mut heap: Ptr<BinomialHeap>, mut tree1: Ptr<BinomialTree>, mut tree2: Ptr<BinomialTree>) -> Ptr<BinomialTree> {
    let mut new_tree: Ptr<BinomialTree>;
    let mut tmp: Ptr<BinomialTree>;
    let mut i: i32;

    if (binomial_heap_cmp(heap.cast(), tree1.value.cast(), tree2.value.cast()) > 0).as_bool() {
        tmp = tree1.cast();
        tree1 = tree2.cast();
        tree2 = tmp.cast();
    }

    new_tree = c_malloc!(c_sizeof!(BinomialTree));

    if (new_tree == NULL!()).as_bool() {
        return NULL!();
    }

    new_tree.refcount = 0;
    new_tree.order = (tree1.order + 1).cast::<u16>();

    new_tree.value = tree1.value.cast();

    new_tree.subtrees = c_malloc!(c_sizeof!(Ptr<BinomialTree>) * new_tree.order);

    if (new_tree.subtrees == NULL!()).as_bool() {
        c_free!(new_tree);
        return NULL!();
    }

    c_memcpy!(new_tree.subtrees, tree1.subtrees, c_sizeof!(Ptr<BinomialTree>) * tree1.order);
    new_tree.subtrees[new_tree.order - 1] = tree2.cast();

    c_for!(i = 0; i < new_tree.order; i.prefix_plus_plus(); {
        binomial_tree_ref(new_tree.subtrees[i].cast());
    });

    return new_tree.cast();
}
