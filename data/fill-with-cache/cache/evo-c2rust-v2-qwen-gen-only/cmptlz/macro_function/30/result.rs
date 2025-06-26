macro_rules! CMPT_STATE_UPDATE_WHEN_LONGREP { ($state:expr) => {
    if $state < 7 {
        LIT_LONGREP
    } else {
        NOTLIT_REP
    }
} }
pub(crate) use CMPT_STATE_UPDATE_WHEN_LONGREP;