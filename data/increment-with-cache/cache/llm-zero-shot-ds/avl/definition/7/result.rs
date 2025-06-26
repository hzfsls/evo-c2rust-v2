#[derive(Debug)]
struct Avl3Node {
    parent: Option<Box<Avl3Node>>,
    left: Option<Box<Avl3Node>>,
    right: Option<Box<Avl3Node>>,
    l_height: i16,
    r_height: i16,
}
