#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecCtx {
    pub prop: CmptLzDecProt,
    pub probs: Ptr<CmptLzDecProb>,
    pub probsPlus1664: Ptr<CmptLzDecProb>,
    pub dict: Ptr<u8>,
    pub dictBufSize: usize,
    pub dictPos: usize,
    pub buf: Ptr<u8>,
    pub range: u32,
    pub code: u32,
    pub processedPos: u32,
    pub checkDicSize: u32,
    pub reps: Array<u32, 4>,
    pub state: u32,
    pub remainLen: u32,
    pub numProbs: u32,
    pub tempBufSize: u32,
    pub tempBuf: Array<u8, { CMPTLZ_REQUIRED_INPUT_MAX!() }>,
}
