use std::mem::size_of;

const CMPTLZ_PROPS_SIZE: usize = 5;
const CMPTLZ_POS_STATE_MAX: u8 = 16;
const CMPTLZ_LIT_CTX_MAX: u8 = 8;

#[repr(C)]
pub struct CmptLzEncCtx {
    dicSize: u32,
    posBits: u8,
    litPos: u8,
    litCtx: u8,
}

pub enum CmptError {
    ErrorData,
    EncErrorHead,
}

pub fn cmpt_head_write(
    enc_ctx: &CmptLzEncCtx,
    prot_data: Option<&mut [u8]>,
    props_size: &mut usize,
) -> Result<(), CmptError> {
    let prot_data = match prot_data {
        Some(data) => data,
        None => {
            // CMPTLZ_LOG(CMPT_ERROR_DATA, "protData is NULL");
            return Err(CmptError::ErrorData);
        }
    };

    if *props_size < CMPTLZ_PROPS_SIZE {
        // CMPTLZ_LOG(CMPT_ERROR_DATA, "propsSize need 5 bytes, get {}", *props_size);
        return Err(CmptError::ErrorData);
    }

    // Write dictionary size in little-endian
    let dic_size_bytes = enc_ctx.dicSize.to_le_bytes();
    prot_data[1..5].copy_from_slice(&dic_size_bytes);

    // Calculate and write the first byte
    prot_data[0] = (enc_ctx.posBits * CMPTLZ_POS_STATE_MAX + enc_ctx.litPos) * CMPTLZ_LIT_CTX_MAX + enc_ctx.litCtx;

    *props_size = CMPTLZ_PROPS_SIZE;
    Ok(())
}
