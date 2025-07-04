#[repr(C)]
pub struct CmptLzDecCtx {
    pub prop: CmptLzDecProt,
    pub probs: *mut CmptLzDecProb,
    pub probs_plus_1664: *mut CmptLzDecProb,
    pub dict: *mut u8,
    pub dict_buf_size: usize,
    pub dict_pos: usize,
    pub buf: *const u8,
    pub range: u32,
    pub code: u32,
    pub processed_pos: u32,
    pub check_dic_size: u32,
    pub reps: [u32; 4],
    pub state: u32,
    pub remain_len: u32,
    pub num_probs: u32,
    pub temp_buf_size: u32,
    pub temp_buf: [u8; CMPTLZ_REQUIRED_INPUT_MAX],
}
