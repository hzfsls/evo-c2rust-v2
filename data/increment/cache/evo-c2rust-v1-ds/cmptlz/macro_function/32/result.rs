macro_rules! CMPT_LIT_SUBCODER {
    ($probs:expr, $litCtx:expr, $lpMask:expr, $pos:expr, $prevByte:expr) => {
        $probs[((($pos) & ($lpMask)) << ($litCtx)) + (($prevByte as u32) >> (8U - ($litCtx)))]
    }
}
pub(crate) use CMPT_LIT_SUBCODER;
