macro_rules! RAPIDLZ_ERROR_OUTPUT { () => { (-((curSrc as isize) - (src as isize)) - 1) as i32 } }
pub(crate) use RAPIDLZ_ERROR_OUTPUT;