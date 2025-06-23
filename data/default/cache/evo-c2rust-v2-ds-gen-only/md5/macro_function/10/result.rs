macro_rules! MD5_FUNC_H {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        $value = MD5_LINEAR_FUNC_H!($md5State[1], $md5State[2], $md5State[3]) + $md5State[0] + $text + $addEnd;
        MD5_CYCLE_MOVE!($value, $moveBit);
        MD5_CHANGE_STATE_IN_TURN!($md5State, $value);
    }
}
pub(crate) use MD5_FUNC_H;
