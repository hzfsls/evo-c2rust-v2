pub fn bzp_write_block_head(out_data: &mut BzpOutComdata, bwt: &BzpBwtInfo) {
    bzp_write_to_array(BZP_BLOCK_HEAD_0, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_BLOCK_HEAD_1, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_BLOCK_HEAD_2, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_BLOCK_HEAD_3, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_BLOCK_HEAD_4, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_BLOCK_HEAD_5, BZP_BITS8, out_data);
    bzp_write_int32(bwt.block_crc, out_data);
    bzp_write_to_array(0, BZP_BIT, out_data);
    bzp_write_to_array(bwt.ori_ptr, BZP_BITS24, out_data);
}
