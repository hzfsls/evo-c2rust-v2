pub fn bzp_check_file_head(in_data: &mut InDeComdata) -> i32 {
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_BLOCK_HEAD_1 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_BLOCK_HEAD_2 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_BLOCK_HEAD_3 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_BLOCK_HEAD_4 {
        return BZP_ERROR_DATA;
    }
    let ch = bzp_read_bits(BZP_BITS8, in_data);
    if ch != BZP_BLOCK_HEAD_5 {
        return BZP_ERROR_DATA;
    }
    BZP_OK
}
