macro_rules! MD5_COMPOSE_DIGEST {
    ($digest:expr, $md5State:expr) => {
        {
            let mut __i: u32 = 0;
            let mut __j: u32 = 0;
            c_for!(__i = 0; __i < $md5State.len().cast(); __i += 1; {
                $digest[__j] = $md5State[__i].cast();
                __j += 1;
                $digest[__j] = ($md5State[__i] >> 8).cast();
                __j += 1;
                $digest[__j] = ($md5State[__i] >> 16).cast();
                __j += 1;
                $digest[__j] = ($md5State[__i] >> 24).cast();
                __j += 1;
            });
        }
    };
}
pub(crate) use MD5_COMPOSE_DIGEST;