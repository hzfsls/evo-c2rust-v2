#[repr(C)]
pub struct AVLBaseTree {
    pub pstRoot: *mut AVLBaseNode,
    pub pstFirst: *mut AVLBaseNode,
    pub pstLast: *mut AVLBaseNode,
}
