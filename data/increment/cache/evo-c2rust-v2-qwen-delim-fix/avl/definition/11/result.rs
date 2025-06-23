#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct AVLBASE_TREE_S {
    pub pstRoot: Ptr<AVLBASE_NODE_S>,
    pub pstFirst: Ptr<AVLBASE_NODE_S>,
    pub pstLast: Ptr<AVLBASE_NODE_S>,
}
pub type AVLBASE_TREE_S_ALIAS = AVLBASE_TREE_S;