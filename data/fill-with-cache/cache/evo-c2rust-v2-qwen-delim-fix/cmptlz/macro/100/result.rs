macro_rules! CMPT_PRIICE_TABLE_SIZE { () => { (CMPTLZ_PROB_MAX_NUM!() >> CMPT_PRICE_BITS_MOVING_NUM!()) } }
pub(crate) use CMPT_PRIICE_TABLE_SIZE;