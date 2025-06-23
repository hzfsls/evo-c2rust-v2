use std::fs::File;

pub fn bzp_de_com_stream_finish(
    in_data: &mut InDeComData,
    in_stream: &mut BzpStream,
    out_stream: &mut BzpStream,
) {
    if let Some(file) = in_stream.file_ptr.take() {
        drop(file); // This closes the file when dropped
    }
    if let Some(file) = out_stream.file_ptr.take() {
        drop(file); // This closes the file when dropped
    }
    bzp_stream_finish(in_stream);
    bzp_stream_finish(out_stream);
    bzp_in_de_comdata_finish(in_data);
}
