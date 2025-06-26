#[derive(Debug, Clone, Copy)]
pub struct CmptlzOpt {
    pub state: CmptlzState,
    pub price: u32,
    pub pos_prev: u32,
    pub back_prev: u32,
    pub backs: [u32; CMPTLZ_NUM_REPS],
}
