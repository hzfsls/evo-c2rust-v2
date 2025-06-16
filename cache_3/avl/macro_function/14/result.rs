macro_rules! VOS_AVL_INIT_NODE {
    ($NODE:expr, $SELF:expr, $KEY:expr) => {
        $NODE.pstParent = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstLeft = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pstRight = AVL_NULL_PTR!().cast::<Ptr<AVL_NODE>>();
        $NODE.pSelf = $SELF;
        $NODE.pKey = $KEY;
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    };
}
pub(crate) use VOS_AVL_INIT_NODE;
