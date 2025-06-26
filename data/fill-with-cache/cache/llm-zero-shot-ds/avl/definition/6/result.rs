#[derive(Debug)]
pub struct Avl3Node {
    pub parent: Option<Box<Avl3Node>>,
    pub left: Option<Box<Avl3Node>>,
    pub right: Option<Box<Avl3Node>>,
    pub l_height: i16,
    pub r_height: i16,
}
