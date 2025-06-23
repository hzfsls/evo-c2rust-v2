macro_rules! MD5_COMPOSE_DIGEST { ($digest:expr, $md5State:expr) =>
    {
        let mut __i: u32 = 0;
        let mut __j: u32 = 0;
        c_for!(; __i < (c_sizeofval!($md5State) / c_sizeofval!($md5State[0])).cast(); __i.plus_plus(); {
            $digest[__j as usize] = $md5State[__i as usize] as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 8) as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 16) as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 24) as u8;
            __j.plus_plus();
        });
    }
}
pub(crate) use MD5_COMPOSE_DIGEST;
