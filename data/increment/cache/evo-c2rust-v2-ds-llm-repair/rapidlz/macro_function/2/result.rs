macro_rules! RAPIDLZ_POSITION_UPDATE { ($curSrc:expr, $curDest:expr, $len:expr) =>
    {
        $curDest += $len;
        $curSrc += $len;
    }
}
pub(crate) use RAPIDLZ_POSITION_UPDATE;
