macro_rules! FIND_LEFTMOST_NODE { ($pstNode:expr) =>
    {
        while $pstNode.pstLeft != AVL_NULL_PTR!()
        {
            $pstNode = $pstNode.pstLeft;
        }
    }
}
pub(crate) use FIND_LEFTMOST_NODE;
