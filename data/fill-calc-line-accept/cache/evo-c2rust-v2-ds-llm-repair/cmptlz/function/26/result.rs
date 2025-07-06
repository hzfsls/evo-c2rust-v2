pub fn CmptLzGetLenCondition(mut decLen: u32) -> u32 {
    return (if decLen < CMPTLZ_LEN_CONDITION_TO_POSSLOT!() { decLen } else { CMPTLZ_LEN_CONDITION_TO_POSSLOT!() - 1 } << CMPTLZ_POS_SLOT_BITS!()).cast();
}
