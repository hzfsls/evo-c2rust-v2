#[inline]
fn rapidlz_zero_bytes_encode(dest: &mut [u8]) -> Result<usize, RapidlzError> {
    if dest.is_empty() {
        return Err(RapidlzError::EncNotOk);
    }
    dest[0] = 0;
    Ok(1)
}
