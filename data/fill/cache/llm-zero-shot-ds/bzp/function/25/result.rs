pub fn bzp_file_finish(bzp_f: Option<Box<BzpFile>>) {
    if let Some(mut bzp_f) = bzp_f {
        bzp_stream_finish(bzp_f.input);
        bzp_stream_finish(bzp_f.output);
        // The Box will be automatically dropped (freed) when it goes out of scope here
    }
}
