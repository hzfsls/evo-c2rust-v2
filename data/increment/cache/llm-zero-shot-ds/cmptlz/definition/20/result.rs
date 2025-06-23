#[repr(C)]
pub struct CmptLenEncoder {
    pub low: [CmptlzProb; 256],
    pub high: [CmptlzProb; 1 << CMPT_LEN_HIGH_BITS],
    pub prices: [[u32; (1 << CMPT_LEN_HIGH_BITS) + (1 << CMPT_LEN_MID_BITS) + (1 << CMPT_LEN_LOW_BITS)]; CMPTLZ_NUM_PB_STATES_MAX],
    pub tableSize: u32,
}
