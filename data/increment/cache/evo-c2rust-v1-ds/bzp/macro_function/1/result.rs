macro_rules! BZP_INVALID_ALPHA_SIZE { ($alphaSize:expr) => { $alphaSize > BZP_MAX_ALPHA_SIZE!() || $alphaSize < 1 } }
pub(crate) use BZP_INVALID_ALPHA_SIZE;
