#[repr(C)]
#[derive(Default)]
pub struct CmptlzOpt {
    pub state: CmptlzState,
    pub price: u32,
    pub posPrev: u32,
    pub backPrev: u32,
    pub backs: Array<u32, { CMPTLZ_NUM_REPS!() }>,
}
