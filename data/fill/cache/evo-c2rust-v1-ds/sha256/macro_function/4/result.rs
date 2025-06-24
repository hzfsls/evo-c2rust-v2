macro_rules! VOS_ROUND {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $k:expr, $W:expr) => {
        $h += (VOS_ROTR32!($e, 6) ^ VOS_ROTR32!($e, 11) ^ VOS_ROTR32!($e, 25)) + ($g ^ ($e & ($f ^ $g))) + $k + $W[$i];
        $d += $h;
        $h += (VOS_ROTR32!($a, 2) ^ VOS_ROTR32!($a, 13) ^ VOS_ROTR32!($a, 22)) + (($a & ($b | $c)) | ($b & $c));
    }
}
pub(crate) use VOS_ROUND;
