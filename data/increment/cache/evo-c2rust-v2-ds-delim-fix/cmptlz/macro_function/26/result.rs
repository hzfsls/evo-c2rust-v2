macro_rules! CMPT_STATE_UPDATE_WHEN_MATCH { ($state:expr) => { $state = if $state < 7 { LIT_MATCH } else { NOTLIT_MATCH } } }
pub(crate) use CMPT_STATE_UPDATE_WHEN_MATCH;
