#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL3_TREE_INFO {
    pub pfCompare: AVL3_COMPARE,
    pub usKeyOffset: u16,
    pub usNodeOffset: u16,
}
