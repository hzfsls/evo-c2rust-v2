pub fn bzp_decompress_one_block(
    in_data: &mut InDeComdata,
    huffman: &mut BzpHuffmanDecode,
    debwt: &mut BzpBwtDecodeInfo,
) -> i32 {
    let mut ret = BZP_OK;
    bzp_check_file_head(in_data);
    let block_crc = bzp_read_uint32(in_data);

    bzp_read_bits(BZP_BIT, in_data);

    let ori_ptr = bzp_read_uint24(in_data);
    if ori_ptr < 0 || ori_ptr > BZP_BASE_BLOCK_SIZE * in_data.block_size {
        return BZP_ERROR_DATA;
    }

    let n_in_use = bzp_get_dictionary_list(in_data);
    huffman.alpha_size = n_in_use + BZP_EXTRA_CHARS_NUM;
    huffman.n_groups = bzp_read_bits(BZP_BITS3, in_data);
    if huffman.n_groups < BZP_NGROUPS_NUM_0 || huffman.n_groups > BZP_NGROUPS_NUM_4 {
        return BZP_ERROR_DATA;
    }
    huffman.n_select = bzp_read_bits(BZP_BITS15, in_data);

    let n_select_upper_limit = (in_data.block_size * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP + 1);
    if huffman.n_select < 1 || huffman.n_select > n_select_upper_limit {
        return BZP_ERROR_DATA;
    }

    ret |= bzp_de_huffman_select(in_data, huffman);

    ret |= bzp_de_huffman_len(in_data, huffman);
    if ret != BZP_OK {
        return ret;
    }

    bzp_generate_decode_table(huffman);

    debwt.ori_ptr = ori_ptr;
    ret = bzp_mtf_de_code(in_data, huffman, debwt);
    if ret != BZP_OK || debwt.n_block >= BZP_BASE_BLOCK_SIZE * in_data.block_size {
        return BZP_ERROR_DATA;
    }

    bzp_bwt_decode(debwt);

    ret = bzp_de_code_to_stream(in_data, debwt);
    if ret != BZP_OK {
        return ret;
    }
    in_data.block_crc = !(in_data.block_crc);

    if block_crc != in_data.block_crc {
        ret = BZP_ERROR_DATA;
    }

    ret
}
