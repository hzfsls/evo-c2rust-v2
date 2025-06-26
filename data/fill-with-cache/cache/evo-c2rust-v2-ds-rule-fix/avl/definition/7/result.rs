#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBaseNode {
    pub pstParent: Ptr<AVLBaseNode>,
    pub pstLeft: Ptr<AVLBaseNode>,
    pub pstRight: Ptr<AVLBaseNode>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}

pub type AVLBASE_NODE_S = AVLBaseNode;
