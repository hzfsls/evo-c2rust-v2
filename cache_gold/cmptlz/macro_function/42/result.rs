macro_rules! CMPT_RC_BREAK_CHECK {
    ($rcCtx:expr, $buf:expr, $res:expr) => {
        if $buf == $rcCtx.bufBase + CMPTLZ_RC_BUFFER_SIZE!() {
            $res = CmptRcFlush64Kb($rcCtx);
            CMPTLZ_RETURN_IF_NOT_OK!($res);
        }
    }
}
pub(crate) use CMPT_RC_BREAK_CHECK;