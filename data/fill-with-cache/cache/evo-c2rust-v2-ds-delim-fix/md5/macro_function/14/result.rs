macro_rules! MD5_H_PROC { ($tmpValue:expr, $tmpState:expr, $textFragment:expr) =>
    {
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[5], 0xfffa3942, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[8], 0x8771f681, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[11], 0x6d9d6122, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[14], 0xfde5380c, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[1], 0xa4beea44, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[4], 0x4bdecfa9, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[7], 0xf6bb4b60, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[10], 0xbebfbc70, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[13], 0x289b7ec6, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[0], 0xeaa127fa, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[3], 0xd4ef3085, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[6], 0x04881d05, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[9], 0xd9d4d039, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[12], 0xe6db99e5, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[15], 0x1fa27cf8, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[2], 0xc4ac5665, 23);
    }
}
pub(crate) use MD5_H_PROC;
