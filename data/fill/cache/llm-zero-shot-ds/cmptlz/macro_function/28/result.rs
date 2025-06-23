macro_rules! cmpt_state_update_when_lit {
    ($state:expr) => {
        $state = if $state <= SHORTREP_LIT_LIT {
            LIT_LIT
        } else if $state <= LIT_SHORTREP {
            $state - 3
        } else {
            $state - 6
        }
    };
}

pub(crate) use cmpt_state_update_when_lit;
