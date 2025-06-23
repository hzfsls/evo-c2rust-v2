#[derive(Debug)]
struct AVLBaseNode {
    parent: Option<Box<AVLBaseNode>>,
    left: Option<Box<AVLBaseNode>>,
    right: Option<Box<AVLBaseNode>>,
    l_height: i16,
    r_height: i16,
}
