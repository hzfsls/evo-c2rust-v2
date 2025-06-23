macro_rules! VOS_AVL_INIT_NODE {
    ($NODE:expr, $SELF:expr, $KEY:expr) => {
        $NODE.pstParent = core::ptr::null_mut();
        $NODE.pstLeft = core::ptr::null_mut();
        $NODE.pstRight = core::ptr::null_mut();
        $NODE.pSelf = $SELF;
        $NODE.pKey = $KEY;
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    };
}

pub(crate) use VOS_AVL_INIT_NODE;
