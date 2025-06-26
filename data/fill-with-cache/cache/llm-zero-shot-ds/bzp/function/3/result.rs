pub fn bzp_in_de_comdata_finish(in_data: Option<Box<InDeComdata>>) {
    if let Some(in_data) = in_data {
        // The Box will be automatically dropped (freed) when it goes out of scope
        // No need to explicitly set to null, Rust's ownership system handles this
    }
}
