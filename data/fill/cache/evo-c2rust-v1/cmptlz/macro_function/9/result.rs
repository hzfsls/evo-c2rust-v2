macro_rules! CMPTLZ_RANGE_UPDATE_0 { ($prob:expr, $range:expr, $rangeBound:expr) =>
    {
        $range = $rangeBound;
        *$prob = (*$prob + ((CMPTLZ_PROB_LG!() - *$prob) >> CMPTLZ_RANGE_CODE_SIZE!()) as CmptLzDecProb;
    }
}
pub(crate) use CMPTLZ_RANGE_UPDATE_0;
