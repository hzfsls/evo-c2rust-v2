use crate::cmpt_lz_enc::CmptLzEncCtx;
use crate::cmpt_lz_enc::CmptlzEncShortRep;
use crate::cmpt_lz_enc::CmptlzEncLongRep;
use crate::constants::{CMPT_OK, CMPTLZ_RETURN_IF_NOT_OK};

pub fn cmpt_enc_short_or_rep0(enc_ctx: &mut CmptLzEncCtx, nowpos32: u32, len_res: u32) -> i32 {
    let mut shift_res = CMPT_OK;
    if len_res == 1 {
        shift_res = CmptlzEncShortRep(enc_ctx, nowpos32);
        CMPTLZ_RETURN_IF_NOT_OK!(shift_res);
    } else {
        shift_res = CmptlzEncLongRep(enc_ctx, 0, nowpos32, len_res);
        CMPTLZ_RETURN_IF_NOT_OK!(shift_res);
    }
    CMPT_OK
}
