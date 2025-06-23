#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct avl3_node {
    pub pstParent: Ptr<avl3_node>,
    pub pstLeft: Ptr<avl3_node>,
    pub pstRight: Ptr<avl3_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}
pub type AVL3_NODE = avl3_node;