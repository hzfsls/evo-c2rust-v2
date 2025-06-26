use std::ptr;
use std::mem;

pub unsafe fn bzp_reset_compress(bwt: *mut BzpBwtInfo, out_data: *mut BzpOutComdata) {
    // Reset outData->num to 0
    (*out_data).num = 0;

    // Reset bwt fields
    (*bwt).n_block = 0;
    (*bwt).block_crc = BZP_INIT_BLOCK_CRC;
    
    // Zero out bwt->inUse
    ptr::write_bytes((*bwt).in_use.as_mut_ptr(), 0, (*bwt).in_use.len());
    
    // Calculate size and zero out bwt->isStartPos
    let n = (*out_data).block_size * BZP_BASE_BLOCK_SIZE * mem::size_of::<i32>();
    ptr::write_bytes((*bwt).is_start_pos, 0, n / mem::size_of::<u8>());
    
    // Increment blockId
    (*bwt).block_id += 1;
}
