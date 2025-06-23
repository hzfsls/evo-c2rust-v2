macro_rules! MD5_G_PROC { ($tmpValue:expr, $tmpState:expr, $textFragment:expr) =>
    {
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[1], 0xf61e2562, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[6], 0xc040b340, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[11], 0x265e5a51, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[0], 0xe9b6c7aa, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[5], 0xd62f105d, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[10], 0x02441453, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[15], 0xd8a1e681, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[4], 0xe7d3fbc8, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[9], 0x21e1cde6, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[14], 0xc33707d6, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[3], 0xf4d50d87, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[8], 0x455a14ed, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[13], 0xa9e3e905, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[2], 0xfcefa3f8, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[7], 0x676f02d9, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[12], 0x8d2a4c8a, 20);
    }
}
pub(crate) use MD5_G_PROC;
