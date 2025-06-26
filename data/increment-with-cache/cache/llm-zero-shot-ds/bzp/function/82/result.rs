pub fn bzp_write_file_end(out_data: &mut BzpOutComdata, combined_crc: i32) {
    bzp_write_to_array(BZP_FILE_END_0, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_FILE_END_1, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_FILE_END_2, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_FILE_END_3, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_FILE_END_4, BZP_BITS8, out_data);
    bzp_write_to_array(BZP_FILE_END_5, BZP_BITS8, out_data);
    bzp_write_int32(combined_crc, out_data);
}
