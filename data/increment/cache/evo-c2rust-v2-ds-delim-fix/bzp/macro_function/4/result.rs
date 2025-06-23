macro_rules! BZP_BLOCK_FULL { ($bwt:expr) => { $bwt.nBlock >= $bwt.nBlockMax } }
pub(crate) use BZP_BLOCK_FULL;
