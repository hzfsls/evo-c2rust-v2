macro_rules! GET_KEYOFFSET {
    ($pstTreeInfo:expr) => {
        (($pstTreeInfo.usKeyOffset as isize) - ($pstTreeInfo.usNodeOffset as isize)) as i32
    };
}
pub(crate) use GET_KEYOFFSET;
