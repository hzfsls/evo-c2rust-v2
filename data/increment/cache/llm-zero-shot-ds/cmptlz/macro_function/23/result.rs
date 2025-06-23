macro_rules! cmptlz_return_if_not_ok {
    ($res:expr) => {
        if crate::unlikely!($res != $crate::CmptOk) {
            return $res;
        }
    };
}

pub(crate) use cmptlz_return_if_not_ok;
