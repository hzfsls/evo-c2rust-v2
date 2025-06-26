use std::os::raw::c_int32;

// Assuming the following constants and types are defined elsewhere:
// const BZP_BITS8: u32 = 8;
// const BZP_HDR_B: u8 = ...;
// const BZP_HDR_Z: u8 = ...;
// const BZP_HDR_H: u8 = ...;
// const BZP_HDR_0: u8 = ...;
// const BZP_ERROR_DATA_MAGIC: c_int32 = ...;
// const BZP_OK: c_int32 = ...;
// 
// fn BZP_INVALID_BLOCK_SIZE(size: i32) -> bool { ... }
// 
// struct InDeComdata {
//     blockSize: i32,
//     // other fields...
// }
// 
// fn BzpReadBits(bits: u32, in_data: &mut InDeComdata) -> u8 { ... }

pub fn bzp_read_file_head(in_data: &mut InDeComdata) -> c_int32 {
    let ch = BzpReadBits(BZP_BITS8, in_data);
    if ch != BZP_HDR_B {
        return BZP_ERROR_DATA_MAGIC;
    }
    
    let ch = BzpReadBits(BZP_BITS8, in_data);
    if ch != BZP_HDR_Z {
        return BZP_ERROR_DATA_MAGIC;
    }
    
    let ch = BzpReadBits(BZP_BITS8, in_data);
    if ch != BZP_HDR_H {
        return BZP_ERROR_DATA_MAGIC;
    }
    
    let ch = BzpReadBits(BZP_BITS8, in_data);
    let block_size = (ch as i32) - (BZP_HDR_0 as i32);
    
    if BZP_INVALID_BLOCK_SIZE(block_size) {
        return BZP_ERROR_DATA_MAGIC;
    }
    
    in_data.blockSize = block_size;
    BZP_OK
}
