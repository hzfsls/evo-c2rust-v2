#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLL_TREE {
    pub stTree: AVL3_TREE,
    pub stTreeInfo: AVL3_TREE_INFO,
}
