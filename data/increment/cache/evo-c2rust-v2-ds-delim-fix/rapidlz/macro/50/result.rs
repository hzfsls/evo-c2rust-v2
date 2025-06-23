macro_rules! RAPIDLZFILENAME { () => { if strrchr(__FILE__!(), b'/').as_bool() { strrchr(__FILE__!(), b'/') + 1 } else { __FILE__!() } } }
pub(crate) use RAPIDLZFILENAME;
