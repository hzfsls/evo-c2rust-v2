#[derive(Default)]
#[repr(C)]
pub struct avl3_tree {
    pub pstRoot: Ptr<AVL3_NODE>,
    pub pstFirst: Ptr<AVL3_NODE>,
    pub pstLast: Ptr<AVL3_NODE>,
}
pub type AVL3_TREE = avl3_tree;