macro_rules! MD5_CHANGE_STATE_IN_TURN { ($state:expr, $value:expr) =>
    {
        $state[0] = $state[3];
        $state[3] = $state[2];
        $state[2] = $state[1];
        $state[1] = $state[1] + $value;
    }
}
pub(crate) use MD5_CHANGE_STATE_IN_TURN;
