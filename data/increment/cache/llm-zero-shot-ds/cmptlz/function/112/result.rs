use std::mem::size_of;

const CMPTLZ_PROPS_SIZE: usize = 5;
const CMPTLZ_DICT_MIN_LEN: u32 = 1 << 12;
const CMPTLZ_LIT_CTX_MAX: u8 = 8;
const CMPTLZ_POS_STATE_MAX: u8 = 4;
const CMPTLZ_LIT_POS_MAX: u8 = CMPTLZ_POS_STATE_MAX * 8;
const CMPT_ERROR_UNSUPPORTED: i32 = -1;
const CMPT_OK: i32 = 0;

#[derive(Debug, Default)]
pub struct CmptLzDecProt {
    pub dic_size: u32,
    pub lit_ctx: u8,
    pub pos_bits: u8,
    pub lit_pos: u8,
}

pub fn cmpt_lz_props_decode(prot_data: &[u8], dec_prot: &mut CmptLzDecProt) -> i32 {
    if prot_data.len() < CMPTLZ_PROPS_SIZE {
        return CMPT_ERROR_UNSUPPORTED;
    }

    let dict_size = u32::from_le_bytes([
        prot_data[1],
        prot_data[2],
        prot_data[3],
        prot_data[4],
    ]);

    let dict_size = if dict_size < CMPTLZ_DICT_MIN_LEN {
        CMPTLZ_DICT_MIN_LEN
    } else {
        dict_size
    };
    dec_prot.dic_size = dict_size;

    let first_data = prot_data[0];
    if first_data >= (CMPTLZ_LIT_CTX_MAX * CMPTLZ_POS_STATE_MAX * CMPTLZ_LIT_POS_MAX) {
        return CMPT_ERROR_UNSUPPORTED;
    }

    dec_prot.lit_ctx = first_data % CMPTLZ_LIT_CTX_MAX;
    let first_data = first_data / CMPTLZ_LIT_CTX_MAX;
    dec_prot.pos_bits = first_data / CMPTLZ_POS_STATE_MAX;
    dec_prot.lit_pos = first_data % CMPTLZ_LIT_POS_MAX;

    CMPT_OK
}
