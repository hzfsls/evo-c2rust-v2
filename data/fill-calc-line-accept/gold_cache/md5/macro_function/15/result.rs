macro_rules! MD5_I_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[0], 0xf4292244, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[7], 0x432aff97, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[14], 0xab9423a7, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[5], 0xfc93a039, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[12], 0x655b59c3, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[3], 0x8f0ccc92, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[10], 0xffeff47d, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[1], 0x85845dd1, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[8], 0x6fa87e4f, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[15], 0xfe2ce6e0, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[6], 0xa3014314, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[13], 0x4e0811a1, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[4], 0xf7537e82, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[11], 0xbd3af235, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[2], 0x2ad7d2bb, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[9], 0xeb86d391, 21);
    };
}
pub(crate) use MD5_I_PROC;