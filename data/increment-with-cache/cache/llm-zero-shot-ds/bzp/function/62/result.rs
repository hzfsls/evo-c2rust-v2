pub fn bzp_decompress_data(in_data: &mut InDeComdata) -> i32 {
    let mut ret = BZP_OK;
    let mut caltotal_crc = 0u32;
    let mut ch;

    ret = bzp_read_file_head(in_data);
    if ret != BZP_OK {
        return ret;
    }

    let mut huffman = bzp_huffman_decode_init(in_data.block_size);
    let mut debwt = bzp_bwt_decode_init(in_data.block_size);

    loop {
        ch = bzp_read_bits(BZP_BITS8, in_data);
        if ch == BZP_FILE_END_0 {
            break;
        }

        if ch != BZP_BLOCK_HEAD_0 {
            ret = BZP_ERROR_DATA;
            break;
        }

        bzp_huffman_decode_reset(&mut huffman);
        in_data.block_crc = BZP_INIT_BLOCK_CRC;

        ret = bzp_decompress_one_block(in_data, &mut huffman, &mut debwt);
        if ret != BZP_OK {
            break;
        }

        caltotal_crc = (caltotal_crc << 1) | (caltotal_crc >> BZP_CRC_MOVE_RIGHT_VAL);
        caltotal_crc ^= in_data.block_crc;
    }

    if ret == BZP_OK {
        ret = bzp_read_file_end(in_data, caltotal_crc);
    }

    bzp_huffman_decode_finish(huffman);
    bzp_bwt_decode_finish(debwt);

    ret
}
