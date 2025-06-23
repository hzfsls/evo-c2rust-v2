macro_rules! FIND_LEFTMOST_NODE {
    ($pstNode:expr) => {
        loop {
            if $pstNode.pstLeft != AVL_NULL_PTR!() {
                $pstNode = $pstNode.pstLeft;
            } else {
                break;
            }
        }
    };
}
pub(crate) use FIND_LEFTMOST_NODE;