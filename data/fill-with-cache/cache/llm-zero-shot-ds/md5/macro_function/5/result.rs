macro_rules! MD5_COMPOSE_DIGEST {
    ($digest:expr, $md5State:expr) => {
        {
            let mut __i = 0;
            let mut __j = 0;
            while __i < $md5State.len() {
                $digest[__j] = ($md5State[__i] & 0xff) as u8;
                __j += 1;
                $digest[__j] = (($md5State[__i] >> 8) & 0xff) as u8;
                __j += 1;
                $digest[__j] = (($md5State[__i] >> 16) & 0xff) as u8;
                __j += 1;
                $digest[__j] = (($md5State[__i] >> 24) & 0xff) as u8;
                __j += 1;
                __i += 1;
            }
        }
    };
}

pub(crate) use MD5_COMPOSE_DIGEST;
