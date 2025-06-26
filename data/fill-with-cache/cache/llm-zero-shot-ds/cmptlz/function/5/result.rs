use std::mem::size_of;

const CMPTLZ_PROPS_SIZE: usize = 5;
const CMPTLZ_DICT_MIN_LEN: u32 = 1 << 12;
const CMPTLZ_LIT_CTX_MAX: u8 = 8;
const CMPTLZ_POS_STATE_MAX: u8 = 4;
const CMPTLZ_LIT_POS_MAX: u8 = CMPTLZ_POS_STATE_MAX * 8;
const CMPT_OK: i32 = 0;
const CMPT_ERROR_UNSUPPORTED: i32 = -1;

#[repr(C)]
pub struct CmptLzDecProt {
    pub dicSize: u32,
    pub litCtx: u8,
    pub posBits: u8,
    pub litPos: u8,
}

pub fn cmpt_lz_props_decode(prot_data: &[u8], dec_prot: &mut CmptLzDecProt) -> i32 {
    if prot_data.len() < CMPTLZ_PROPS_SIZE {
        return CMPT_ERROR_UNSUPPORTED;
    }

    let dict_size = u32::from(prot_data[1])
        | (u32::from(prot_data[2]) << 8)
        | (u32::from(prot_data[3]) << 16)
        | (u32::from(prot_data[4]) << 24);

    let dict_size = if dict_size < CMPTLZ_DICT_MIN_LEN {
        CMPTLZ_DICT_MIN_LEN
    } else {
        dict_size
    };

    dec_prot.dicSize = dict_size;

    let first_data = prot_data[0];
    if first_data >= (CMPTLZ_LIT_CTX_MAX * CMPTLZ_POS_STATE_MAX * CMPTLZ_LIT_POS_MAX) {
        return CMPT_ERROR_UNSUPPORTED;
    }

    dec_prot.litCtx = first_data % CMPTLZ_LIT_CTX_MAX;
    let first_data = first_data / CMPTLZ_LIT_CTX_MAX;
    dec_prot.posBits = first_data / CMPTLZ_POS_STATE_MAX;
    dec_prot.litPos = first_data % CMPTLZ_LIT_POS_MAX;

    CMPT_OK
}
