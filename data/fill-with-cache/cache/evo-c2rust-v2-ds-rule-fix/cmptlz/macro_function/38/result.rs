macro_rules! CMPT_MF_LEFT_SON_UPDATE { ($ptr1:expr, $pair:expr, $curMatch:expr, $len1:expr, $len:expr) =>
    {
        *$ptr1 = $curMatch;
        $ptr1 = $pair + 1;
        $curMatch = *$ptr1;
        $len1 = $len;
    }
}
pub(crate) use CMPT_MF_LEFT_SON_UPDATE;
