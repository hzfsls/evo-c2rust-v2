#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL3_NODE {
    pub pstParent: Ptr<AVL3_NODE>,
    pub pstLeft: Ptr<AVL3_NODE>,
    pub pstRight: Ptr<AVL3_NODE>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}
