macro_rules! PUT_UINT32_BE { ($v:expr, $p:expr, $i:expr) =>
    {
        $p[$i + 0] = ($v >> 24) as u8;
        $p[$i + 1] = ($v >> 16) as u8;
        $p[$i + 2] = ($v >> 8) as u8;
        $p[$i + 3] = ($v >> 0) as u8;
    }
}
pub(crate) use PUT_UINT32_BE;
