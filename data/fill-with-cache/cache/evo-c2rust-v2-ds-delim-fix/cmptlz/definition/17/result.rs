#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct LitMarcov {
    pub pos: u32,
    pub prevByte: u32,
    pub literal: Array<Array<CmptlzProb, { CMPTLZ_LIT_MAX_SIZE!() }>, { 1 << CMPTLZ_LCLP_MAX!() }>,
    pub lcBits: u32,
    pub posMask: u32,
}
