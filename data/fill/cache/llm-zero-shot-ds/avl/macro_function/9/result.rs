macro_rules! VOS_AVL3_INIT_NODE {
    ($NODE:expr) => {
        $NODE.pstParent = core::ptr::null_mut();
        $NODE.pstLeft = core::ptr::null_mut();
        $NODE.pstRight = core::ptr::null_mut();
        $NODE.sLHeight = -1;
        $NODE.sRHeight = -1;
    };
}

pub(crate) use VOS_AVL3_INIT_NODE;
