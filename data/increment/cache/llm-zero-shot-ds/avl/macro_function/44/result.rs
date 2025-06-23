macro_rules! GET_NODE_START_ADDRESS {
    ($pstNode:expr, $usOffset:expr) => {
        if $pstNode.is_null() {
            std::ptr::null()
        } else {
            (($pstNode as *const u8).wrapping_offset(-($usOffset as isize))) as *const _
        }
    };
}
pub(crate) use GET_NODE_START_ADDRESS;
