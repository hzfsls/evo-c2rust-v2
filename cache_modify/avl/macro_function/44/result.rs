macro_rules! GET_NODE_START_ADDRESS {
    ($pstNode:expr, $usOffset:expr) => {
        if $pstNode != AVL_NULL_PTR!() {
            ($pstNode.cast::<Ptr<u8>>() - $usOffset).cast::<VoidPtr>()
        } else {
            AVL_NULL_PTR!()
        }
    }
}
pub(crate) use GET_NODE_START_ADDRESS;
