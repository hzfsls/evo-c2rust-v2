macro_rules! CMPTLZ_LOG {
    ($error_code:expr, $fmt:expr, $($args:tt)*) => {
        CmptlzLogWrite(
            $error_code as usize,
            std::module_path!(),
            line!(),
            $fmt,
            $($args)*
        );
    };
}

pub(crate) use CMPTLZ_LOG;
