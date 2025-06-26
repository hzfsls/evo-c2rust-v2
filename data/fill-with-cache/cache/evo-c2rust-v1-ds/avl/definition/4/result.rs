#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl3_tree_info {
    pub pfCompare: AVL3_COMPARE,
    pub usKeyOffset: u16,
    pub usNodeOffset: u16,
}

pub type AVL3_TREE_INFO = avl3_tree_info;
