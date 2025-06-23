macro_rules! CMPT_STATE_UPDATE_WHEN_SHORTREP {
    ($state:expr) => {
        $state = if $state < 7 { LIT_SHORTREP } else { NOTLIT_REP }
    };
}

pub(crate) use CMPT_STATE_UPDATE_WHEN_SHORTREP;
