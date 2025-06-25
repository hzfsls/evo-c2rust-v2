#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct _AVLTreeNode {
    pub children: Array<Ptr<AVLTreeNode>, 2>,
    pub parent: Ptr<AVLTreeNode>,
    pub key: AVLTreeKey,
    pub value: AVLTreeValue,
    pub height: i32,
}
