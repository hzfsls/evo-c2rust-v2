macro_rules! VOS_ROTR32 {
    ($x:expr, $uiBlcLen:expr) => {
        (($x << (32 - $uiBlcLen)) | ($x >> $uiBlcLen))
    };
}
pub(crate) use VOS_ROTR32;
