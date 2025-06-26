macro_rules! CMPT_RC_GET_NEWBOUND { ($prob:expr, $bit0Prob:expr, $range:expr, $newBound:expr) =>
    {
        $bit0Prob = *$prob;
        $newBound = ($range >> 11) * $bit0Prob;
    }
}
pub(crate) use CMPT_RC_GET_NEWBOUND;
