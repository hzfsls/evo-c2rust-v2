#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL_NODE {
    pub pstParent: Ptr<AVL_NODE>,
    pub pstLeft: Ptr<AVL_NODE>,
    pub pstRight: Ptr<AVL_NODE>,
    pub sLHeight: i16,
    pub sRHeight: i16,
    pub pSelf: VoidPtr,
    pub pKey: VoidPtr,
}
