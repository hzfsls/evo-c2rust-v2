macro_rules! md5_change_state_in_turn {
    ($state:expr, $value:expr) => {
        $state[0] = $state[3];
        $state[3] = $state[2];
        $state[2] = $state[1];
        $state[1] = $state[1] + $value;
    };
}

pub(crate) use md5_change_state_in_turn;
