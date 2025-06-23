macro_rules! RAPIDLZ_EXPAND_FORWARD {
    ($srcBegin:expr, $matchBegin:expr, $srcCurr:expr, $srcAnchor:expr) => {
        while $srcBegin < $matchBegin && $srcCurr > $srcAnchor && RAPIDLZ_UNLIKELY!($matchBegin[-1] == $srcCurr[-1]) {
            $matchBegin.minus_minus();
            $srcCurr.minus_minus();
        }
    };
}
pub(crate) use RAPIDLZ_EXPAND_FORWARD;
