#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct avl_tree {
    pub pfnCompare: AVL_V2_COMPARE_FUNC,
    pub pstRoot: Ptr<AVL_NODE>,
    pub pstFirst: Ptr<AVL_NODE>,
    pub pstLast: Ptr<AVL_NODE>,
}

pub type AVL_TREE = avl_tree;
