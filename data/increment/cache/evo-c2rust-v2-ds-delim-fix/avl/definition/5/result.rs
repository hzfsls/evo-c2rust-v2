#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL_TREE {
    pub pfnCompare: AVL_V2_COMPARE_FUNC,
    pub pstRoot: Ptr<AVL_NODE>,
    pub pstFirst: Ptr<AVL_NODE>,
    pub pstLast: Ptr<AVL_NODE>,
}
