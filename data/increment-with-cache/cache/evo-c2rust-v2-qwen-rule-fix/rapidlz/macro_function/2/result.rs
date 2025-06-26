macro_rules! RAPIDLZ_POSITION_UPDATE { ($curDest:expr, $curSrc:expr, $len:expr) => 
    {
        $curDest += $len;
        $curSrc += $len;
    }
}
pub(crate) use RAPIDLZ_POSITION_UPDATE;