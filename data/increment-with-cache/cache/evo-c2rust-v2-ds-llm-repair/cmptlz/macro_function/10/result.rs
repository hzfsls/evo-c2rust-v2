macro_rules! GET_LEN_TO_POS_STATE { ($len:expr) => { if $len < CMPT_NUM_LEN_POS_STATE + 1 { $len - 2 } else { CMPT_NUM_LEN_POS_STATE - 1 } } }
pub(crate) use GET_LEN_TO_POS_STATE;
