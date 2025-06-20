macro_rules! VOS_AVL3_INIT_NODE {
    ($NODE:expr) => {{
        $NODE.pstParent = AVL_NULL_PTR!();
        $NODE.pstLeft = AVL_NULL_PTR!();
        $NODE.pstRight = AVL_NULL_PTR!();
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    }};
}
pub(crate) use VOS_AVL3_INIT_NODE;