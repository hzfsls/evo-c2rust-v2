#[derive(Debug)]
pub struct AvlNode {
    pub parent: Option<Box<AvlNode>>,
    pub left: Option<Box<AvlNode>>,
    pub right: Option<Box<AvlNode>>,
    pub left_height: i16,
    pub right_height: i16,
    pub self_ptr: *mut (),
    pub key_ptr: *mut (),
}
