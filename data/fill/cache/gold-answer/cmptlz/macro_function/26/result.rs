macro_rules! CMPTLZ_RETURN_IF_NOT_OK {
    ($res:expr) => {
        if CMPTLZ_UNLIKELY!($res != CMPT_OK!()) {
            return $res;
        }
    };
}
pub(crate) use CMPTLZ_RETURN_IF_NOT_OK;