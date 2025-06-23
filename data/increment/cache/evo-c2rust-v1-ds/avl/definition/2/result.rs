#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avll_tree {
    pub stTree: AVL3_TREE,
    pub stTreeInfo: AVL3_TREE_INFO,
}

pub type AVLL_TREE = avll_tree;
