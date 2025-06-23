macro_rules! RAPIDLZ_LOG {
    ($error_code:expr, $fmt:expr, $($args:tt)*) => {
        RapidlzLogWrite(
            $error_code as usize,
            RAPIDLZFILENAME,
            line!(),
            $fmt,
            $($args)*
        );
    };
}

pub(crate) use RAPIDLZ_LOG;
