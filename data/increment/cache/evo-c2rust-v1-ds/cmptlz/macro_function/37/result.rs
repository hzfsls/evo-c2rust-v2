macro_rules! CMPTLZ_RANGE_UPDATE_1 { ($prob:expr, $range:expr, $rangeCode:expr, $rangeBound:expr) =>
    {
        $range -= $rangeBound;
        $rangeCode -= $rangeBound;
        *$prob = (*$prob - (*$prob >> CMPTLZ_RANGE_CODE_SIZE!())) as CmptLzDecProb;
    }
}
pub(crate) use CMPTLZ_RANGE_UPDATE_1;
