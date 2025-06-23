use std::ptr;
use std::mem::MaybeUninit;
use std::os::raw::c_uint;

const SHA256_BLOCK_SIZE: usize = 64;
const SHA256_ERROR: u32 = 0;
const SHA256_OK: u32 = 1;

#[repr(C)]
struct VOS_SHA256_CTX {
    block: [u8; SHA256_BLOCK_SIZE],
    blocklen: u32,
    corrupted: u32,
    // Other fields as needed
}

extern "C" {
    fn vosSha256CompressMul(ctx: *mut VOS_SHA256_CTX, block: *const u8, count: u32);
}

fn vos_sha256_last_padding(
    puc_data: *const u8,
    ui_len: u32,
    pst_ctx: *mut VOS_SHA256_CTX,
    pui_padding_len: *mut u32,
) -> u32 {
    unsafe {
        let ui_blc_len = (*pst_ctx).blocklen as usize;
        let puc_block = (*pst_ctx).block.as_mut_ptr();

        if ui_len as usize >= SHA256_BLOCK_SIZE || ui_len as usize + ui_blc_len >= SHA256_BLOCK_SIZE {
            let copy_len = SHA256_BLOCK_SIZE - ui_blc_len;
            ptr::copy_nonoverlapping(
                puc_data,
                puc_block.add(ui_blc_len),
                copy_len,
            );

            vosSha256CompressMul(pst_ctx, puc_block, 1);
            *pui_padding_len = copy_len as u32;
            (*pst_ctx).blocklen = 0;
            ptr::write_bytes(puc_block, 0, SHA256_BLOCK_SIZE);
        } else {
            ptr::copy_nonoverlapping(
                puc_data,
                puc_block.add(ui_blc_len),
                ui_len as usize,
            );
            (*pst_ctx).blocklen += ui_len;
            return SHA256_ERROR;
        }
    }
    SHA256_OK
}
