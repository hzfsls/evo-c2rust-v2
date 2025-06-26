#[derive(Debug, Clone, Copy)]
struct CmptlzOpt {
    state: CmptlzState,
    price: u32,
    pos_prev: u32,
    back_prev: u32,
    backs: [u32; CMPTLZ_NUM_REPS],
}
