#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBASE_NODE_S {
    pub pstParent: Ptr<AVLBASE_NODE_S>,
    pub pstLeft: Ptr<AVLBASE_NODE_S>,
    pub pstRight: Ptr<AVLBASE_NODE_S>,
    pub sLHeight: i16,
    pub sRHeight: i16,
}
