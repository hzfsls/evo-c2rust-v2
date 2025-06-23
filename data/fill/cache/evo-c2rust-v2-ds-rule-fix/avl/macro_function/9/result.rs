macro_rules! VOS_AVL3_INIT_NODE { ($NODE:expr) =>
    {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL3_NODE>>();
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    }
}
pub(crate) use VOS_AVL3_INIT_NODE;
