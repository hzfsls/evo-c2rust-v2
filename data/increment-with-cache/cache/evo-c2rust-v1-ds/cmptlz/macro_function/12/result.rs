macro_rules! CMPT_RC_BREAK_SHIFTING { ($rcCtx:expr, $buf:expr, $res:expr) =>
    {
        CMPT_RC_BREAK_CHECK!($rcCtx, $buf, $res);
        if $rcCtx.cacheSize == 0
        {
            return CMPT_OK;
        }
    }
}
pub(crate) use CMPT_RC_BREAK_SHIFTING;
