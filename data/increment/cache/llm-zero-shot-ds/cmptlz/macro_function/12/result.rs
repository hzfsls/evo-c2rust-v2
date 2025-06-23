macro_rules! cmpt_rc_break_shifting {
    ($rcCtx:expr, $buf:expr, $res:expr) => {
        cmpt_rc_break_check!($rcCtx, $buf, $res);
        if $rcCtx.cacheSize == 0 {
            return $crate::CmptOk;
        }
    };
}

pub(crate) use cmpt_rc_break_shifting;
