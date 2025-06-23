pub fn bzp_file_finish(bzp_f: *mut BzpFile) {
    if !bzp_f.is_null() {
        unsafe {
            let bzp_file = &mut *bzp_f;
            bzp_stream_finish(bzp_file.input);
            bzp_stream_finish(bzp_file.output);
            Box::from_raw(bzp_f);
        }
    }
}
