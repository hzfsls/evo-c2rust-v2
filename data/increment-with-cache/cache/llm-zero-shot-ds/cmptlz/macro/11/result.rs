macro_rules! CMPT_ENC_ERROR_WRITE {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_ERROR_WRITE)
    };
}

pub(crate) use CMPT_ENC_ERROR_WRITE;
