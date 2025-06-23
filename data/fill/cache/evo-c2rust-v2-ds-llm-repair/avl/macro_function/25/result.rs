macro_rules! FIND_RIGHTMOST_NODE { ($pstNode:expr) =>
    {
        while $pstNode.pstRight != AVL_NULL_PTR!()
        {
            $pstNode = $pstNode.pstRight;
        }
    }
}
pub(crate) use FIND_RIGHTMOST_NODE;
