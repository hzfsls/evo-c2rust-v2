macro_rules! CMPT_MF_RIGHT_SON_UPDATE { ($ptr0:expr, $pair:expr, $curMatch:expr, $len0:expr, $len:expr) =>
    {
        *$ptr0 = $curMatch;
        $ptr0 = $pair;
        $curMatch = *$ptr0;
        $len0 = $len;
    }
}
pub(crate) use CMPT_MF_RIGHT_SON_UPDATE;
