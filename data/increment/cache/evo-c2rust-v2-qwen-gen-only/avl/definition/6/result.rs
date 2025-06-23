#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL3_NODE;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVL3_TREE {
    pub pstRoot: Ptr<AVL3_NODE>,
    pub pstFirst: Ptr<AVL3_NODE>,
    pub pstLast: Ptr<AVL3_NODE>,
}

pub type AVL3_TREE_ALIAS = AVL3_TREE;