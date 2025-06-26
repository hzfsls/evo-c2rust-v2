macro_rules! MD5_F_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[0], 0xd76aa478, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[1], 0xe8c7b756, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[2], 0x242070db, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[3], 0xc1bdceee, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[4], 0xf57c0faf, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[5], 0x4787c62a, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[6], 0xa8304613, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[7], 0xfd469501, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[8], 0x698098d8, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[9], 0x8b44f7af, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[10], 0xffff5bb1, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[11], 0x895cd7be, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[12], 0x6b901122, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[13], 0xfd987193, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[14], 0xa679438e, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[15], 0x49b40821, 22);
    };
}

pub(crate) use MD5_F_PROC;
