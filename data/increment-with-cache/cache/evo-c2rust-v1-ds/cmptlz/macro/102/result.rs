macro_rules! CMPTLZ_HIGH_LENPROB_OFFSET { () => { CMPTLZ_LOW_LENPROB_OFFSET!() + ((1 << CMPTLZ_PB_BITS_MAX!()) << (CMPTLZ_LOW_LEN_BIT!() + 1)) } }
