macro_rules! NOT_EQUAL_2_BYTES { ($a:expr, $b:expr) => { $a[0] != $b[0] || $a[1] != $b[1] } }
pub(crate) use NOT_EQUAL_2_BYTES;
