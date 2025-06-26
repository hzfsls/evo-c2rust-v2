macro_rules! CMPTLZ_LITERAL {
    () => {
        $crate::CMPTLZ_POSSLOT + ($crate::CMPTLZ_LEN_CONDITION_TO_POSSLOT << $crate::CMPTLZ_POS_SLOT_BITS)
    };
}

pub(crate) use CMPTLZ_LITERAL;
