pub fn bzp_compress_end(bzp_info: &mut BzpAlgorithmInfo) {
    if let Some(file_ptr) = bzp_info.compress_file.input.file_ptr {
        unsafe {
            libc::fclose(file_ptr);
        }
        bzp_info.compress_file.input.file_ptr = None;
    }
    if let Some(file_ptr) = bzp_info.compress_file.output.file_ptr {
        unsafe {
            libc::fclose(file_ptr);
        }
        bzp_info.compress_file.output.file_ptr = None;
    }
    bzp_algorithm_info_finish(bzp_info);
}
