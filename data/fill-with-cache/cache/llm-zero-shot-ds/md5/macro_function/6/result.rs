macro_rules! MD5_CYCLE_MOVE {
    ($numMoved:expr, $moveBit:expr) => {
        let __tmp_value = $numMoved >> (32 - $moveBit);
        $numMoved = $numMoved << $moveBit;
        $numMoved += __tmp_value;
    };
}

pub(crate) use MD5_CYCLE_MOVE;
