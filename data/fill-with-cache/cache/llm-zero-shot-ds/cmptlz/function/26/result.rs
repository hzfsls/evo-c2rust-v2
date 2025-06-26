const CMPTLZ_LEN_CONDITION_TO_POSSLOT: u32 = 20;
const CMPTLZ_POS_SLOT_BITS: u32 = 6;

fn cmpt_lz_get_len_condition(dec_len: u32) -> u32 {
    ((if dec_len < CMPTLZ_LEN_CONDITION_TO_POSSLOT {
        dec_len
    } else {
        CMPTLZ_LEN_CONDITION_TO_POSSLOT - 1
    }) << CMPTLZ_POS_SLOT_BITS)
}
