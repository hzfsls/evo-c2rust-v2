macro_rules! FIND_RIGHTMOST_NODE {
    ($pstNode:expr) => {
        loop {
            if $pstNode.pstRight != AVL_NULL_PTR!() {
                $pstNode = $pstNode.pstRight;
            } else {
                break;
            }
        }
    };
}
pub(crate) use FIND_RIGHTMOST_NODE;