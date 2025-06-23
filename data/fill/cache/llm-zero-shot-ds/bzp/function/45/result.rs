pub fn bzp_process_data(bzp_info: &mut BzpAlgorithmInfo, is_last_data: bool) -> i32 {
    let bzpf = &mut bzp_info.compress_file;
    let out_data = &mut bzp_info.out_data;
    let bwt = &mut bzp_info.bwt;
    bzpf.state = BzpState::InputCompress;
    let mut ret = BZP_OK;

    while bzpf.state != BzpState::ReturnCompress {
        match bzpf.state {
            BzpState::OutputCompress => {
                ret = bzp_buff_to_stream(bzpf, out_data);
                bzp_reset_compress(bwt, out_data);
                bzpf.state = BzpState::InputCompress;
                if is_last_data && bzp_buff_read_empty(bzpf) {
                    bzpf.state = BzpState::ReturnCompress;
                }
            }
            BzpState::InputCompress => {
                bzp_buff_to_block_rlc(bzpf, bwt, is_last_data);
                if is_last_data && bzp_buff_read_empty(bzpf) {
                    ret = bzp_compress_one_block(bzp_info, out_data);
                    bzp_write_file_end(out_data, bwt.combined_crc);
                    bzp_flush_buf(out_data);
                    bzpf.state = BzpState::OutputCompress;
                } else if bzp_block_full(bwt) {
                    ret = bzp_compress_one_block(bzp_info, out_data);
                    bzpf.state = BzpState::OutputCompress;
                } else {
                    bzpf.state = BzpState::ReturnCompress;
                }
            }
            _ => {}
        }

        if ret != BZP_OK {
            return ret;
        }
    }

    ret
}
