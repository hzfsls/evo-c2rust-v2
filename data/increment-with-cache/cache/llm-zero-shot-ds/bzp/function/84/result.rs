pub fn bzp_compress_one_block(bzp_info: &mut BzpAlgorithmInfo, out_data: &mut BzpOutComdata) -> i32 {
    let bwt = &mut bzp_info.bwt;
    let mtf = &mut bzp_info.mtf;
    let huffman = &mut bzp_info.huffman;
    
    if bwt.n_block == 0 {
        return BZP_OK;
    }

    bzp_write_file_head(out_data, bwt.block_id);
    
    if bwt.n_block > 0 {
        bzp_calculate_crc(bwt);
        bzp_block_sort_main(bwt);
        
        bzp_mtf_reset(mtf);
        
        mtf.block = bwt.block;
        mtf.map = bwt.sort_block;
        mtf.in_use = bwt.in_use;
        mtf.n_block = bwt.n_block;
        
        bzp_mtf_main(mtf);
        
        let ret = bzp_huffman_groups_reset(huffman, mtf.n_use + BZP_EXTRA_CHARS_NUM);
        if ret != BZP_OK {
            return ret;
        }
        
        huffman.block = mtf.mtf_v;
        huffman.mtf_freq = mtf.mtf_freq;
        huffman.n_block = mtf.n_mtf;
        
        bzp_huffman_main(huffman);
        
        bzp_write_block_head(out_data, bwt);
        bzp_write_valid_ascii(out_data, bwt);
        bzp_write_to_array(huffman.n_groups, BZP_BITS3, out_data);
        bzp_write_select(out_data, huffman);
        bzp_write_len(out_data, huffman);
        bzp_write_input_encode(out_data, mtf, huffman);
    }
    
    BZP_OK
}
