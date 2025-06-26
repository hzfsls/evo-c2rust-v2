macro_rules! MD5_CYCLE_MOVE { ($numMoved:expr, $moveBit:expr) =>
    {
        let mut __tmpValue: u32;
        __tmpValue = $numMoved >> (32 - $moveBit);
        $numMoved = $numMoved << $moveBit;
        $numMoved += __tmpValue;
    }
}
pub(crate) use MD5_CYCLE_MOVE;
