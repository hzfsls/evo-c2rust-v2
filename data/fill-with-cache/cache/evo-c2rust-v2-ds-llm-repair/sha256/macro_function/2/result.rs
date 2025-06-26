macro_rules! GET_UINT32_BE {
    ($p:expr, $i:expr) => {
        (($p[$i + 0] as u32) << 24) | (($p[$i + 1] as u32) << 16) | (($p[$i + 2] as u32) << 8) | (($p[$i + 3] as u32) << 0)
    }
}
pub(crate) use GET_UINT32_BE;
