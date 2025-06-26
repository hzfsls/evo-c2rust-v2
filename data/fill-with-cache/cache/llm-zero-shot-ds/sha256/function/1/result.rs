use std::num::Wrapping;

const SHIFTS_PER_BYTE: u32 = 3;
const SHA256_ERROR: u32 = 1;
const SHA256_OK: u32 = 0;

#[repr(C)]
struct VOS_SHA256_CTX {
    N: [u32; 2],
    corrupted: u32,
}

fn vos_sha256_ctx_prepare(pst_ctx: &mut VOS_SHA256_CTX, ui_len: u32) -> u32 {
    let ui_cnt_first = (pst_ctx.N[0] + (ui_len << SHIFTS_PER_BYTE)) & 0xffffffff;
    
    if ui_cnt_first < pst_ctx.N[0] {
        pst_ctx.N[1] = pst_ctx.N[1].wrapping_add(1);
        if pst_ctx.N[1] == 0 {
            pst_ctx.corrupted = 1;
            return SHA256_ERROR;
        }
    }
    
    let ui_cnt_sec = pst_ctx.N[1].wrapping_add(ui_len >> (32 - SHIFTS_PER_BYTE));
    
    if ui_cnt_sec < pst_ctx.N[1] {
        pst_ctx.corrupted = 1;
        return SHA256_ERROR;
    }
    
    pst_ctx.N[1] = ui_cnt_sec;
    pst_ctx.N[0] = ui_cnt_first;
    
    SHA256_OK
}
