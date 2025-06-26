macro_rules! CMPT_STATE_UPDATE_WHEN_LIT { ($state:expr) => 
    {
        $state = if $state <= SHORTREP_LIT_LIT {
            LIT_LIT
        } else if $state <= LIT_SHORTREP {
            $state - 3
        } else {
            $state - 6
        }
    }
}
pub(crate) use CMPT_STATE_UPDATE_WHEN_LIT;
