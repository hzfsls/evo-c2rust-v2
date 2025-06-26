macro_rules! RAPIDLZ_EXPAND_FORWARD {
    ($srcBegin:expr, $matchBegin:expr, $srcCurr:expr, $srcAnchor:expr) => {
        while $srcBegin < $matchBegin && $srcCurr > $srcAnchor && 
              unlikely!($matchBegin[-1] == $srcCurr[-1]) {
            $matchBegin = $matchBegin.offset(-1);
            $srcCurr = $srcCurr.offset(-1);
        }
    };
}

pub(crate) use RAPIDLZ_EXPAND_FORWARD;
