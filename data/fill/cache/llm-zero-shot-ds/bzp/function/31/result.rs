pub fn bzp_write_file_head(out_data: &mut BzpOutComdata, block_id: i32) {
    if block_id == 0 {
        bzp_write_to_array(BZP_HDR_B, BZP_BITS8, out_data);
        bzp_write_to_array(BZP_HDR_Z, BZP_BITS8, out_data);
        bzp_write_to_array(BZP_HDR_H, BZP_BITS8, out_data);
        bzp_write_to_array(BZP_HDR_0 + out_data.block_size, BZP_BITS8, out_data);
    }
}
