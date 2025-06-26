macro_rules! GET_KEYOFFSET { ($pstTreeInfo:expr) => { ($pstTreeInfo.usKeyOffset - $pstTreeInfo.usNodeOffset) as i32 } }
pub(crate) use GET_KEYOFFSET;
