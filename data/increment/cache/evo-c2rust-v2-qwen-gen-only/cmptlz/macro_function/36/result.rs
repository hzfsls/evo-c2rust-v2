macro_rules! CMPTLZ_RETURN_IF_NOT_OK { ($res:expr) => 
    {
        if $res != CMPT_OK {
            return $res;
        }
    }
}
pub(crate) use CMPTLZ_RETURN_IF_NOT_OK;