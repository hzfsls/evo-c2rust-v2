#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_node {
    pub pstParent: Ptr<avl_node>,
    pub pstLeft: Ptr<avl_node>,
    pub pstRight: Ptr<avl_node>,
    pub sLHeight: i16,
    pub sRHeight: i16,
    pub pSelf: VoidPtr,
    pub pKey: VoidPtr,
}

pub type AVL_NODE = avl_node;
