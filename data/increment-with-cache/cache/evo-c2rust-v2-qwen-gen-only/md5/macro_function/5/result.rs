macro_rules! MD5_COMPOSE_DIGEST { ($digest:expr, $md5State:expr) =>
    {
        let mut __i: u32 = 0;
        let mut __j: u32 = 0;
        while __i < (c_sizeofval!($md5State) / c_sizeofval!($md5State[0])).cast()
        {
            $digest[__j] = $md5State[__i] as u8;
            __j += 1;
            $digest[__j] = ($md5State[__i] >> 8) as u8;
            __j += 1;
            $digest[__j] = ($md5State[__i] >> 16) as u8;
            __j += 1;
            $digest[__j] = ($md5State[__i] >> 24) as u8;
            __j += 1;
            __i += 1;
        }
    }
}
pub(crate) use MD5_COMPOSE_DIGEST;