pub fn bzp_read_file_end(in_data: &mut InDeComData, cal_total_crc: u32) -> i32 {
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_FILE_END_1 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_FILE_END_2 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_FILE_END_3 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_FILE_END_4 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_FILE_END_5 {
        return BZP_ERROR_DATA;
    }

    let stored_combined_crc = bzp_read_uint32(in_data);

    if cal_total_crc != stored_combined_crc {
        return BZP_ERROR_DATA;
    }
    BZP_OK
}
