#[derive(Default)]
#[repr(C)]
pub struct AVLBaseTree {
    pub pstRoot: Ptr<AVLBASE_NODE_S>,
    pub pstFirst: Ptr<AVLBASE_NODE_S>,
    pub pstLast: Ptr<AVLBASE_NODE_S>,
}
pub type AVLBASE_TREE_S = AVLBaseTree;