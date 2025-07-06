use crate::translation_utils::*;

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzUnalignU32 {
    pub v: u32,
}

pub type EnCmptErrNo = i32;
macro_rules! CMPTLZ_ERROR_DATA {
    () => {
        1
    };
}
pub(crate) use CMPTLZ_ERROR_DATA;
macro_rules! CMPTLZ_ERROR_MEM {
    () => {
        2
    };
}
pub(crate) use CMPTLZ_ERROR_MEM;
macro_rules! CMPTLZ_ERROR_UNSUPPORTED {
    () => {
        3
    };
}
pub(crate) use CMPTLZ_ERROR_UNSUPPORTED;
macro_rules! CMPTLZ_ENC_ERROR_FILESIZE {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_ENC_ERROR_FILESIZE;
macro_rules! CMPTLZ_ENC_CTX_INIT_FAIL {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_ENC_CTX_INIT_FAIL;
macro_rules! CMPTLZ_ENC_RC_INIT_FAIL {
    () => {
        6
    };
}
pub(crate) use CMPTLZ_ENC_RC_INIT_FAIL;
macro_rules! CMPTLZ_ENC_MF_INIT_FAIL {
    () => {
        7
    };
}
pub(crate) use CMPTLZ_ENC_MF_INIT_FAIL;
macro_rules! CMPTLZ_ENC_ERROR_WRITE {
    () => {
        8
    };
}
pub(crate) use CMPTLZ_ENC_ERROR_WRITE;
macro_rules! CMPTLZ_ENC_ERROR_HEAD {
    () => {
        9
    };
}
pub(crate) use CMPTLZ_ENC_ERROR_HEAD;
macro_rules! CMPTLZ_ENC_ERROR_PARAM {
    () => {
        10
    };
}
pub(crate) use CMPTLZ_ENC_ERROR_PARAM;
macro_rules! CMPTLZ_ERROR_BUTT {
    () => {
        11
    };
}
pub(crate) use CMPTLZ_ERROR_BUTT;

pub type CmptLzDecProb = u16;

pub type EnCmptLzMemType = i32;
macro_rules! CMPTLZ_DICT_MEM {
    () => {
        1
    };
}
pub(crate) use CMPTLZ_DICT_MEM;
macro_rules! CMPTLZ_PROB_MEM {
    () => {
        2
    };
}
pub(crate) use CMPTLZ_PROB_MEM;
macro_rules! CMPTLZ_ENC_CCTX {
    () => {
        3
    };
}
pub(crate) use CMPTLZ_ENC_CCTX;
macro_rules! CMPTLZ_MF_CCTX {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_MF_CCTX;
macro_rules! CMPTLZ_MF_HASH {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_MF_HASH;
macro_rules! CMPTLZ_MF_SON {
    () => {
        6
    };
}
pub(crate) use CMPTLZ_MF_SON;
macro_rules! CMPTLZ_RC_CCTX {
    () => {
        7
    };
}
pub(crate) use CMPTLZ_RC_CCTX;
macro_rules! CMPTLZ_RC_BUF {
    () => {
        8
    };
}
pub(crate) use CMPTLZ_RC_BUF;
macro_rules! CMPTLZ_MEM_TYPE_BUT {
    () => {
        9
    };
}
pub(crate) use CMPTLZ_MEM_TYPE_BUT;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzMemHook {
    pub CmptLzAlloc: FuncPtr<fn(i32, usize) -> VoidPtr>,
    pub CmptLzFree: FuncPtr<fn(i32, VoidPtr)>,
}

pub type CmptlzLogFunc = FuncPtr<fn(Ptr<u8>, usize)>;

pub type EnCmptLzFinMode = i32;
macro_rules! CMPTLZ_FINISH_ANY {
    () => {
        0
    };
}
pub(crate) use CMPTLZ_FINISH_ANY;
macro_rules! CMPTLZ_FINISH_END {
    () => {
        1
    };
}
pub(crate) use CMPTLZ_FINISH_END;

pub type EnCmptLzStatus = i32;
macro_rules! CMPTLZ_STATUS_NOT_SPECIFIED {
    () => {
        0
    };
}
pub(crate) use CMPTLZ_STATUS_NOT_SPECIFIED;
macro_rules! CMPTLZ_STATUS_FINISHED_WITH_MARK {
    () => {
        1
    };
}
pub(crate) use CMPTLZ_STATUS_FINISHED_WITH_MARK;
macro_rules! CMPTLZ_STATUS_NOT_FINISHED {
    () => {
        2
    };
}
pub(crate) use CMPTLZ_STATUS_NOT_FINISHED;
macro_rules! CMPTLZ_STATUS_NEEDS_MORE_INPUT {
    () => {
        3
    };
}
pub(crate) use CMPTLZ_STATUS_NEEDS_MORE_INPUT;
macro_rules! CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
macro_rules! CMPTLZ_STATUS_BUT {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_STATUS_BUT;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecProt {
    pub litCtx: u8,
    pub litPos: u8,
    pub posBits: u8,
    pub alignPad: u8,
    pub dicSize: u32,
}

#[repr(C)]
#[derive(Default)]
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

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecIn {
    pub pSrcIn: Ptr<u8>,
    pub strInLen: usize,
    pub strInCostLen: usize,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecOut {
    pub pDestOut: Ptr<u8>,
    pub destOutLen: usize,
    pub destOutFillLen: usize,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzDecParam {
    pub protData: Ptr<u8>,
    pub protSize: u32,
    pub memHook: Ptr<CmptLzMemHook>,
}

pub type CmptLzEncCtx = TagCmptLzEncCtx;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzCompParam {
    pub level: i32,
    pub dictSize: u32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub fastBytes: i32,
    pub numThreads: i32,
    pub protData: Ptr<u8>,
    pub protSize: usize,
    pub memHook: Ptr<CmptLzMemHook>,
}

pub type CmptlzProb = u16;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzEncParam {
    pub level: i32,
    pub dictSize: u32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub fastBytes: i32,
    pub numThreads: i32,
}

#[repr(C)]
#[derive(Default)]
pub struct LitMarcov {
    pub pos: u32,
    pub prevByte: u32,
    pub literal: Array<Array<CmptlzProb, { CMPTLZ_LIT_MAX_SIZE!() }>, { 1 << CMPTLZ_LCLP_MAX!() }>,
    pub lcBits: u32,
    pub posMask: u32,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptRcCtx {
    pub range: u32,
    pub cache: u64,
    pub low: u64,
    pub cacheSize: u64,
    pub buf: Ptr<u8>,
    pub bufBase: Ptr<u8>,
    pub outBuf: Ptr<u8>,
    pub outBufLeft: usize,
}

pub type CmptMfCtx = TagCmptMatchFinder;

#[repr(C)]
#[derive(Default)]
pub struct TagCmptMatchFinder {
    pub srcStart: Ptr<u8>,
    pub srcLen: usize,
    pub hashRootTable: Array<u32, 256>,
    pub mfStart: u32,
    pub niceLen: u32,
    pub readAhead: u32,
    pub readPos: u32,
    pub cyclePos: u32,
    pub cycleSize: u32,
    pub offset: u32,
    pub hash: Ptr<u32>,
    pub son: Ptr<u32>,
    pub depth: u32,
    pub hashCount: u32,
    pub sonsCount: u32,
    pub hashMask: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct CmptLenEncoder {
    pub low: Array<CmptlzProb, 256>,
    pub high: Array<CmptlzProb, { 1 << CMPT_LEN_HIGH_BITS!() }>,
    pub prices: Array<
        Array<
            u32,
            {
                (1 << CMPT_LEN_HIGH_BITS!())
                    + (1 << CMPT_LEN_MID_BITS!())
                    + (1 << CMPT_LEN_LOW_BITS!())
            },
        >,
        { CMPTLZ_NUM_PB_STATES_MAX!() },
    >,
    pub tableSize: u32,
}

pub type CmptlzState = i32;
macro_rules! LIT_LIT {
    () => {
        0
    };
}
pub(crate) use LIT_LIT;
macro_rules! MATCH_LIT_LIT {
    () => {
        1
    };
}
pub(crate) use MATCH_LIT_LIT;
macro_rules! REP_LIT_LIT {
    () => {
        2
    };
}
pub(crate) use REP_LIT_LIT;
macro_rules! SHORTREP_LIT_LIT {
    () => {
        3
    };
}
pub(crate) use SHORTREP_LIT_LIT;
macro_rules! MATCH_LIT {
    () => {
        4
    };
}
pub(crate) use MATCH_LIT;
macro_rules! REP_LIT {
    () => {
        5
    };
}
pub(crate) use REP_LIT;
macro_rules! SHORTREP_LIT {
    () => {
        6
    };
}
pub(crate) use SHORTREP_LIT;
macro_rules! LIT_MATCH {
    () => {
        7
    };
}
pub(crate) use LIT_MATCH;
macro_rules! LIT_LONGREP {
    () => {
        8
    };
}
pub(crate) use LIT_LONGREP;
macro_rules! LIT_SHORTREP {
    () => {
        9
    };
}
pub(crate) use LIT_SHORTREP;
macro_rules! NOTLIT_MATCH {
    () => {
        10
    };
}
pub(crate) use NOTLIT_MATCH;
macro_rules! NOTLIT_REP {
    () => {
        11
    };
}
pub(crate) use NOTLIT_REP;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzMatchPair {
    pub len: u32,
    pub dist: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct CmptlzOpt {
    pub state: CmptlzState,
    pub price: u32,
    pub posPrev: u32,
    pub backPrev: u32,
    pub backs: Array<u32, { CMPTLZ_NUM_REPS!() }>,
}

#[repr(C)]
#[derive(Default)]
pub struct TagCmptLzEncCtx {
    pub level: i32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub dicSize: u32,
    pub endMarker: i32,
    pub numFastBytes: u32,
    pub encNeedFinish: bool,
    pub nowpos64: u64,
    pub cmptlzResponse: u32,
    pub state: CmptlzState,
    pub litMarcov: LitMarcov,
    pub reps: Array<u32, { CMPTLZ_NUM_REPS!() }>,
    pub isRep: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG0: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG1: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isRepG2: Array<CmptlzProb, { CMPTLZ_NUM_STATES!() }>,
    pub isMatch:
        Array<Array<CmptlzProb, { CMPTLZ_NUM_PB_STATES_MAX!() }>, { CMPTLZ_NUM_STATES!() }>,
    pub isRep0Long:
        Array<Array<CmptlzProb, { CMPTLZ_NUM_PB_STATES_MAX!() }>, { CMPTLZ_NUM_STATES!() }>,
    pub probDistSlot:
        Array<Array<CmptlzProb, { 1 << CMPTLZ_DIST_SLOT_BITS!() }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub probDistSpecial: Array<CmptlzProb, { CMPT_DIST_LIMIT_2!() }>,
    pub probAlign: Array<CmptlzProb, { 1 << CMPTLZ_ALIGN_BITS!() }>,
    pub posMask: u32,
    pub pbMask: u64,
    pub lpMask: u64,
    pub rcCtx: Ptr<CmptRcCtx>,
    pub mfCtx: Ptr<CmptMfCtx>,
    pub matches: Array<CmptlzMatchPair, { CMPT_MF_LONGEST_MATCH!() + 1 }>,
    pub matchesCount: u32,
    pub longestMatchLen: u32,
    pub backRes: u32,
    pub lenRes: u32,
    pub optEndIndex: u32,
    pub optsCurIndex: u32,
    pub opts: Array<CmptlzOpt, { CMPT_DP_OPTMAX!() }>,
    pub matchLenEncoder: CmptLenEncoder,
    pub repLenEncoder: CmptLenEncoder,
    pub repLenPriceCount: i32,
    pub matchPriceCount: i32,
    pub priceRootTable: Array<u32, { CMPT_PRIICE_TABLE_SIZE!() }>,
    pub priceDistSlotTable:
        Array<Array<u32, { 1 << CMPTLZ_DIST_SLOT_BITS!() }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub priceDistTable: Array<Array<u32, { 1 << 7 }>, { CMPTLZ_DIST_STATE_TOTAL!() }>,
    pub priceAlignTable: Array<u32, { 1 << CMPTLZ_ALIGN_BITS!() }>,
    pub distTableSize: u32,
}

pub static g_cmptlzLogFunc: Global<CmptlzLogFunc> = global!(NULL!());

macro_rules! CMPTLZ_MODULE {
    () => {
        0x0A00 + 0x0D
    };
}
pub(crate) use CMPTLZ_MODULE;

macro_rules! CMPT_OK {
    () => {
        0
    };
}
pub(crate) use CMPT_OK;

macro_rules! CMPT_ERROR_DATA {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ERROR_DATA!())
    };
}
pub(crate) use CMPT_ERROR_DATA;

macro_rules! CMPT_ERROR_MEM {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ERROR_MEM!())
    };
}
pub(crate) use CMPT_ERROR_MEM;

macro_rules! CMPT_ERROR_UNSUPPORTED {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ERROR_UNSUPPORTED!())
    };
}
pub(crate) use CMPT_ERROR_UNSUPPORTED;

macro_rules! CMPT_ENC_ERROR_FILESIZE {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_ERROR_FILESIZE!())
    };
}
pub(crate) use CMPT_ENC_ERROR_FILESIZE;

macro_rules! CMPT_ENC_CTX_INIT_FAIL {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_CTX_INIT_FAIL!())
    };
}
pub(crate) use CMPT_ENC_CTX_INIT_FAIL;

macro_rules! CMPT_ENC_RC_INIT_FAIL {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_RC_INIT_FAIL!())
    };
}
pub(crate) use CMPT_ENC_RC_INIT_FAIL;

macro_rules! CMPT_ENC_MF_INIT_FAIL {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_MF_INIT_FAIL!())
    };
}
pub(crate) use CMPT_ENC_MF_INIT_FAIL;

macro_rules! CMPT_ENC_ERROR_WRITE {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_ERROR_WRITE!())
    };
}
pub(crate) use CMPT_ENC_ERROR_WRITE;

macro_rules! CMPT_ENC_ERROR_HEAD {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_ERROR_HEAD!())
    };
}
pub(crate) use CMPT_ENC_ERROR_HEAD;

macro_rules! CMPT_ENC_ERROR_PARAM {
    () => {
        CMPTLZ_ERROR_CONVERT!(CMPTLZ_ENC_ERROR_PARAM!())
    };
}
pub(crate) use CMPT_ENC_ERROR_PARAM;

macro_rules! CMPTLZ_PROPS_SIZE {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_PROPS_SIZE;

macro_rules! CMPTLZ_REQUIRED_INPUT_MAX {
    () => {
        20
    };
}
pub(crate) use CMPTLZ_REQUIRED_INPUT_MAX;

macro_rules! CMPTLZ_PROB_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_PROB_MEM!())
    };
}
pub(crate) use CMPTLZ_PROB_HANDLE;

macro_rules! CMPTLZ_ENC_CCTX_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_ENC_CCTX!())
    };
}
pub(crate) use CMPTLZ_ENC_CCTX_HANDLE;

macro_rules! CMPTLZ_MF_CCTX_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_MF_CCTX!())
    };
}
pub(crate) use CMPTLZ_MF_CCTX_HANDLE;

macro_rules! CMPTLZ_MF_HASH_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_MF_HASH!())
    };
}
pub(crate) use CMPTLZ_MF_HASH_HANDLE;

macro_rules! CMPTLZ_MF_SON_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_MF_SON!())
    };
}
pub(crate) use CMPTLZ_MF_SON_HANDLE;

macro_rules! CMPTLZ_RC_CCTX_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_RC_CCTX!())
    };
}
pub(crate) use CMPTLZ_RC_CCTX_HANDLE;

macro_rules! CMPTLZ_RC_BUF_HANDLE {
    () => {
        CMPTLZ_HANDLE_CONVERT!(CMPTLZ_RC_BUF!())
    };
}
pub(crate) use CMPTLZ_RC_BUF_HANDLE;

macro_rules! LOG_BUF_SIZE {
    () => {
        1024
    };
}
pub(crate) use LOG_BUF_SIZE;

macro_rules! CMPTLZ_LIT_CTX_MAX {
    () => {
        9
    };
}
pub(crate) use CMPTLZ_LIT_CTX_MAX;

macro_rules! CMPTLZ_POS_STATE_MAX {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_POS_STATE_MAX;

macro_rules! CMPTLZ_LIT_POS_MAX {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_LIT_POS_MAX;

macro_rules! CMPTLZ_DEC_INPUT_EOF {
    () => {
        1
    };
}
pub(crate) use CMPTLZ_DEC_INPUT_EOF;

macro_rules! CMPTLZ_DICT_MIN_LEN {
    () => {
        1 << 12
    };
}
pub(crate) use CMPTLZ_DICT_MIN_LEN;

macro_rules! CMPTLZ_RANGE_CODE_SIZE {
    () => {
        5
    };
}
pub(crate) use CMPTLZ_RANGE_CODE_SIZE;

macro_rules! CMPTLZ_MKSTATE_NUM {
    () => {
        12
    };
}
pub(crate) use CMPTLZ_MKSTATE_NUM;

macro_rules! CMPTLZ_LIT_STATES {
    () => {
        7
    };
}
pub(crate) use CMPTLZ_LIT_STATES;

macro_rules! CMPTLZ_RANGE_DOWN_LIMIT {
    () => {
        (1isize as u32) << 24
    };
}
pub(crate) use CMPTLZ_RANGE_DOWN_LIMIT;

macro_rules! CMPTLZ_ONE_BYTE_WIDTH {
    () => {
        8
    };
}
pub(crate) use CMPTLZ_ONE_BYTE_WIDTH;

macro_rules! CMPTLZ_PROB_LG_BIT {
    () => {
        11
    };
}
pub(crate) use CMPTLZ_PROB_LG_BIT;

macro_rules! CMPTLZ_PROB_LG {
    () => {
        1 << CMPTLZ_PROB_LG_BIT!()
    };
}
pub(crate) use CMPTLZ_PROB_LG;

macro_rules! CMPTLZ_PB_STATE_NUM_ALIGN {
    () => {
        16
    };
}
pub(crate) use CMPTLZ_PB_STATE_NUM_ALIGN;

macro_rules! CMPTLZ_PB_BITS_MAX {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_PB_BITS_MAX;

macro_rules! CMPTLZ_MATCH_MAX_LEN {
    () => {
        274
    };
}
pub(crate) use CMPTLZ_MATCH_MAX_LEN;

macro_rules! CMPTLZ_LOW_LEN_BIT {
    () => {
        3
    };
}
pub(crate) use CMPTLZ_LOW_LEN_BIT;

macro_rules! CMPTLZ_LOW_LEN_CLASS {
    () => {
        1 << CMPTLZ_LOW_LEN_BIT!()
    };
}
pub(crate) use CMPTLZ_LOW_LEN_CLASS;

macro_rules! CMPTLZ_HIGH_LEN_BIT {
    () => {
        8
    };
}
pub(crate) use CMPTLZ_HIGH_LEN_BIT;

macro_rules! CMPTLZ_HIGH_LEN_CLASS {
    () => {
        1 << CMPTLZ_HIGH_LEN_BIT!()
    };
}
pub(crate) use CMPTLZ_HIGH_LEN_CLASS;

macro_rules! CMPTLZ_LOW_LENPROB_OFFSET {
    () => {
        0
    };
}
pub(crate) use CMPTLZ_LOW_LENPROB_OFFSET;

macro_rules! CMPTLZ_HIGH_LENPROB_OFFSET {
    () => {
        CMPTLZ_LOW_LENPROB_OFFSET!() + ((1 << CMPTLZ_PB_BITS_MAX!()) << (CMPTLZ_LOW_LEN_BIT!() + 1))
    };
}
pub(crate) use CMPTLZ_HIGH_LENPROB_OFFSET;

macro_rules! CMPTLZ_LEN_CHOICE {
    () => {
        CMPTLZ_LOW_LENPROB_OFFSET!()
    };
}
pub(crate) use CMPTLZ_LEN_CHOICE;

macro_rules! CMPTLZ_LEN_CHOICE2 {
    () => {
        CMPTLZ_LEN_CHOICE!() + CMPTLZ_LOW_LEN_CLASS!()
    };
}
pub(crate) use CMPTLZ_LEN_CHOICE2;

macro_rules! CMPTLZ_LENPROB_NUM {
    () => {
        CMPTLZ_HIGH_LENPROB_OFFSET!() + CMPTLZ_HIGH_LEN_CLASS!()
    };
}
pub(crate) use CMPTLZ_LENPROB_NUM;

macro_rules! CMPTLZ_LEN_CONDITION_TO_POSSLOT {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_LEN_CONDITION_TO_POSSLOT;

macro_rules! CMPTLZ_POS_SLOT_BITS {
    () => {
        6
    };
}
pub(crate) use CMPTLZ_POS_SLOT_BITS;

macro_rules! CMPTLZ_LOW_POSSLOT {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_LOW_POSSLOT;

macro_rules! CMPTLZ_HIGH_POSSLOT {
    () => {
        14
    };
}
pub(crate) use CMPTLZ_HIGH_POSSLOT;

macro_rules! CMPTLZ_FULL_DISTANCE {
    () => {
        1 << (CMPTLZ_HIGH_POSSLOT!() >> 1)
    };
}
pub(crate) use CMPTLZ_FULL_DISTANCE;

macro_rules! CMPTLZ_LARGE_DIST_LOW_BITS {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_LARGE_DIST_LOW_BITS;

macro_rules! CMPTLZ_ALIGN_TABLE_SIZE {
    () => {
        1 << CMPTLZ_LARGE_DIST_LOW_BITS!()
    };
}
pub(crate) use CMPTLZ_ALIGN_TABLE_SIZE;

macro_rules! CMPTLZ_OFFSET {
    () => {
        1664
    };
}
pub(crate) use CMPTLZ_OFFSET;

macro_rules! CMPTLZ_SPEC_POS {
    () => {
        (-CMPTLZ_OFFSET!())
    };
}
pub(crate) use CMPTLZ_SPEC_POS;

macro_rules! CMPTLZ_REP0_LONG {
    () => {
        CMPTLZ_SPEC_POS!() + CMPTLZ_FULL_DISTANCE!()
    };
}
pub(crate) use CMPTLZ_REP0_LONG;

macro_rules! CMPTLZ_REP_LEN_CODER {
    () => {
        CMPTLZ_REP0_LONG!() + (CMPTLZ_PB_STATE_NUM_ALIGN!() << CMPTLZ_PB_BITS_MAX!())
    };
}
pub(crate) use CMPTLZ_REP_LEN_CODER;

macro_rules! CMPTLZ_MATCH_LEN_CODER {
    () => {
        CMPTLZ_REP_LEN_CODER!() + CMPTLZ_LENPROB_NUM!()
    };
}
pub(crate) use CMPTLZ_MATCH_LEN_CODER;

macro_rules! CMPTLZ_IS_MATCH {
    () => {
        CMPTLZ_MATCH_LEN_CODER!() + CMPTLZ_LENPROB_NUM!()
    };
}
pub(crate) use CMPTLZ_IS_MATCH;

macro_rules! CMPTLZ_ALIGN {
    () => {
        CMPTLZ_IS_MATCH!() + (CMPTLZ_PB_STATE_NUM_ALIGN!() << CMPTLZ_PB_BITS_MAX!())
    };
}
pub(crate) use CMPTLZ_ALIGN;

macro_rules! CMPTLZ_ISREP {
    () => {
        CMPTLZ_ALIGN!() + CMPTLZ_ALIGN_TABLE_SIZE!()
    };
}
pub(crate) use CMPTLZ_ISREP;

macro_rules! CMPTLZ_ISREPG0 {
    () => {
        CMPTLZ_ISREP!() + CMPTLZ_MKSTATE_NUM!()
    };
}
pub(crate) use CMPTLZ_ISREPG0;

macro_rules! CMPTLZ_ISREPG1 {
    () => {
        CMPTLZ_ISREPG0!() + CMPTLZ_MKSTATE_NUM!()
    };
}
pub(crate) use CMPTLZ_ISREPG1;

macro_rules! CMPTLZ_ISREPG2 {
    () => {
        CMPTLZ_ISREPG1!() + CMPTLZ_MKSTATE_NUM!()
    };
}
pub(crate) use CMPTLZ_ISREPG2;

macro_rules! CMPTLZ_POSSLOT {
    () => {
        CMPTLZ_ISREPG2!() + CMPTLZ_MKSTATE_NUM!()
    };
}
pub(crate) use CMPTLZ_POSSLOT;

macro_rules! CMPTLZ_LITERAL {
    () => {
        CMPTLZ_POSSLOT!() + (CMPTLZ_LEN_CONDITION_TO_POSSLOT!() << CMPTLZ_POS_SLOT_BITS!())
    };
}
pub(crate) use CMPTLZ_LITERAL;

macro_rules! NUM_BASE_PROBS {
    () => {
        CMPTLZ_LITERAL!() + CMPTLZ_OFFSET!()
    };
}
pub(crate) use NUM_BASE_PROBS;

macro_rules! CMPTLZ_REP4 {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_REP4;

macro_rules! CMPTLZ_REP3 {
    () => {
        3
    };
}
pub(crate) use CMPTLZ_REP3;

macro_rules! CMPTLZ_REP2 {
    () => {
        2
    };
}
pub(crate) use CMPTLZ_REP2;

macro_rules! CMPTLZ_MIN_DICTSIZE {
    () => {
        1024
    };
}
pub(crate) use CMPTLZ_MIN_DICTSIZE;

macro_rules! CMPTLZ_MAX_DICTSIZE {
    () => {
        128 * 1024 * 1024
    };
}
pub(crate) use CMPTLZ_MAX_DICTSIZE;

macro_rules! CMPTLZ_UINT32_MAX {
    () => {
        (-1isize) as u32
    };
}
pub(crate) use CMPTLZ_UINT32_MAX;

macro_rules! CMPT_EMPTY_HASH_VALUE {
    () => {
        0
    };
}
pub(crate) use CMPT_EMPTY_HASH_VALUE;

macro_rules! CMPTLZ_HASH_2_SIZE {
    () => {
        1 << 10
    };
}
pub(crate) use CMPTLZ_HASH_2_SIZE;

macro_rules! CMPTLZ_HASH_3_SIZE {
    () => {
        1 << 16
    };
}
pub(crate) use CMPTLZ_HASH_3_SIZE;

macro_rules! CMPTLZ_HASH_2_MASK {
    () => {
        (CMPTLZ_HASH_2_SIZE!() - 1)
    };
}
pub(crate) use CMPTLZ_HASH_2_MASK;

macro_rules! CMPTLZ_HASH_3_MASK {
    () => {
        (CMPTLZ_HASH_3_SIZE!() - 1)
    };
}
pub(crate) use CMPTLZ_HASH_3_MASK;

macro_rules! CMPTLZ_FIX_3_HASH {
    () => {
        CMPTLZ_HASH_2_SIZE!()
    };
}
pub(crate) use CMPTLZ_FIX_3_HASH;

macro_rules! CMPTLZ_FIX_4_HASH {
    () => {
        CMPTLZ_HASH_2_SIZE!() + CMPTLZ_HASH_3_SIZE!()
    };
}
pub(crate) use CMPTLZ_FIX_4_HASH;

macro_rules! CMPT_RC_MIN_RANGE {
    () => {
        1 << 24
    };
}
pub(crate) use CMPT_RC_MIN_RANGE;

macro_rules! CMPT_NUM_LEN_POS_STATE {
    () => {
        4
    };
}
pub(crate) use CMPT_NUM_LEN_POS_STATE;

macro_rules! CMPTLZ_NUM_REPS {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_NUM_REPS;

macro_rules! CMPTLZ_NUM_STATES {
    () => {
        12
    };
}
pub(crate) use CMPTLZ_NUM_STATES;

macro_rules! CMPTLZ_MATCH_LEN_MIN {
    () => {
        2
    };
}
pub(crate) use CMPTLZ_MATCH_LEN_MIN;

macro_rules! CMPTLZ_PB_MAX {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_PB_MAX;

macro_rules! CMPTLZ_LC_MAX {
    () => {
        8
    };
}
pub(crate) use CMPTLZ_LC_MAX;

macro_rules! CMPTLZ_LP_MAX {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_LP_MAX;

macro_rules! CMPTLZ_LCLP_MAX {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_LCLP_MAX;

macro_rules! CMPTLZ_NUM_PB_STATES_MAX {
    () => {
        1 << CMPTLZ_PB_MAX!()
    };
}
pub(crate) use CMPTLZ_NUM_PB_STATES_MAX;

macro_rules! CMPTLZ_LIT_MAX_SIZE {
    () => {
        0x300
    };
}
pub(crate) use CMPTLZ_LIT_MAX_SIZE;

macro_rules! CMPTLZ_PROB_MAX_NUM {
    () => {
        2048
    };
}
pub(crate) use CMPTLZ_PROB_MAX_NUM;

macro_rules! CMPTLZ_PROB_INIT {
    () => {
        1024
    };
}
pub(crate) use CMPTLZ_PROB_INIT;

macro_rules! CMPTLZ_RC_BUFFER_SIZE {
    () => {
        1 << 16
    };
}
pub(crate) use CMPTLZ_RC_BUFFER_SIZE;

macro_rules! CMPT_DIST_LIMIT_2 {
    () => {
        128
    };
}
pub(crate) use CMPT_DIST_LIMIT_2;

macro_rules! CMPTLZ_DIST_STATE_TOTAL {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_DIST_STATE_TOTAL;

macro_rules! CMPTLZ_ALIGN_BITS {
    () => {
        4
    };
}
pub(crate) use CMPTLZ_ALIGN_BITS;

macro_rules! CMPTLZ_DIST_SLOT_BITS {
    () => {
        6
    };
}
pub(crate) use CMPTLZ_DIST_SLOT_BITS;

macro_rules! CMPT_INFINITY_PRICE {
    () => {
        (1isize << 30) as u32
    };
}
pub(crate) use CMPT_INFINITY_PRICE;

macro_rules! CMPT_PRICE_BITS_MOVING_NUM {
    () => {
        4
    };
}
pub(crate) use CMPT_PRICE_BITS_MOVING_NUM;

macro_rules! CMPT_PRIICE_TABLE_SIZE {
    () => {
        CMPTLZ_PROB_MAX_NUM!() >> CMPT_PRICE_BITS_MOVING_NUM!()
    };
}
pub(crate) use CMPT_PRIICE_TABLE_SIZE;

macro_rules! CMPT_PRICE_COUNT {
    () => {
        64
    };
}
pub(crate) use CMPT_PRICE_COUNT;

macro_rules! CMPT_DOUBLE {
    () => {
        2
    };
}
pub(crate) use CMPT_DOUBLE;

macro_rules! CMPT_LEN_LOW_BITS {
    () => {
        3
    };
}
pub(crate) use CMPT_LEN_LOW_BITS;

macro_rules! CMPT_LEN_MID_BITS {
    () => {
        3
    };
}
pub(crate) use CMPT_LEN_MID_BITS;

macro_rules! CMPT_LEN_HIGH_BITS {
    () => {
        8
    };
}
pub(crate) use CMPT_LEN_HIGH_BITS;

macro_rules! CMPT_LEN_BOUND {
    () => {
        8
    };
}
pub(crate) use CMPT_LEN_BOUND;

macro_rules! CMPT_MF_LONGEST_MATCH {
    () => {
        273
    };
}
pub(crate) use CMPT_MF_LONGEST_MATCH;

macro_rules! CMPT_MF_HASH_TABLE_SIZE {
    () => {
        256
    };
}
pub(crate) use CMPT_MF_HASH_TABLE_SIZE;

macro_rules! CMPT_MF_BASE_DEPTH {
    () => {
        16
    };
}
pub(crate) use CMPT_MF_BASE_DEPTH;

macro_rules! CMPT_MF_MATCH_2_BYTES {
    () => {
        2
    };
}
pub(crate) use CMPT_MF_MATCH_2_BYTES;

macro_rules! CMPT_MF_MATCH_3_BYTES {
    () => {
        3
    };
}
pub(crate) use CMPT_MF_MATCH_3_BYTES;

macro_rules! CMPT_DP_OPTMAX {
    () => {
        1 << 11
    };
}
pub(crate) use CMPT_DP_OPTMAX;

macro_rules! CMPT_ONE_BLOCK_MAX_SIZE {
    () => {
        1 << 17
    };
}
pub(crate) use CMPT_ONE_BLOCK_MAX_SIZE;

macro_rules! CMPTLZ_WRITE32BIT {
    ($ptr:expr, $val:expr) => {
        ($ptr.cast::<Ptr<CmptlzUnalignU32>>()).v = $val
    };
}
pub(crate) use CMPTLZ_WRITE32BIT;

macro_rules! CMPTLZ_ERROR_CONVERT {
    ($x:expr) => {
        (CMPTLZ_MODULE!() << 16) | $x
    };
}
pub(crate) use CMPTLZ_ERROR_CONVERT;

macro_rules! CMPTLZ_HANDLE_CONVERT {
    ($x:expr) => {
        (CMPTLZ_MODULE!() << 16) | (($x as u32) << 8) as i32
    };
}
pub(crate) use CMPTLZ_HANDLE_CONVERT;

macro_rules! CMPTLZ_LIKELY {
    ($expr:expr) => {
        $expr
    };
}
pub(crate) use CMPTLZ_LIKELY;

macro_rules! CMPTLZ_UNLIKELY {
    ($expr:expr) => {
        $expr
    };
}
pub(crate) use CMPTLZ_UNLIKELY;

macro_rules! CMPTLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        CmptlzLogWrite(($error_code as usize).cast(), __FUNCTION__!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr),*) => {
        CmptlzLogWrite(($error_code as usize).cast(), __FUNCTION__!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    }
}
pub(crate) use CMPTLZ_LOG;

macro_rules! CMPTLZ_CALC_POS_STATE {
    ($procPos:expr, $pbMask:expr) => {
        (($procPos) & ($pbMask)) << 4
    };
}
pub(crate) use CMPTLZ_CALC_POS_STATE;

macro_rules! CMPTLZ_RANGE_NORMALIZE {
    ($range:expr, $rangeCode:expr, $bufToDec:expr) => {
        if $range < CMPTLZ_RANGE_DOWN_LIMIT!() {
            $range <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode |= (*$bufToDec.plus_plus()) as u32;
        }
    };
}
pub(crate) use CMPTLZ_RANGE_NORMALIZE;

macro_rules! CMPTLZ_IS_THE_BIT_0 {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr) => {{
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probSlot as u32);
        $rangeCode < $rangeBound
    }};
}
pub(crate) use CMPTLZ_IS_THE_BIT_0;

macro_rules! CMPTLZ_RANGE_UPDATE_0 {
    ($prob:expr, $range:expr, $rangeBound:expr) => {
        $range = $rangeBound;
        *$prob =
            (*$prob + ((CMPTLZ_PROB_LG!() - *$prob) >> CMPTLZ_RANGE_CODE_SIZE!())) as CmptLzDecProb;
    };
}
pub(crate) use CMPTLZ_RANGE_UPDATE_0;

macro_rules! CMPTLZ_RANGE_UPDATE_1 {
    ($prob:expr, $range:expr, $rangeCode:expr, $rangeBound:expr) => {
        $range -= $rangeBound;
        $rangeCode -= $rangeBound;
        *$prob = (*$prob - (*$prob >> CMPTLZ_RANGE_CODE_SIZE!())) as CmptLzDecProb;
    };
}
pub(crate) use CMPTLZ_RANGE_UPDATE_1;

macro_rules! CMPTLZ_NORMAL_BIT_DEC {
    ($probLit:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probLit as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!($probLit, $range, $rangeBound);
            $decSym = $decSym << 1;
        } else {
            CMPTLZ_RANGE_UPDATE_1!($probLit, $range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
    };
}
pub(crate) use CMPTLZ_NORMAL_BIT_DEC;

macro_rules! CMPTLZ_MATCH_BIT_DEC {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $matchSym:expr, $offset:expr, $bit:expr, $bufToDec:expr) => {
        $matchSym <<= 1;
        $bit = $offset;
        $offset &= $matchSym as u32;
        let mut probLit = $probSlot + ($offset + $bit + $decSym);
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*probLit as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!(probLit, $range, $rangeBound);
            $decSym = $decSym << 1;
            $offset ^= $bit;
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probLit, $range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
        CMPTLZ_RANGE_NORMALIZE!($range, $rangeCode, $bufToDec);
    };
}
pub(crate) use CMPTLZ_MATCH_BIT_DEC;

macro_rules! CMPTLZ_DIST_BIT_DEC {
    ($probDist:expr, $probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decDist:expr, $decBit:expr) => {
        $probDist = $probSlot + $decDist;
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probDist as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!($probDist, $range, $rangeBound);
            $decDist += $decBit;
        } else {
            CMPTLZ_RANGE_UPDATE_1!($probDist, $range, $rangeCode, $rangeBound);
            $decDist += $decBit * 2;
        }
    };
}
pub(crate) use CMPTLZ_DIST_BIT_DEC;

macro_rules! CMPTLZ_LEN_BIT_DEC {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $bufToDec:expr) => {
        CMPTLZ_NORMAL_BIT_DEC!($probSlot, $range, $rangeCode, $rangeBound, $decSym);
        CMPTLZ_RANGE_NORMALIZE!($range, $rangeCode, $bufToDec);
    };
}
pub(crate) use CMPTLZ_LEN_BIT_DEC;

macro_rules! CMPTLZ_POSSLOT_BIT_DEC {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $bufToDec:expr) => {
        CMPTLZ_NORMAL_BIT_DEC!($probSlot, $range, $rangeCode, $rangeBound, $decSym);
        CMPTLZ_RANGE_NORMALIZE!($range, $rangeCode, $bufToDec);
    };
}
pub(crate) use CMPTLZ_POSSLOT_BIT_DEC;

macro_rules! CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0 {
    ($range:expr, $rangeBound:expr) => {
        $range = $rangeBound;
    };
}
pub(crate) use CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0;

macro_rules! CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1 {
    ($range:expr, $rangeCode:expr, $rangeBound:expr) => {
        $range -= $rangeBound;
        $rangeCode -= $rangeBound;
    };
}
pub(crate) use CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1;

macro_rules! CMPTLZ_RANGE_TRY_NORMALIZE {
    ($range:expr, $rangeCode:expr, $bufTryDec:expr, $bufLimit:expr) => {
        if $range < CMPTLZ_RANGE_DOWN_LIMIT!() {
            if $bufTryDec >= $bufLimit {
                return CMPTLZ_DEC_INPUT_EOF!();
            }
            $range <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode |= *$bufTryDec.plus_plus() as u32;
        }
    };
}
pub(crate) use CMPTLZ_RANGE_TRY_NORMALIZE;

macro_rules! CMPTLZ_SINGLE_BIT_TRY_DEC {
    ($range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $probSym:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probSym as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!($range, $rangeBound);
            $decSym = $decSym << 1;
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!($range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
    };
}
pub(crate) use CMPTLZ_SINGLE_BIT_TRY_DEC;

macro_rules! CMPTLZ_MATCH_BIT_TRY_DEC {
    ($range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $probSym:expr, $offset:expr, $bit:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probSym as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!($range, $rangeBound);
            $decSym = $decSym << 1;
            $offset ^= $bit;
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!($range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
    };
}
pub(crate) use CMPTLZ_MATCH_BIT_TRY_DEC;

macro_rules! CMPTLZ_SET_DICTSIZE_BY_LEVEL {
    ($level:expr, $dictSize:expr) => {
        $dictSize = if $level <= 5 {
            1 << ($level * 2 + 14)
        } else if $level <= 7 {
            1 << 25
        } else {
            1 << 26
        };
    };
}
pub(crate) use CMPTLZ_SET_DICTSIZE_BY_LEVEL;

macro_rules! CMPTLZ_SET_FB_BY_LEVEL {
    ($level:expr, $fastBytes:expr) => {
        $fastBytes = if $level < 7 { 32 } else { 64 }
    };
}
pub(crate) use CMPTLZ_SET_FB_BY_LEVEL;

macro_rules! CMPTLZ_FIND_MIN {
    ($x:expr, $y:expr) => {
        if $x < $y {
            $x
        } else {
            $y
        }
    };
}
pub(crate) use CMPTLZ_FIND_MIN;

macro_rules! CMPTLZ_FIND_MAX {
    ($x:expr, $y:expr) => {
        if $x > $y {
            $x
        } else {
            $y
        }
    };
}
pub(crate) use CMPTLZ_FIND_MAX;

macro_rules! NOT_EQUAL_2_BYTES {
    ($a:expr, $b:expr) => {
        $a[0] != $b[0] || $a[1] != $b[1]
    };
}
pub(crate) use NOT_EQUAL_2_BYTES;

macro_rules! CMPTLZ_RETURN_IF_NOT_OK {
    ($res:expr) => {
        if CMPTLZ_UNLIKELY!($res != CMPT_OK!()) {
            return $res;
        }
    };
}
pub(crate) use CMPTLZ_RETURN_IF_NOT_OK;

macro_rules! CMPT_GET_DIST_STATE {
    ($len:expr) => {
        if $len < 4 + CMPTLZ_MATCH_LEN_MIN!() {
            $len - CMPTLZ_MATCH_LEN_MIN!()
        } else {
            4 - 1
        }
    };
}
pub(crate) use CMPT_GET_DIST_STATE;

macro_rules! CMPT_STATE_UPDATE_WHEN_LIT {
    ($state:expr) => {
        $state = if $state <= SHORTREP_LIT_LIT!() {
            LIT_LIT!()
        } else if $state <= LIT_SHORTREP!() {
            $state - 3
        } else {
            $state - 6
        }
    };
}
pub(crate) use CMPT_STATE_UPDATE_WHEN_LIT;

macro_rules! CMPT_STATE_UPDATE_WHEN_MATCH {
    ($state:expr) => {{
        $state = if $state < 7 {
            LIT_MATCH!()
        } else {
            NOTLIT_MATCH!()
        };
        $state
    }};
}
pub(crate) use CMPT_STATE_UPDATE_WHEN_MATCH;

macro_rules! CMPT_STATE_UPDATE_WHEN_LONGREP {
    ($state:expr) => {{
        $state = if $state < 7 {
            LIT_LONGREP!()
        } else {
            NOTLIT_REP!()
        };
        $state
    }};
}
pub(crate) use CMPT_STATE_UPDATE_WHEN_LONGREP;

macro_rules! CMPT_STATE_UPDATE_WHEN_SHORTREP {
    ($state:expr) => {{
        $state = if $state < 7 {
            LIT_SHORTREP!()
        } else {
            NOTLIT_REP!()
        };
        $state
    }};
}
pub(crate) use CMPT_STATE_UPDATE_WHEN_SHORTREP;

macro_rules! CMPT_HASH_MASK_CALC {
    ($hashMask:expr) => {
        $hashMask |= $hashMask >> 1;
        $hashMask |= $hashMask >> 2;
        $hashMask |= $hashMask >> 4;
        $hashMask |= $hashMask >> 8;
        $hashMask >>= 1;
        $hashMask |= 0xFFFF;
        if $hashMask > (1 << 24) {
            $hashMask >>= 1;
        }
    };
}
pub(crate) use CMPT_HASH_MASK_CALC;

macro_rules! CMPT_HASH_4_CALC {
    ($mf:expr, $cur:expr, $temp:expr, $hash2Value:expr, $hash3Value:expr, $hashValue:expr) => {
        $temp = $mf.hashRootTable[$cur[0]] ^ $cur[1] as u32;
        $hash2Value = $temp & CMPTLZ_HASH_2_MASK!();
        $hash3Value = ($temp ^ (($cur[2] as u32) << 8)) & CMPTLZ_HASH_3_MASK!();
        $hashValue =
            ($temp ^ (($cur[2] as u32) << 8) ^ ($mf.hashRootTable[$cur[3]] << 5)) & $mf.hashMask;
    };
}
pub(crate) use CMPT_HASH_4_CALC;

macro_rules! CMPT_HASH_UPDATE {
    ($mf:expr, $hash2Value:expr, $hash3Value:expr, $hashValue:expr, $pos:expr) => {
        $mf.hash[$hash2Value] = $pos;
        $mf.hash[CMPTLZ_FIX_3_HASH!() + $hash3Value] = $pos;
        $mf.hash[CMPTLZ_FIX_4_HASH!() + $hashValue] = $pos;
    };
}
pub(crate) use CMPT_HASH_UPDATE;

macro_rules! CMPT_HASH_FIND_2_BYTES {
    ($mf:expr, $delta2:expr, $longestLen:expr, $matchesCount:expr, $cur:expr, $matches:expr) => {
        if $delta2 < $mf.cycleSize && *($cur - $delta2) == *$cur {
            $longestLen = CMPT_MF_MATCH_2_BYTES!();
            $matches[0].len = CMPT_MF_MATCH_2_BYTES!();
            $matches[0].dist = $delta2 - 1;
            $matchesCount = 1;
        }
    };
}
pub(crate) use CMPT_HASH_FIND_2_BYTES;

macro_rules! CMPT_HASH_FIND_3_BYTES {
    ($mf:expr, $delta2:expr, $delta3:expr, $longestLen:expr, $matchesCount:expr, $cur:expr, $matches:expr) => {
        if $delta2 != $delta3 && $delta3 < $mf.cycleSize && *($cur - $delta3) == *$cur {
            $longestLen = CMPT_MF_MATCH_3_BYTES!();
            $matches[$matchesCount.plus_plus()].dist = $delta3 - 1;
            $delta2 = $delta3;
        }
    };
}
pub(crate) use CMPT_HASH_FIND_3_BYTES;

macro_rules! CMPT_MF_MOVE_POS {
    ($mf:expr) => {
        $mf.readPos.plus_plus();
        $mf.cyclePos.plus_plus();
        $mf.cyclePos = if $mf.cyclePos == $mf.cycleSize {
            0
        } else {
            $mf.cyclePos
        };
        if CMPTLZ_UNLIKELY!($mf.readPos + $mf.offset == CMPTLZ_UINT32_MAX!()) {
            CmptMfMovePos($mf.cast());
        }
    };
}
pub(crate) use CMPT_MF_MOVE_POS;

macro_rules! CMPT_MF_LEFT_SON_UPDATE {
    ($ptr1:expr, $pair:expr, $curMatch:expr, $len1:expr, $len:expr) => {
        *$ptr1 = $curMatch;
        $ptr1 = $pair + 1;
        $curMatch = *$ptr1;
        $len1 = $len;
    };
}
pub(crate) use CMPT_MF_LEFT_SON_UPDATE;

macro_rules! CMPT_MF_RIGHT_SON_UPDATE {
    ($ptr0:expr, $pair:expr, $curMatch:expr, $len0:expr, $len:expr) => {
        *$ptr0 = $curMatch;
        $ptr0 = $pair;
        $curMatch = *$ptr0;
        $len0 = $len;
    };
}
pub(crate) use CMPT_MF_RIGHT_SON_UPDATE;

macro_rules! CMPT_LIT_SUBCODER {
    ($probs:expr, $litCtx:expr, $lpMask:expr, $pos:expr, $prevByte:expr) => {
        $probs[(($pos & $lpMask) << $litCtx) + (($prevByte as u32) >> (8u32 - $litCtx))]
    };
}
pub(crate) use CMPT_LIT_SUBCODER;

macro_rules! GET_LEN_TO_POS_STATE {
    ($len:expr) => {
        if $len < CMPT_NUM_LEN_POS_STATE!() + 1 {
            $len - 2
        } else {
            CMPT_NUM_LEN_POS_STATE!() - 1
        }
    };
}
pub(crate) use GET_LEN_TO_POS_STATE;

macro_rules! CMPT_RC_BREAK_CHECK {
    ($rcCtx:expr, $buf:expr, $res:expr) => {
        if $buf == ($rcCtx.bufBase + CMPTLZ_RC_BUFFER_SIZE!()) {
            $res = CmptRcFlush64Kb($rcCtx.cast());
            CMPTLZ_RETURN_IF_NOT_OK!($res);
        }
    };
}
pub(crate) use CMPT_RC_BREAK_CHECK;

macro_rules! CMPT_RC_BREAK_SHIFTING {
    ($rcCtx:expr, $buf:expr, $res:expr) => {
        CMPT_RC_BREAK_CHECK!($rcCtx, $buf, $res);
        if $rcCtx.cacheSize == 0 {
            return CMPT_OK!();
        }
    };
}
pub(crate) use CMPT_RC_BREAK_SHIFTING;

macro_rules! CMPT_RC_NORMALIZE {
    ($rcCtx:expr, $range:expr, $shiftRes:expr) => {
        if $range < CMPT_RC_MIN_RANGE!() {
            $range <<= 8;
            $shiftRes = CmptRcShiftLow($rcCtx);
        }
    };
}
pub(crate) use CMPT_RC_NORMALIZE;

macro_rules! CMPT_RC_GET_NEWBOUND {
    ($prob:expr, $bit0Prob:expr, $range:expr, $newBound:expr) => {
        $bit0Prob = *$prob as u32;
        $newBound = ($range >> 11) * $bit0Prob;
    };
}
pub(crate) use CMPT_RC_GET_NEWBOUND;

macro_rules! CMPT_RC_BIT_PROCESS {
    ($rcCtx:expr, $prob:expr, $bit:expr, $bit0Prob:expr, $range:expr, $newBound:expr, $shiftRes:expr) => {{
        let mut mask = 0 - ($bit as u32);
        CMPT_RC_GET_NEWBOUND!($prob, $bit0Prob, $range, $newBound);
        $range &= mask;
        mask &= $newBound;
        $range -= mask;
        $rcCtx.low += mask as u64;
        mask = ($bit as u32) - 1;
        $range += $newBound & mask;
        mask = (CMPTLZ_PROB_MAX_NUM!() - ((1 << 5) - 1)) & mask;
        mask += (1 << 5) - 1;
        $bit0Prob += (mask - $bit0Prob) >> 5;
        *$prob = $bit0Prob as CmptlzProb;
        CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
    }};
}
pub(crate) use CMPT_RC_BIT_PROCESS;

macro_rules! CMPT_RC_BIT_0 {
    ($prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr) => {
        $range = $newBound;
        *$prob = ($bit0Prob + ((CMPTLZ_PROB_MAX_NUM!() - $bit0Prob) >> 5)).cast();
    };
}
pub(crate) use CMPT_RC_BIT_0;

macro_rules! CMPT_RC_BIT_1 {
    ($rcCtx:expr, $prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr) => {
        $range -= $newBound;
        $rcCtx.low += $newBound as u64;
        *$prob = ($bit0Prob - ($bit0Prob >> 5)).cast();
    };
}
pub(crate) use CMPT_RC_BIT_1;

macro_rules! CMPT_RC_BIT_0_PROCESS {
    ($rcCtx:expr, $prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr, $shiftRes:expr) => {
        CMPT_RC_BIT_0!($prob, $newBound, $range, $bit0Prob);
        CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
    };
}
pub(crate) use CMPT_RC_BIT_0_PROCESS;

macro_rules! CMPT_RC_BIT_1_PROCESS {
    ($rcCtx:expr, $prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr, $shiftRes:expr) => {
        CMPT_RC_BIT_1!($rcCtx, $prob, $newBound, $range, $bit0Prob);
        CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
    };
}
pub(crate) use CMPT_RC_BIT_1_PROCESS;

macro_rules! CMPT_LIT_PROB_GET {
    ($encCtx:expr, $litProb:expr, $pos:expr, $prevByte:expr) => {
        $litProb
            + (3 * (((($pos << 8) + $prevByte as u32) & $encCtx.lpMask as u32)
                << $encCtx.litMarcov.lcBits))
    };
}
pub(crate) use CMPT_LIT_PROB_GET;

pub fn CmptlzIsLE() -> i32 {
    let mut n: i32 = 1;
    return (*c_ref!(n).cast::<Ptr<u8>>()).cast::<i32>();
}

pub fn CmptlzSwap32(mut val: u32) -> u32 {
    return ((0xff000000 & (val << 24))
        | (0x000000ff & (val >> 24))
        | (0x00ff0000 & (val << 8))
        | (0x0000ff00 & (val >> 8)))
        .cast();
}

pub fn CmptlzWriteLE32Bit(mut addr: Ptr<Void>, mut val: u32) {
    if (CmptlzIsLE() != 0).as_bool() {
        CMPTLZ_WRITE32BIT!(addr, val);
    } else {
        CMPTLZ_WRITE32BIT!(addr, CmptlzSwap32(val.cast()));
    }
}

pub fn CmptlzLogWrite(
    mut errorCode: usize,
    mut funcName: Ptr<u8>,
    mut line: u16,
    mut fmt: Ptr<u8>,
    mut alist: VaList,
) {
    // alist already initialized at parameter list
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut ret: i32 = Default::default();
    let mut len: usize = Default::default();
    let mut func: CmptlzLogFunc = *g_cmptlzLogFunc.lock();
    if (func == NULL!()).as_bool() {
        return;
    }
    ret = c_snprintf_s!(
        output,
        LOG_BUF_SIZE!(),
        LOG_BUF_SIZE!() - 1,
        cstr!("\n[Cmptlz-Log] Func={}, Line={}, Error={}\n"),
        funcName,
        line,
        errorCode
    );
    if (ret < 0).as_bool() {
        return;
    }
    len = ret.cast();
    // va_start not needed
    ret = c_vsnprintf_s!(
        output.cast::<Ptr<u8>>() + len,
        LOG_BUF_SIZE!() - len,
        LOG_BUF_SIZE!() - len - 1,
        fmt,
        alist
    );
    // va_end not needed
    if ret < 0 {
        return;
    }
    func(output.cast(), c_strlen!(output) + 1);
}

pub fn CmptlzLogRegister(mut func: CmptlzLogFunc) {
    *g_cmptlzLogFunc.lock() = func.cast();
}

pub fn CmptLzPropsDecode(
    mut protData: Ptr<u8>,
    mut protSize: u8,
    mut decProt: Ptr<CmptLzDecProt>,
) -> i32 {
    let mut dictSize: u32;
    if (protSize < CMPTLZ_PROPS_SIZE!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    } else {
        dictSize = ((protData[1] as u32)
            | ((protData[2] as u32) << 8)
            | ((protData[3] as u32) << 16)
            | ((protData[4] as u32) << 24))
            .cast();
    }
    if (dictSize < CMPTLZ_DICT_MIN_LEN!()) {
        dictSize = CMPTLZ_DICT_MIN_LEN!();
    }
    decProt.dicSize = dictSize;
    let mut firstData: u8 = protData[0];
    if (firstData >= (CMPTLZ_LIT_CTX_MAX!() * CMPTLZ_POS_STATE_MAX!() * CMPTLZ_LIT_POS_MAX!())) {
        return CMPT_ERROR_UNSUPPORTED!();
    }
    decProt.litCtx = (firstData % CMPTLZ_LIT_CTX_MAX!());
    firstData /= CMPTLZ_LIT_CTX_MAX!();
    decProt.posBits = (firstData / CMPTLZ_POS_STATE_MAX!());
    decProt.litPos = (firstData % CMPTLZ_LIT_POS_MAX!());
    return CMPT_OK!();
}

pub fn CmptLzDecInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    decCtx.dictPos = 0;
    decCtx.tempBufSize = 0;
    decCtx.processedPos = 0;
    decCtx.checkDicSize = 0;
    decCtx.remainLen = CMPTLZ_MATCH_MAX_LEN!() + 2;
}

pub fn CmptLzDecMemAlloc(
    mut memHook: Ptr<CmptLzMemHook>,
    mut memHandle: i32,
    mut allocSize: usize,
) -> Ptr<Void> {
    return (memHook.CmptLzAlloc)(memHandle, allocSize);
}

pub fn CmptLzDecMemFree(
    mut memHook: Ptr<CmptLzMemHook>,
    mut memHandle: i32,
    mut freeAddress: Ptr<Void>,
) {
    (memHook.CmptLzFree)(memHandle, freeAddress);
}

pub fn CmptLzDecFreeProbs(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) {
    if (decCtx.probs != NULL!()) {
        CmptLzDecMemFree(
            memHook,
            CMPTLZ_PROB_HANDLE!(),
            decCtx.probs.cast::<Ptr<Void>>(),
        );
        decCtx.probs = NULL!();
    }
}

pub fn CmptLzDecAllocateProbs(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut decProt: Ptr<CmptLzDecProt>,
    mut memHook: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut numProbs: u32 = CmptLzGetNumProbs(decProt);
    if (decCtx.probs == NULL!()) {
        decCtx.probs = CmptLzDecMemAlloc(
            memHook,
            CMPTLZ_PROB_HANDLE!(),
            (numProbs as usize * c_sizeof!(CmptLzDecProb)),
        )
        .cast::<Ptr<CmptLzDecProb>>();
    } else {
        if (numProbs != decCtx.numProbs) {
            CmptLzDecFreeProbs(decCtx, memHook);
            decCtx.probs = CmptLzDecMemAlloc(
                memHook,
                CMPTLZ_PROB_HANDLE!(),
                (numProbs as usize * c_sizeof!(CmptLzDecProb)),
            )
            .cast::<Ptr<CmptLzDecProb>>();
        }
    }
    if (decCtx.probs == NULL!()) {
        return CMPT_ERROR_MEM!();
    }
    decCtx.probsPlus1664 = (decCtx.probs + 1664);
    decCtx.numProbs = numProbs;
    return CMPT_OK!();
}

pub fn CmptLzDecConstruct(mut decCtx: Ptr<CmptLzDecCtx>) {
    decCtx.dict = NULL!();
    decCtx.probs = NULL!();
}

pub fn CmptLzDecode(
    mut pDecIn: Ptr<CmptLzDecIn>,
    mut pDecOut: Ptr<CmptLzDecOut>,
    mut protData: Ptr<u8>,
    mut finMode: EnCmptLzFinMode,
    mut finStatus: Ptr<EnCmptLzStatus>,
    mut memHook: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut res: i32;
    let mut inSize: usize = pDecIn.strInLen;
    let mut decProt: CmptLzDecProt = Default::default();
    let mut decCtx: CmptLzDecCtx = Default::default();
    decCtx.numProbs = 0;
    if (inSize < CMPTLZ_PROPS_SIZE!()) {
        return CMPT_ERROR_UNSUPPORTED!();
    }
    CmptLzDecConstruct(c_ref!(decCtx));
    res = CmptLzPropsDecode(protData, CMPTLZ_PROPS_SIZE!(), c_ref!(decProt));
    if (res != CMPT_OK!()) {
        return res;
    }
    res = CmptLzDecAllocateProbs(c_ref!(decCtx), c_ref!(decProt), memHook);
    if (res != CMPT_OK!()) {
        return res;
    }
    decCtx.prop = decProt;
    decCtx.dict = pDecOut.pDestOut;
    decCtx.dictBufSize = pDecOut.destOutLen;
    CmptLzDecInit(c_ref!(decCtx));
    *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
    res = CmptLzDecDecodeToDic(
        c_ref!(decCtx),
        pDecOut.destOutLen,
        pDecIn.pSrcIn,
        c_ref!(inSize),
        finMode,
        finStatus,
    );
    pDecIn.strInCostLen = inSize;
    pDecOut.destOutFillLen = decCtx.dictPos;
    CmptLzDecFreeProbs(c_ref!(decCtx), memHook);
    return res;
}

pub fn CmptLzGetProbsMatrix(mut decCtx: Ptr<CmptLzDecCtx>) -> Ptr<CmptLzDecProb> {
    return decCtx.probsPlus1664.cast();
}

pub fn CmptLzGetIsMatchProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_IS_MATCH!()).cast();
}

pub fn CmptLzGetIsRepProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREP!()).cast();
}

pub fn CmptLzGetIsRepG0Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG0!()).cast();
}

pub fn CmptLzGetIsRepG1Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG1!()).cast();
}

pub fn CmptLzGetIsRepG2Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG2!()).cast();
}

pub fn CmptLzGetIsRepG0LongProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_REP0_LONG!()).cast();
}

pub fn CmptLzGetLiteralProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_LITERAL!()).cast();
}

pub fn CmptLzGetPosSlotProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_POSSLOT!()).cast();
}

pub fn CmptLzGetSpecPosProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_SPEC_POS!()).cast();
}

pub fn CmptLzGetAilgnProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ALIGN!()).cast();
}

pub fn CmptLzGetRepLenCoderProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_REP_LEN_CODER!()).cast();
}

pub fn CmptLzGetMatchLenCoderProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_MATCH_LEN_CODER!()).cast();
}

pub fn CmptLzGetLenCondition(mut decLen: u32) -> u32 {
    return (if decLen < CMPTLZ_LEN_CONDITION_TO_POSSLOT!() {
        decLen
    } else {
        CMPTLZ_LEN_CONDITION_TO_POSSLOT!() - 1
    } << CMPTLZ_POS_SLOT_BITS!())
    .cast();
}

pub fn CmptLzGetBaseDistByPosSlot(mut posSlot: u32) -> u32 {
    return (2 | (posSlot & 1)).cast();
}

pub fn CmptLzGetNumProbs(mut decProt: Ptr<CmptLzDecProt>) -> u32 {
    return (NUM_BASE_PROBS!() + (0x300 << (decProt.litCtx + decProt.litPos))).cast();
}

pub fn CmptLzDistDecHelper(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut distDec: u32,
    mut bufToDec: Ptr<u8>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
    mut range: u32,
    mut rangeCode: u32,
    mut rangeBound: u32,
) {
    decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()].cast();
    decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1].cast();
    decCtx.reps[1] = decCtx.reps[0].cast();
    decCtx.reps[0] = (distDec + 1).cast();
    decCtx.buf = bufToDec.cast();
    decCtx.state = if decCtx.state < CMPTLZ_LIT_STATES!() {
        CMPTLZ_LIT_STATES!()
    } else {
        CMPTLZ_LIT_STATES!() + CMPTLZ_REP3!()
    };
    *pRange = range.cast();
    *pRangeCode = rangeCode.cast();
    *pRangeBound = rangeBound.cast();
}

pub fn CmptLzDistDec(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut probsMatrix: Ptr<CmptLzDecProb>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
    mut decLen: u32,
) -> usize {
    let mut assistBits: u32;
    let mut posSlot: u32 = 1;
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut distDec: u32;
    let mut probPosSlot: Ptr<CmptLzDecProb> =
        CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decLen);
    let mut i: i32 = 0;
    c_for!(i = 0; i < CMPTLZ_POS_SLOT_BITS!() as i32; i.suffix_plus_plus(); {
        CMPTLZ_POSSLOT_BIT_DEC!((probPosSlot + posSlot), range, rangeCode, rangeBound, posSlot, bufToDec);
    });
    posSlot -= 64;
    if (posSlot < CMPTLZ_LOW_POSSLOT!() as u32) {
        distDec = posSlot;
        CmptLzDistDecHelper(
            decCtx,
            distDec,
            bufToDec,
            pRange,
            pRangeCode,
            pRangeBound,
            range,
            rangeCode,
            rangeBound,
        );
        if (distDec == 0xFFFFFFFFu32) {
            return distDec as usize;
        } else {
            return (distDec + 1) as usize;
        }
    }
    let mut directBitNum: u32 = ((posSlot >> 1) - 1);
    distDec = CmptLzGetBaseDistByPosSlot(posSlot);
    if (posSlot < CMPTLZ_HIGH_POSSLOT!() as u32) {
        assistBits = 1;
        distDec <<= directBitNum;
        distDec += assistBits;
        probPosSlot = CmptLzGetSpecPosProb(probsMatrix);
        c_do!({
            if CMPTLZ_IS_THE_BIT_0!((probPosSlot + distDec), range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0!((probPosSlot + distDec), range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                distDec += assistBits;
                assistBits <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_1!((probPosSlot + distDec), range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                assistBits <<= 1;
                distDec += assistBits;
            }
        } while directBitNum.prefix_minus_minus() != 0);
        distDec -= assistBits;
    } else {
        directBitNum -= CMPTLZ_REP4!() as u32;
        c_do!({
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            range >>= 1;
            rangeCode -= range;
            assistBits = (0 - ((rangeCode >> 31) as u32));
            distDec = (distDec << 1) + (assistBits + 1);
            rangeCode += range & assistBits;
        } while directBitNum.prefix_minus_minus() != 0);
        let mut probDist: Ptr<CmptLzDecProb>;
        probPosSlot = CmptLzGetAilgnProb(probsMatrix);
        distDec <<= CMPTLZ_LARGE_DIST_LOW_BITS!() as u32;
        assistBits = 1;
        let mut cycleSym: u32 = 1;
        c_for!(i = 0; i < 3; i.suffix_plus_plus(); {
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            CMPTLZ_DIST_BIT_DEC!(probDist, probPosSlot, range, rangeCode, rangeBound, assistBits, cycleSym);
            cycleSym <<= 1;
        });
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probDist = (probPosSlot + assistBits);
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!() as u32) * (*probDist).cast::<u32>();
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probDist, range, rangeBound);
            assistBits -= 8;
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probDist, range, rangeCode, rangeBound);
        }
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        distDec |= assistBits;
    }
    CmptLzDistDecHelper(
        decCtx,
        distDec,
        bufToDec,
        pRange,
        pRangeCode,
        pRangeBound,
        range,
        rangeCode,
        rangeBound,
    );
    if (distDec == 0xFFFFFFFFu32) {
        return distDec as usize;
    } else {
        return (distDec + 1) as usize;
    }
}

pub fn CmptLzLenDec(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut probSlot: Ptr<CmptLzDecProb>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
    mut posState: u32,
) -> u32 {
    let mut decLen: u32 = 1;
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut probLen: Ptr<CmptLzDecProb> = (probSlot + CMPTLZ_LEN_CHOICE!());
    let mut i: i32 = 0;
    if CMPTLZ_IS_THE_BIT_0!(probLen, range, rangeCode, rangeBound) {
        CMPTLZ_RANGE_UPDATE_0!(probLen, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probLen = (probSlot + CMPTLZ_LOW_LENPROB_OFFSET!() + posState);
        c_for!(i = 0; i < CMPTLZ_LOW_LEN_BIT!(); i.suffix_plus_plus(); {
            CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
        });
        decLen -= 8;
    } else {
        CMPTLZ_RANGE_UPDATE_1!(probLen, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probLen = (probSlot + CMPTLZ_LEN_CHOICE2!());
        if CMPTLZ_IS_THE_BIT_0!(probLen, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probLen, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            probLen = (probSlot + (CMPTLZ_LEN_CHOICE2!() + posState));
            c_for!(i = 0; i < CMPTLZ_LOW_LEN_BIT!(); i.suffix_plus_plus(); {
                CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            });
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probLen, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            probLen = (probSlot + CMPTLZ_HIGH_LENPROB_OFFSET!());
            c_for!(i = 0; i < CMPTLZ_HIGH_LEN_BIT!(); i.suffix_plus_plus(); {
                CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            });
            decLen -= CMPTLZ_HIGH_LEN_CLASS!();
            decLen += (CMPTLZ_LOW_LEN_CLASS!() << 1);
        }
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx.buf = bufToDec;
    return decLen;
}

pub fn CmptLzDecByDistAndLen(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut matchDist: usize,
    mut matchLen: u32,
    mut dicPosLimit: usize,
) -> u32 {
    let mut dicCopyPos: usize;
    let mut dicPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut remainDicLen: u32 = (dicPosLimit - dicPos).cast::<u32>();
    let mut dict: Ptr<u8> = decCtx.dict;
    if (remainDicLen == 0) {
        return CMPT_ERROR_DATA!().cast::<u32>();
    }
    let mut decDicLen: u32 = if remainDicLen < matchLen {
        remainDicLen
    } else {
        matchLen
    };
    decCtx.processedPos += decDicLen;
    decCtx.dictPos += decDicLen.cast::<usize>();
    decCtx.remainLen = (matchLen - decDicLen);
    if (dicPos < matchDist) {
        dicCopyPos = (dictBufSize - matchDist + dicPos);
    } else {
        dicCopyPos = (dicPos - matchDist);
    }
    c_do!({
        dict[dicPos] = dict[dicCopyPos];
        dicPos += 1;
        if (dicCopyPos.suffix_plus_plus() == dictBufSize) {
            dicCopyPos = 0;
        }
    } while (decDicLen.prefix_minus_minus() != 0));
    return CMPT_OK!();
}

pub fn CmptLzShortRepDec(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut rep0: u32 = decCtx.reps[0];
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut dictPos: usize = decCtx.dictPos;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    dict[dictPos] = dict[dictPos - rep0 as usize
        + if dictPos < rep0 as usize {
            dictBufSize
        } else {
            0
        }];
    decCtx.dictPos += 1;
    decCtx.processedPos += 1;
    if (decCtx.state < CMPTLZ_LIT_STATES!()) {
        decCtx.state = 9;
    } else {
        decCtx.state = 11;
    }
}

pub fn CmptLzRepDec(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
    mut dicPosLimit: usize,
    mut posState: u32,
) -> u32 {
    let mut repLen: u32;
    let mut repDist: u32;
    let mut mkState: u32 = decCtx.state;
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    probSlot = (CmptLzGetIsRepG0Prob(probsMatrix) + mkState);
    if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = (CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState);
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            *pRange = range;
            *pRangeCode = rangeCode;
            *pRangeBound = rangeBound;
            decCtx.buf = bufToDec;
            CmptLzShortRepDec(decCtx);
            return CMPT_OK!();
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[0];
        }
    } else {
        CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = (CmptLzGetIsRepG1Prob(probsMatrix) + mkState);
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[1];
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            probSlot = (CmptLzGetIsRepG2Prob(probsMatrix) + mkState);
            if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
                CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP2!()];
            } else {
                CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP3!()];
                decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()];
            }
            decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1];
        }
        decCtx.reps[1] = decCtx.reps[0];
        decCtx.reps[0] = repDist;
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx.buf = bufToDec;
    decCtx.state = if mkState < CMPTLZ_LIT_STATES!() {
        8
    } else {
        11
    };
    probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
    repLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    return CmptLzDecByDistAndLen(decCtx, repDist.cast(), (repLen + 2), dicPosLimit);
}

pub fn CmptLzMatchDec(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
    mut dicPosLimit: usize,
    mut posState: u32,
) -> u32 {
    let mut matchLen: u32;
    let mut matchDist: usize;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
    matchLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);

    matchDist = CmptLzDistDec(
        decCtx,
        probsMatrix,
        pRange,
        pRangeCode,
        pRangeBound,
        matchLen,
    );

    if (matchDist > decCtx.dictBufSize) {
        if (matchDist == 0xFFFFFFFF) {
            decCtx.remainLen = CMPTLZ_MATCH_MAX_LEN!();
            decCtx.state -= CMPTLZ_MKSTATE_NUM!();
            return CMPT_OK!().cast::<u32>();
        } else {
            return CMPT_ERROR_DATA!().cast::<u32>();
        }
    }
    return CmptLzDecByDistAndLen(decCtx, matchDist, (matchLen + 2), dicPosLimit);
}

pub fn CmptLzLitDec(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut pRange: Ptr<u32>,
    mut pRangeCode: Ptr<u32>,
    mut pRangeBound: Ptr<u32>,
) -> u32 {
    let mut decSym: u32 = 1;
    let mut mkState: u32 = decCtx.state;
    let mut procPos: u32 = decCtx.processedPos;
    let mut checkDicSize: u32 = decCtx.checkDicSize;
    let mut litCtx: u32 = decCtx.prop.litCtx.cast();
    let mut litPosMask: u32 = ((0x100 as u32) << decCtx.prop.litPos) - ((0x100 as u32) >> litCtx);
    let mut probLit: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut dictPos: usize = decCtx.dictPos;
    let mut range: u32 = *pRange;
    let mut rangeBound: u32 = *pRangeBound;
    let mut rangeCode: u32 = *pRangeCode;
    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0) || (checkDicSize != 0) {
        probSlot += (CMPTLZ_REP3!() as u32)
            * ((((procPos << 8)
                + dict[(if dictPos == 0 { dictBufSize } else { dictPos }) - 1].cast::<u32>())
                & litPosMask)
                << litCtx);
    }
    let mut i: i32 = 0;
    if (mkState < CMPTLZ_LIT_STATES!()) {
        mkState -= if (mkState < 4) { mkState } else { 3 };
        c_for!(; i < 8; i.suffix_plus_plus(); {
            CMPTLZ_NORMAL_BIT_DEC!((probSlot + decSym.cast::<u16>()), range, rangeCode, rangeBound, decSym);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        });
    } else {
        let mut bit: u32 = Default::default();
        let mut offset: u32 = 0x100;
        let mut rep0: usize = decCtx.reps[0].cast::<usize>();
        let mut matchSym: u32 =
            dict[dictPos - rep0 + (if (dictPos < rep0) { dictBufSize } else { 0 })].cast();
        mkState -= if (mkState < 10) { CMPTLZ_REP3!() } else { 6 };
        c_for!(; i < 8; i.suffix_plus_plus(); {
            CMPTLZ_MATCH_BIT_DEC!(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec);
        });
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    dict[dictPos] = decSym.cast::<u8>();
    dictPos += 1;
    decCtx.processedPos += 1;
    decCtx.state = mkState;
    decCtx.dictPos = dictPos;
    decCtx.buf = bufToDec;
    return CMPT_OK!();
}

pub fn CmptLzDecDirectProcess(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut dicPosLimit: usize,
    mut bufLimit: Ptr<u8>,
) -> i32 {
    let mut decRes: u32 = Default::default();
    let mut pbMask: u32 = ((1 as u32) << decCtx.prop.posBits) - 1;
    let mut procPos: u32;
    let mut mkState: u32;
    let mut posState: u32;
    let mut range: u32 = decCtx.range;
    let mut rangeCode: u32 = decCtx.code;
    let mut rangeBound: u32 = 0;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    c_do!({
        procPos = decCtx.processedPos;
        mkState = decCtx.state;
        posState = CMPTLZ_CALC_POS_STATE!(procPos, pbMask);
        probSlot = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
            decRes = CmptLzLitDec(decCtx, c_ref!(range), c_ref!(rangeCode), c_ref!(rangeBound));
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
            probSlot = CmptLzGetIsRepProb(probsMatrix) + mkState;
            if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
                decRes = CmptLzMatchDec(decCtx, c_ref!(range), c_ref!(rangeCode), c_ref!(rangeBound), dicPosLimit, posState);
            } else {
                CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, decCtx.buf);
                decRes = CmptLzRepDec(decCtx, c_ref!(range), c_ref!(rangeCode), c_ref!(rangeBound), dicPosLimit, posState);
            }
            if (decRes != CMPT_OK!()) {
                break;
            }
        }
    } while (decCtx.dictPos < dicPosLimit) && (decCtx.buf < bufLimit) && (decCtx.remainLen < CMPTLZ_MATCH_MAX_LEN!() ));
    decCtx.range = range;
    decCtx.code = rangeCode;
    return decRes.cast::<i32>();
}

pub fn CmptLzTryDecLenAndDist(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut mkState: u32,
    mut range: u32,
    mut rangeCode: u32,
    mut rangeBound: u32,
    mut probSlot: Ptr<CmptLzDecProb>,
    mut bufTryDec: Ptr<u8>,
    mut pbufLimit: Ptr<Ptr<u8>>,
) -> i32 {
    let mut offset: u32;
    let mut bits2BeDec: u32;
    let mut pbMask: u32 = ((1 as u32) << decCtx.prop.posBits) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    let mut probBit: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut probLen: Ptr<CmptLzDecProb>;
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    probLen = probSlot + CMPTLZ_LEN_CHOICE!();
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen).cast::<u32>();
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET!() + posState;
        bits2BeDec = 3;
        offset = 0;
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        probLen = probSlot + CMPTLZ_LEN_CHOICE2!();
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen).cast::<u32>();
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE!() + CMPTLZ_LEN_CHOICE2!() + posState;
            bits2BeDec = 3;
            offset = (CMPTLZ_LOW_LEN_CLASS!() << 1);
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET!();
            bits2BeDec = 8;
            offset = (CMPTLZ_LOW_LEN_CLASS!() << 1);
        }
    }
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    let mut decSym: u32 = 1;
    c_do!({
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    } while decSym < ((1 as u32) << bits2BeDec));
    decSym -= ((1 as u32) << bits2BeDec);
    decSym += offset;
    if (mkState >= 4) {
        *pbufLimit = bufTryDec;
        return CMPT_OK!();
    }
    probSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decSym);
    decSym = 1;
    c_do!({
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    } while decSym < (1 << CMPTLZ_POS_SLOT_BITS!()));
    decSym -= (1 << CMPTLZ_POS_SLOT_BITS!());
    bits2BeDec = ((decSym >> 1) - 1);
    if (decSym >= CMPTLZ_LOW_POSSLOT!()) {
        if (decSym < CMPTLZ_HIGH_POSSLOT!()) {
            probSlot = CmptLzGetSpecPosProb(probsMatrix)
                + (CmptLzGetBaseDistByPosSlot(decSym) << bits2BeDec);
        } else {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS!();
            c_do!({
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1);
            } while bits2BeDec.prefix_minus_minus() > 0);
            probSlot = CmptLzGetAilgnProb(probsMatrix);
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS!();
        }
        decSym = 1;
        offset = 1;
        c_do!({
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probBit).cast::<u32>();
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
        } while bits2BeDec.prefix_minus_minus() > 0);
    }
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK!();
}

pub fn CmptLzTryDecLitPacket(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut range: u32,
    mut rangeCode: u32,
    mut rangeBound: u32,
    mut bufTryDec: Ptr<u8>,
    mut pbufLimit: Ptr<Ptr<u8>>,
) -> i32 {
    let mut probBit: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut procPos: u32 = decCtx.processedPos;
    let mut litPosMask: u32 =
        ((0x100 as u32) << decCtx.prop.litPos) - ((0x100 as u32) >> decCtx.prop.litCtx);
    let mut dictBufSize: usize = decCtx.dictBufSize;
    let mut dicPos: usize = decCtx.dictPos;
    let mut dict: Ptr<u8> = decCtx.dict;
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    if (decCtx.dictPos >= decCtx.dictBufSize) {
        return CMPT_ERROR_DATA!();
    }
    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0) || (decCtx.checkDicSize != 0) {
        probSlot += 3
            * ((((procPos << 8)
                + dict[(if dicPos == 0 { dictBufSize } else { dicPos }) - 1] as u32)
                & litPosMask)
                << decCtx.prop.litCtx)
    }
    let mut decSym: u32 = 1;
    if (decCtx.state < CMPTLZ_LIT_STATES!()) {
        c_do!({
            probBit = (probSlot + decSym);
            CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        } while decSym < 0x100);
    } else {
        let mut bit: u32 = Default::default();
        let mut matchSym: u32 = dict[dicPos - decCtx.reps[0] as usize
            + if dicPos < decCtx.reps[0] as usize {
                dictBufSize
            } else {
                0
            }] as u32;
        let mut offset: u32 = 0x100;
        c_do!({
            matchSym <<= 1;
            bit = offset;
            offset &= matchSym;
            probBit = (probSlot + (offset + bit + decSym));
            CMPTLZ_MATCH_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit, offset, bit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        } while decSym < 0x100);
    }
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK!();
}

pub fn CmptLzTryDecOnePacket(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut bufTryDec: Ptr<u8>,
    mut pbufLimit: Ptr<Ptr<u8>>,
) -> i32 {
    let mut rangeBound: u32 = 0;
    let mut range: u32 = decCtx.range;
    let mut rangeCode: u32 = decCtx.code;
    let mut mkState: u32 = decCtx.state;
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot1: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot2: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut pbMask: u32 = ((1).cast::<u32>() << decCtx.prop.posBits) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    probSlot1 = (CmptLzGetIsMatchProb(probsMatrix) + posState + mkState);
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot1).cast::<u32>();
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx, range, rangeCode, rangeBound, bufTryDec, pbufLimit);
    }
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    probSlot2 = (CmptLzGetIsRepProb(probsMatrix) + mkState);
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot2).cast::<u32>();
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
        mkState = 0;
    } else {
        if (decCtx.dictPos >= decCtx.dictBufSize) {
            return CMPT_ERROR_DATA!();
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        probSlot = (CmptLzGetIsRepG0Prob(probsMatrix) + mkState);
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = (CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState);
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                *pbufLimit = bufTryDec;
                return CMPT_OK!();
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            }
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = (CmptLzGetIsRepG1Prob(probsMatrix) + mkState);
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                probSlot = (CmptLzGetIsRepG2Prob(probsMatrix) + mkState);
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
                if (rangeCode < rangeBound) {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                }
            }
        }
        probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
        mkState = CMPTLZ_MKSTATE_NUM!();
    }
    return CmptLzTryDecLenAndDist(
        decCtx, mkState, range, rangeCode, rangeBound, probSlot, bufTryDec, pbufLimit,
    );
}

pub fn CmptLzDecCarefulProcess(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut dicPosLimit: usize,
    mut bufLimit: Ptr<u8>,
) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut remainLen: u32;
    let mut bufLimitTmp: Ptr<u8>;
    let mut pSrcIn: Ptr<u8>;
    c_do!({
        bufLimitTmp = bufLimit.cast();
        pSrcIn = decCtx.buf.cast();
        res = CmptLzTryDecOnePacket(decCtx.cast(), pSrcIn.cast(), c_ref!(bufLimitTmp).cast()).cast();
        if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
            break;
        }
        res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimitTmp.cast()).cast();
        if (res != CMPT_OK!()).as_bool() || (decCtx.buf != bufLimitTmp).as_bool() {
            return CMPT_ERROR_DATA!();
        }
        if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()).as_bool() {
            break;
        }
    } while (decCtx.dictPos < dicPosLimit));
    if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() && (decCtx.buf < bufLimit).as_bool() {
        remainLen = (bufLimit - decCtx.buf).cast();
        decCtx.tempBufSize = remainLen.cast();
        c_for!(let mut idx: u32 = 0; idx < remainLen; idx.suffix_plus_plus(); {
            decCtx.tempBuf[idx] = decCtx.buf[idx].cast();
        });
    }
    return CMPT_OK!();
}

pub fn CmptLzDecSinglePacket(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut dicPosLimit: usize,
    mut pSrcIn: Ptr<u8>,
    mut srcInLen: usize,
    mut psrcCostLen: Ptr<usize>,
) -> i32 {
    let mut res: i32;
    let mut lookAheadLen: usize = 0;
    let mut newTempBufSize: u32 = decCtx.tempBufSize.cast();
    let mut oldTmpBuf: Ptr<u8> = (c_ref!(decCtx.tempBuf[0]) + decCtx.tempBufSize).cast();
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX!()).as_bool()
        && (lookAheadLen < srcInLen).as_bool()
    {
        decCtx.tempBuf[newTempBufSize] = pSrcIn[lookAheadLen].cast();
        newTempBufSize += 1;
        lookAheadLen += 1;
    }
    let mut bufLimit: Ptr<u8> = decCtx.tempBuf.cast::<Ptr<u8>>() + newTempBufSize;
    res = CmptLzTryDecOnePacket(
        decCtx.cast(),
        decCtx.tempBuf.cast(),
        c_ref!(bufLimit).cast(),
    )
    .cast();
    if (res == CMPTLZ_DEC_INPUT_EOF!()).as_bool() {
        *psrcCostLen = lookAheadLen.cast();
        decCtx.tempBufSize = newTempBufSize.cast();
        return CMPTLZ_DEC_INPUT_EOF!();
    }
    if (res == CMPT_ERROR_DATA!()).as_bool() {
        return res;
    }
    decCtx.buf = c_ref!(decCtx.tempBuf[0]).cast();
    res = CmptLzDecDirectProcess(decCtx.cast(), dicPosLimit.cast(), bufLimit.cast()).cast();
    if (res != CMPT_OK!()).as_bool()
        || (bufLimit != decCtx.buf).as_bool()
        || (bufLimit <= oldTmpBuf).as_bool()
    {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA!();
    }
    *psrcCostLen = (bufLimit - oldTmpBuf).cast::<usize>();
    decCtx.tempBufSize = 0;
    return res.cast();
}

pub fn CmptLzDecCheckDictSizeUpdate(mut decCtx: Ptr<CmptLzDecCtx>) {
    if (decCtx.checkDicSize == 0 && decCtx.processedPos >= decCtx.prop.dicSize).as_bool() {
        decCtx.checkDicSize = decCtx.prop.dicSize.cast();
    }
}

pub fn CmptLzDecRemWriteInDict(mut decCtx: Ptr<CmptLzDecCtx>, mut dicPosLimit: usize) {
    let mut dictPos: usize = decCtx.dictPos.cast();
    let mut remainDecLen: usize = decCtx.remainLen.cast();
    let mut dictBufSize: usize = decCtx.dictBufSize.cast();
    let mut remainDicLen: usize = dicPosLimit - dictPos;
    if (remainDicLen < remainDecLen).as_bool() {
        remainDecLen = remainDicLen.cast();
    }
    if (remainDecLen == 0).as_bool() {
        return;
    }
    decCtx.processedPos += remainDecLen.cast::<u32>();
    decCtx.remainLen -= remainDecLen.cast::<u32>();
    let mut dict: Ptr<u8> = decCtx.dict.cast();
    let mut rep0: usize = decCtx.reps[0].cast();
    while (remainDecLen != 0).as_bool() {
        remainDecLen -= 1;
        dict[dictPos] = dict[dictPos - rep0 + if dictPos < rep0 { dictBufSize } else { 0 }].cast();
        dictPos += 1;
    }
    decCtx.dictPos = dictPos.cast();
    CmptLzDecCheckDictSizeUpdate(decCtx.cast());
}

pub fn CmptLzDecGetProbsInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut idx: u32 = Default::default();
    let mut numProbs: u32 = CmptLzGetNumProbs(c_ref!(decCtx.prop).cast()).cast();
    let mut decProbs: Ptr<CmptLzDecProb> = decCtx.probs.cast();
    c_for!(idx = 0; idx < numProbs; idx.suffix_plus_plus(); {
        decProbs[idx] = (CMPTLZ_PROB_LG!() >> 1).cast();
    });
    decCtx.state = 0;
}

pub fn CmptLzRangeCodeInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut rangeCode: u32 = (decCtx.tempBuf[1].cast::<u32>()) << 24;
    rangeCode |= (decCtx.tempBuf[2].cast::<u32>()) << 16;
    rangeCode |= (decCtx.tempBuf[3].cast::<u32>()) << 8;
    rangeCode |= decCtx.tempBuf[4].cast::<u32>();
    decCtx.code = rangeCode.cast();
    decCtx.range = 0xFFFFFFFF;
}

pub fn CmptLzDecCtxPrepare(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut pSrcIn: Ptr<u8>,
    mut srcInLen: usize,
    mut finStatus: Ptr<EnCmptLzStatus>,
) -> i32 {
    let mut readCodeLen: usize = (CMPTLZ_RANGE_CODE_SIZE!() - decCtx.tempBufSize).cast::<usize>();
    readCodeLen = if srcInLen < readCodeLen {
        srcInLen
    } else {
        readCodeLen
    };
    while (readCodeLen.suffix_minus_minus() > 0) {
        let tmp0 = decCtx.tempBufSize;
        decCtx.tempBuf[tmp0] = *pSrcIn;
        decCtx.tempBufSize += 1;
        pSrcIn += 1;
    }
    if (decCtx.tempBufSize != 0) && (decCtx.tempBuf[0] != 0) {
        decCtx.tempBufSize = 0;
        *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
        return CMPT_ERROR_DATA!();
    }
    if (decCtx.tempBufSize < CMPTLZ_RANGE_CODE_SIZE!()) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    CmptLzRangeCodeInit(decCtx);
    if (decCtx.remainLen > CMPTLZ_MATCH_MAX_LEN!() + 1) {
        CmptLzDecGetProbsInit(decCtx);
        decCtx.reps[0] = 1;
        decCtx.reps[1] = 1;
        decCtx.reps[2] = 1;
        decCtx.reps[3] = 1;
    }
    decCtx.remainLen = 0;
    return CMPT_OK!();
}

pub fn CmptLzDecDecodeToDic(
    mut decCtx: Ptr<CmptLzDecCtx>,
    mut dicPosLimit: usize,
    mut pSrcIn: Ptr<u8>,
    mut pStrInLen: Ptr<usize>,
    mut finMode: EnCmptLzFinMode,
    mut finStatus: Ptr<EnCmptLzStatus>,
) -> i32 {
    let mut res: i32;
    let mut carefulDecDone: bool = false;
    let mut srcDecLenTmp: usize = Default::default();
    let mut srcDecLen: usize = 0;
    let mut srcInLen: usize = *pStrInLen;
    if (decCtx.remainLen > CMPTLZ_MATCH_MAX_LEN!()) {
        let mut oldTempBufSize: usize = decCtx.tempBufSize.cast::<usize>();
        res = CmptLzDecCtxPrepare(decCtx, pSrcIn, srcInLen, finStatus);
        srcDecLenTmp = (decCtx.tempBufSize.cast::<usize>() - oldTempBufSize).cast();
        if (res != CMPT_OK!()) || (*finStatus == CMPTLZ_STATUS_NEEDS_MORE_INPUT!()) {
            *pStrInLen = srcDecLenTmp;
            return res;
        }
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        decCtx.tempBufSize = 0;
    }
    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()) {
        if (decCtx.code != 0) {
            return CMPT_ERROR_DATA!();
        }
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen != 0) {
        CmptLzDecRemWriteInDict(decCtx, dicPosLimit);
    }
    if (decCtx.tempBufSize != 0) {
        res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, c_ref!(srcDecLenTmp));
        *pStrInLen = srcDecLenTmp;
        if (res == CMPT_ERROR_DATA!()) {
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
            return CMPT_ERROR_DATA!();
        } else if (res == CMPTLZ_DEC_INPUT_EOF!()) {
            *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
            return CMPT_OK!();
        } else {
            srcDecLen += srcDecLenTmp;
            pSrcIn += srcDecLenTmp;
            srcInLen -= srcDecLenTmp;
        }
    }
    while (decCtx.dictPos < dicPosLimit) && (carefulDecDone == false) {
        decCtx.buf = pSrcIn;
        if (srcInLen <= CMPTLZ_REQUIRED_INPUT_MAX!()) {
            res = CmptLzDecCarefulProcess(decCtx, dicPosLimit, (pSrcIn + srcInLen));
            carefulDecDone = true;
        } else {
            res = CmptLzDecDirectProcess(
                decCtx,
                dicPosLimit,
                (pSrcIn + srcInLen - CMPTLZ_REQUIRED_INPUT_MAX!()),
            );
        }
        srcDecLenTmp = (decCtx.buf - pSrcIn).cast::<usize>() + decCtx.tempBufSize.cast::<usize>();
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        if (res == CMPT_ERROR_DATA!()) {
            *pStrInLen = srcDecLen;
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED!();
            return CMPT_ERROR_DATA!();
        }
    }
    *pStrInLen = srcDecLen;
    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()) && (decCtx.code == 0) {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }
    if (decCtx.dictPos < dicPosLimit) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen == 0) && (decCtx.code == 0) {
        *finStatus = CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK!();
        return CMPT_OK!();
    }
    if (finMode == CMPTLZ_FINISH_ANY!()) {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen != 0) {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
        return CMPT_ERROR_DATA!();
    }
    srcDecLenTmp = 0;
    res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, c_ref!(srcDecLenTmp));
    srcDecLen += srcDecLenTmp;
    *pStrInLen = srcDecLen;
    if (res == CMPTLZ_DEC_INPUT_EOF!()) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT!();
        return CMPT_OK!();
    }
    if (decCtx.remainLen == CMPTLZ_MATCH_MAX_LEN!()) && (decCtx.code == 0) {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK!();
        return CMPT_OK!();
    }
    *finStatus = CMPTLZ_STATUS_NOT_FINISHED!();
    return CMPT_ERROR_DATA!();
}

pub fn CmptlzDecompress(
    mut src: Ptr<Void>,
    mut srcSize: usize,
    mut dst: Ptr<Void>,
    mut dstSize: Ptr<usize>,
    mut param: Ptr<CmptlzDecParam>,
) -> i32 {
    if (src == NULL!()).as_bool() || (dst == NULL!()).as_bool() || (dstSize == NULL!()).as_bool() {
        CMPTLZ_LOG!(
            CMPT_ERROR_UNSUPPORTED!(),
            cstr!("The input parameter NULL is incorrect.")
        );
        return CMPT_ERROR_UNSUPPORTED!();
    }
    if (srcSize > 0x7fffffff).as_bool() || (*dstSize > 0x7fffffff).as_bool() {
        CMPTLZ_LOG!(
            CMPT_ERROR_UNSUPPORTED!(),
            cstr!("dstSize:0x{} srcSize:0x{}"),
            *dstSize,
            srcSize
        );
        return CMPT_ERROR_UNSUPPORTED!();
    }
    if (param == NULL!()).as_bool()
        || (param.memHook == NULL!()).as_bool()
        || (param.protData == NULL!()).as_bool()
        || (param.protSize != CMPTLZ_PROPS_SIZE!()).as_bool()
    {
        CMPTLZ_LOG!(
            CMPT_ERROR_UNSUPPORTED!(),
            cstr!("The compress param NULL is incorrect.")
        );
        return CMPT_ERROR_UNSUPPORTED!();
    }
    let mut decIn: CmptLzDecIn = CmptLzDecIn {
        pSrcIn: src,
        strInLen: srcSize,
        strInCostLen: 0,
    };
    let mut decOut: CmptLzDecOut = CmptLzDecOut {
        pDestOut: dst,
        destOutLen: *dstSize,
        destOutFillLen: 0,
    };
    let mut enFinStat: EnCmptLzStatus = CMPTLZ_STATUS_BUT!();
    let mut ret: i32 = CmptLzDecode(
        c_ref!(decIn).cast(),
        c_ref!(decOut).cast(),
        param.protData.cast(),
        CMPTLZ_FINISH_ANY!(),
        c_ref!(enFinStat).cast(),
        param.memHook.cast(),
    )
    .cast();
    *dstSize = decOut.destOutFillLen.cast();
    return ret.cast();
}

pub fn CmptlzEndMarker() {
    return;
}

pub fn CmptlzFlush(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    encCtx.encNeedFinish = true;
    if (encCtx.endMarker != 0).as_bool() {
        CmptlzEndMarker();
    }
    CmptRcFlushData(encCtx.rcCtx.cast());
    return CmptRcFlush64Kb(encCtx.rcCtx.cast()).cast();
}

pub fn CmptPriceCheck(mut encCtx: Ptr<CmptLzEncCtx>) {
    if (encCtx.matchPriceCount >= CMPT_PRICE_COUNT!()).as_bool() {
        CmptPriceGenDistTable(encCtx.cast());
        CmptPriceGenAlignTable(encCtx.cast());
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.matchLenEncoder).cast());
    }
    if (encCtx.repLenPriceCount <= 0).as_bool() {
        encCtx.repLenPriceCount = CMPT_PRICE_COUNT!();
        CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.repLenEncoder).cast());
    }
}

pub fn CmptEncShortOrRep0(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut nowpos32: u32,
    mut lenRes: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    if (lenRes == 1).as_bool() {
        shiftRes = CmptlzEncShortRep(encCtx.cast(), nowpos32.cast()).cast();
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes;
        }
    } else {
        shiftRes = CmptlzEncLongRep(encCtx.cast(), 0, nowpos32.cast(), lenRes.cast()).cast();
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes;
        }
    }
    return CMPT_OK!();
}

pub fn CmptEncodeOneBlock(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx;
    let mut nowpos32: u32 = encCtx.nowpos64.cast();
    let mut startpos: u32 = nowpos32;
    let mut backRes: u32 = Default::default();
    let mut lenRes: u32 = Default::default();
    let mut shiftRes: i32 = CMPT_OK!();
    loop {
        CmptlzDp(encCtx, mf, nowpos32);
        backRes = encCtx.backRes;
        lenRes = encCtx.lenRes;
        c_switch!(backRes;
            CMPTLZ_UINT32_MAX!() => {
                shiftRes = CmptlzEncLit(encCtx, mf, nowpos32);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            0 => {
                shiftRes = CmptEncShortOrRep0(encCtx, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            1 => {
                shiftRes = CmptlzEncLongRep(encCtx, 1, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            2 => {
                shiftRes = CmptlzEncLongRep(encCtx, 2, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            3 => {
                shiftRes = CmptlzEncLongRep(encCtx, 3, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
            _ => {
                shiftRes = CmptlzEncNormalMatch(encCtx, nowpos32, backRes, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
                break;
            },
        );
        nowpos32 += lenRes;
        mf.mfStart += lenRes;
        mf.readAhead -= lenRes;
        if (mf.readAhead == 0) {
            CmptPriceCheck(encCtx);
            if (mf.srcLen <= mf.mfStart as usize) {
                break;
            }
            if (nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE!()) {
                encCtx.nowpos64 += (nowpos32 - startpos) as u64;
                return 0;
            }
        }
    }
    encCtx.nowpos64 += (nowpos32 - startpos) as u64;
    return CmptlzFlush(encCtx);
}

pub fn CmptEncodeAll(mut encCtx: Ptr<CmptLzEncCtx>) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx;
    let mut mf: Ptr<CmptMfCtx> = encCtx.mfCtx;
    if (mf.srcLen == 0) {
        return CmptlzFlush(encCtx);
    }
    if (encCtx.nowpos64 == 0) {
        let mut range: u32;
        let mut bit0Prob: u32;
        let mut newBound: u32;
        range = rc.range;
        let tmp0 = encCtx.state;
        let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[tmp0][0]);
        CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
        let mut shiftRes: i32 = CMPT_OK!();
        CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        rc.range = range;
        let mut curByte: u8 = (*mf.srcStart);
        let mut litProb: Ptr<CmptlzProb> = c_ref!(encCtx.litMarcov.literal[0][0]);
        shiftRes = CmptRcLitProcess(rc, litProb, curByte.cast::<u32>());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        mf.mfStart += 1;
        encCtx.nowpos64 += 1;
        mf.readPos += 1;
        if (mf.srcLen == 1) {
            return CmptlzFlush(encCtx);
        }
    }
    let mut res: i32;
    loop {
        res = CmptEncodeOneBlock(encCtx);
        if (res != 0) || encCtx.encNeedFinish {
            break;
        }
    }
    return res;
}

pub fn CmptlzDpInitShortRep(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut repMatchPrice: u32,
    mut posState: u32,
) {
    let mut shortRepPrice: u32 = (repMatchPrice
        + CmptPriceShortRep(encCtx.cast(), encCtx.state.cast(), posState.cast()))
    .cast();
    if (shortRepPrice < encCtx.opts[1].price).as_bool() {
        encCtx.opts[1].price = shortRepPrice.cast();
        encCtx.opts[1].backPrev = 0;
    }
}

pub fn CmptlzDpInitLongRep(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut repLens: Ptr<u32>,
    mut repMatchPrice: u32,
    mut posState: u32,
) {
    let mut i: u32 = Default::default();
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        let mut repLen: u32 = repLens[i];
        if (repLen < CMPTLZ_MATCH_LEN_MIN!()).as_bool() {
            continue;
        }
        let mut price: u32 = repMatchPrice + CmptPriceLongRep(encCtx.cast(), i.cast(), encCtx.state.cast(), posState.cast());
        c_do!({
            let mut curAndLenPrice: u32 = price + CmptPriceLen(c_ref!(encCtx.repLenEncoder).cast(), repLen.cast(), posState.cast());
            if (curAndLenPrice < encCtx.opts[repLen].price).as_bool() {
                encCtx.opts[repLen].price = curAndLenPrice.cast();
                encCtx.opts[repLen].posPrev = 0;
                encCtx.opts[repLen].backPrev = i.cast();
            }
            repLen -= 1;
        } while repLen >= CMPTLZ_MATCH_LEN_MIN!());
    });
}

pub fn CmptlzDpInitMatch(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut matchesCount: u32,
    mut normalMatchPrice: u32,
    mut posState: u32,
    mut len: u32,
) {
    let mut i: u32 = 0;
    while (len > encCtx.matches[i].len).as_bool() {
        i += 1;
    }
    loop {
        let mut dist: u32 = encCtx.matches[i].dist.cast();
        let mut curAndLenPrice: u32 = (normalMatchPrice
            + CmptPriceDistWithLen(encCtx.cast(), dist.cast(), len.cast(), posState.cast()))
        .cast();
        if (curAndLenPrice < encCtx.opts[len].price).as_bool() {
            encCtx.opts[len].price = curAndLenPrice.cast();
            encCtx.opts[len].posPrev = 0;
            encCtx.opts[len].backPrev = (dist + CMPTLZ_NUM_REPS!()).cast();
        }
        if (len == encCtx.matches[i].len).as_bool() {
            i += 1;
            if (i == matchesCount).as_bool() {
                break;
            }
        }
        len += 1;
    }
}

pub fn CmptlzDpInit(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut mf: Ptr<CmptMfCtx>,
    mut position: u32,
) -> u32 {
    let niceLen: u32 = mf.niceLen;
    let mut lenMain: u32;
    let mut matchesCount: u32 = 0;
    if (mf.readAhead == 0) {
        lenMain = CmptlzMatchFinder(mf, c_ref!(matchesCount), encCtx.matches.cast());
    } else {
        lenMain = encCtx.longestMatchLen;
        matchesCount = encCtx.matchesCount;
    }
    let buf: Ptr<u8> = CmptMfGetPtr(mf) - 1;
    let bufAvail: u32 = CMPTLZ_FIND_MIN!(CmptMfAvail(mf) + 1, CMPT_MF_LONGEST_MATCH!());
    if (bufAvail < CMPTLZ_MATCH_LEN_MIN!()) {
        encCtx.backRes = CMPTLZ_UINT32_MAX!();
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    let mut repLens: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    let mut repMaxIndex: u32 = 0;
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        let bufBack: Ptr<u8> = buf - encCtx.reps[i] - 1;
        if NOT_EQUAL_2_BYTES!(buf, bufBack) {
            repLens[i] = 0;
            continue;
        }
        repLens[i] = CmptMemCmpLenSafe(buf, bufBack, CMPTLZ_MATCH_LEN_MIN!(), bufAvail);
        if (repLens[i] > repLens[repMaxIndex]) {
            repMaxIndex = i;
        }
    });
    if (repLens[repMaxIndex] >= niceLen) {
        encCtx.backRes = repMaxIndex;
        encCtx.lenRes = repLens[repMaxIndex];
        CmptlzMatchSkiper(mf, (repLens[repMaxIndex] - 1));
        return CMPTLZ_UINT32_MAX!();
    }
    if (lenMain >= niceLen) {
        encCtx.backRes = (encCtx.matches[matchesCount - 1].dist + CMPTLZ_NUM_REPS!());
        encCtx.lenRes = lenMain;
        CmptlzMatchSkiper(mf, (lenMain - 1));
        return CMPTLZ_UINT32_MAX!();
    }
    let mut currentByte: u8 = *buf;
    let mut matchByte: u8 = *(buf - encCtx.reps[0] - 1);
    let lenEnd: u32 = CMPTLZ_FIND_MAX!(lenMain, repLens[repMaxIndex]);
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN!()) && (currentByte != matchByte) {
        encCtx.backRes = CMPTLZ_UINT32_MAX!();
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    encCtx.opts[0].state = encCtx.state;
    let posState: u32 = position & encCtx.posMask;
    encCtx.litMarcov.pos = position;
    encCtx.litMarcov.prevByte = *(buf - 1) as u32;
    let isLiteralState: bool = (encCtx.state < 7);
    let isMatchMode: bool = !isLiteralState;
    encCtx.opts[1].price = (CmptPriceBit0(encCtx, encCtx.isMatch[encCtx.state][posState])
        + CmptPriceLiteral(encCtx, isMatchMode, matchByte.cast(), currentByte.cast()));
    encCtx.opts[1].backPrev = CMPTLZ_UINT32_MAX!();
    let tmp0 = encCtx.state;
    let matchPrice: u32 = CmptPriceBit1(encCtx, encCtx.isMatch[tmp0][posState]);
    let repMatchPrice: u32 = (matchPrice + CmptPriceBit1(encCtx, encCtx.isRep[encCtx.state]));
    if (matchByte == currentByte) {
        CmptlzDpInitShortRep(encCtx, repMatchPrice, posState);
    }
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN!()) {
        encCtx.backRes = encCtx.opts[1].backPrev;
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    encCtx.opts[1].posPrev = 0;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.opts[0].backs[i] = encCtx.reps[i];
    });
    let mut len: u32 = lenEnd;
    c_do!({
        encCtx.opts[len].price = CMPT_INFINITY_PRICE!();
        len.suffix_minus_minus();
    } while len >= CMPTLZ_MATCH_LEN_MIN!());
    CmptlzDpInitLongRep(encCtx, repLens.cast(), repMatchPrice, posState);
    let normalMatchPrice: u32 = (matchPrice + CmptPriceBit0(encCtx, encCtx.isRep[encCtx.state]));
    len = if repLens[0] > CMPTLZ_MATCH_LEN_MIN!() {
        repLens[0] + 1
    } else {
        CMPTLZ_MATCH_LEN_MIN!()
    };
    if (len <= lenMain) {
        CmptlzDpInitMatch(encCtx, matchesCount, normalMatchPrice, posState, len);
    }
    return lenEnd;
}

pub fn CmptlzDpPre(mut encCtx: Ptr<CmptLzEncCtx>, mut mainReps: Ptr<u32>, mut cur: u32) {
    let mut posPointer: u32 = encCtx.opts[cur].posPrev;
    let mut state: CmptlzState = encCtx.opts[posPointer].state;
    if (posPointer == cur - 1) {
        if (encCtx.opts[cur].backPrev == 0) {
            state = if state < 7 {
                LIT_SHORTREP!()
            } else {
                NOTLIT_REP!()
            };
        } else {
            state = if state <= SHORTREP_LIT_LIT!() {
                LIT_LIT!()
            } else if state <= LIT_SHORTREP!() {
                LIT_SHORTREP!()
            } else {
                LIT_LONGREP!()
            };
        }
    } else {
        let mut backPointer: u32;
        backPointer = encCtx.opts[cur].backPrev;
        if (backPointer < CMPTLZ_NUM_REPS!()) {
            state = if state < 7 {
                LIT_LONGREP!()
            } else {
                NOTLIT_REP!()
            };
        } else {
            state = if state < 7 {
                LIT_MATCH!()
            } else {
                NOTLIT_MATCH!()
            };
        }
        let mut i: u32;
        if (backPointer < CMPTLZ_NUM_REPS!()) {
            mainReps[0] = encCtx.opts[posPointer].backs[backPointer];
            c_for!(i = 1; i <= backPointer; i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i - 1];
            });
            c_for!(; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i];
            });
        } else {
            mainReps[0] = (backPointer - CMPTLZ_NUM_REPS!());
            c_for!(i = 1; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i - 1];
            });
        }
    }
    encCtx.opts[cur].state = state;
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.opts[cur].backs[i] = mainReps[i];
    });
}

pub fn CmptlzDpTryCurAndLit(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut curPrice: u32,
    mut curState: CmptlzState,
    mut posState: u32,
    mut cur: u32,
    mut latestMatchByte: u8,
    mut curByte: u8,
) {
    let mut isLiteralState: bool = (curState < 7);
    let mut isMatchMode: bool = !isLiteralState;
    let mut curAndLitPrice: u32 = (curPrice
        + CmptPriceBit0(encCtx, encCtx.isMatch[curState][posState])
        + CmptPriceLiteral(
            encCtx,
            isMatchMode,
            latestMatchByte.cast::<u32>(),
            curByte.cast::<u32>(),
        ));
    if (curAndLitPrice < encCtx.opts[cur + 1].price) {
        encCtx.opts[cur + 1].price = curAndLitPrice;
        encCtx.opts[cur + 1].posPrev = cur;
        encCtx.opts[cur + 1].backPrev = CMPTLZ_UINT32_MAX!();
    }
}

pub fn CmptlzDpTryCurAndShort(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut repMatchPrice: u32,
    mut cur: u32,
    mut curState: CmptlzState,
    mut posState: u32,
) {
    let mut shortRepPrice: u32 =
        (repMatchPrice + CmptPriceShortRep(encCtx.cast(), curState.cast(), posState.cast())).cast();
    if (shortRepPrice < encCtx.opts[cur + 1].price).as_bool() {
        encCtx.opts[cur + 1].price = shortRepPrice.cast();
        encCtx.opts[cur + 1].posPrev = cur.cast();
        encCtx.opts[cur + 1].backPrev = 0;
    }
}

pub fn CmptlzDpTryCurAndLong(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut prefixPrice: u32,
    mut cur: u32,
    mut mainRepIndex: u32,
    mut lenEqual: u32,
    mut posState: u32,
) {
    c_do!({
        let mut curLongRepPrice: u32 = (prefixPrice + CmptPriceLen(c_ref!(encCtx.repLenEncoder).cast(), lenEqual.cast(), posState.cast())).cast();
        if (curLongRepPrice < encCtx.opts[cur + lenEqual].price).as_bool() {
            encCtx.opts[cur + lenEqual].price = curLongRepPrice.cast();
            encCtx.opts[cur + lenEqual].posPrev = cur.cast();
            encCtx.opts[cur + lenEqual].backPrev = mainRepIndex.cast();
        }
    } while lenEqual.prefix_minus_minus() >= CMPTLZ_MATCH_LEN_MIN!());
}

pub fn CmptlzDpTryCurAndMatch(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut startLen: u32,
    mut matchCount: u32,
    mut normalmatch_prefixPrice: u32,
    mut cur: u32,
    mut posState: u32,
) {
    let mut i: u32 = 0;
    while (startLen > encCtx.matches[i].len).as_bool() {
        i += 1;
    }
    let mut lenTest: u32;
    c_for!(lenTest = startLen; ; lenTest += 1; {
        let mut curBack: u32 = encCtx.matches[i].dist.cast();
        let mut cur_normalmatchPrice: u32 = (normalmatch_prefixPrice + CmptPriceDistWithLen(encCtx.cast(), curBack.cast(), lenTest.cast(), posState.cast())).cast();
        if (cur_normalmatchPrice < encCtx.opts[cur + lenTest].price).as_bool() {
            encCtx.opts[cur + lenTest].price = cur_normalmatchPrice.cast();
            encCtx.opts[cur + lenTest].posPrev = cur.cast();
            encCtx.opts[cur + lenTest].backPrev = (curBack + CMPTLZ_NUM_REPS!()).cast();
        }
        if (lenTest == encCtx.matches[i].len).as_bool() {
            if (i.prefix_plus_plus() == matchCount).as_bool() {
                break;
            }
        }
    });
}

pub fn CmptlzDpProcess(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut mf: Ptr<CmptMfCtx>,
    mut mainReps: Ptr<u32>,
    mut lenEnd: u32,
    mut position: u32,
    mut cur: u32,
) -> u32 {
    let mut curState: CmptlzState = encCtx.opts[cur].state;
    let mut bufAvailFull: u32 = CMPTLZ_FIND_MIN!(CmptMfAvail(mf) + 1, CMPT_DP_OPTMAX!() - 1 - cur);
    let mut buf: Ptr<u8> = CmptMfGetPtr(mf) - 1;
    let mut niceLen: u32 = mf.niceLen;
    let mut curPrice: u32 = encCtx.opts[cur].price;
    let mut curByte: u8 = *buf;
    let mut latestMatchByte: u8 = *(buf - mainReps[0] - 1);
    let mut posState: u32 = position & encCtx.posMask;
    encCtx.litMarcov.pos = position;
    encCtx.litMarcov.prevByte = (*(buf - 1)).cast::<u32>();
    CmptlzDpTryCurAndLit(
        encCtx,
        curPrice,
        curState,
        posState,
        cur,
        latestMatchByte,
        curByte,
    );
    let mut matchPrice: u32 = curPrice + CmptPriceBit1(encCtx, encCtx.isMatch[curState][posState]);
    let mut repMatchPrice: u32 = matchPrice + CmptPriceBit1(encCtx, encCtx.isRep[curState]);
    let tmp0 = cur + 1;
    if (curByte == latestMatchByte) && !(encCtx.opts[tmp0].backPrev == 0) {
        CmptlzDpTryCurAndShort(encCtx, repMatchPrice, cur, curState, posState);
    }
    if (bufAvailFull < CMPTLZ_MATCH_LEN_MIN!()) {
        return lenEnd;
    }
    let mut bufAvail: u32 = CMPTLZ_FIND_MIN!(bufAvailFull, niceLen);
    let mut startLen: u32 = CMPTLZ_MATCH_LEN_MIN!();
    let mut mainRepIndex: u32;
    c_for!(mainRepIndex = 0; mainRepIndex < CMPTLZ_NUM_REPS!(); mainRepIndex.suffix_plus_plus(); {
        let mut bufRepBack: Ptr<u8> = (buf - mainReps[mainRepIndex] - 1);
        if NOT_EQUAL_2_BYTES!(buf, bufRepBack) {
            continue;
        }
        let mut lenEqual: u32;
        lenEqual = CmptMemCmpLenSafe(buf, bufRepBack, CMPTLZ_MATCH_LEN_MIN!(), bufAvail);
        while (lenEnd < cur + lenEqual) {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!();
        }
        let mut lenEqualMem: u32 = lenEqual;
        let mut prefixPrice: u32 = repMatchPrice + CmptPriceLongRep(encCtx, mainRepIndex, curState, posState);
        CmptlzDpTryCurAndLong(encCtx, prefixPrice, cur, mainRepIndex, lenEqual, posState);
        lenEqual = lenEqualMem;
        if (mainRepIndex == 0) {
            startLen = lenEqual + 1;
        }
    });
    let mut newLongestLen: u32 = encCtx.longestMatchLen;
    let mut matchCount: u32 = encCtx.matchesCount;
    if (newLongestLen > bufAvail) {
        newLongestLen = bufAvail;
        matchCount = 0;
        while (newLongestLen > encCtx.matches[matchCount].len) {
            matchCount.suffix_plus_plus();
        }
        encCtx.matches[matchCount].len = newLongestLen;
        matchCount.suffix_plus_plus();
    }
    if (newLongestLen >= startLen) {
        let mut normalmatch_prefixPrice: u32 =
            matchPrice + CmptPriceBit0(encCtx, encCtx.isRep[curState]);
        while (lenEnd < cur + newLongestLen) {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!();
        }
        CmptlzDpTryCurAndMatch(
            encCtx,
            startLen,
            matchCount,
            normalmatch_prefixPrice,
            cur,
            posState,
        );
    }
    return lenEnd;
}

pub fn CmptlzDpReverse(mut encCtx: Ptr<CmptLzEncCtx>, mut cur: u32) {
    encCtx.optEndIndex = cur;
    let mut posTmp: u32 = encCtx.opts[cur].posPrev;
    let mut backTmp: u32 = encCtx.opts[cur].backPrev;
    let mut posPrev: u32;
    let mut backCurPacket: u32;
    c_do!({
        posPrev = posTmp;
        backCurPacket = backTmp;
        backTmp = encCtx.opts[posPrev].backPrev;
        posTmp = encCtx.opts[posPrev].posPrev;
        encCtx.opts[posPrev].backPrev = backCurPacket;
        encCtx.opts[posPrev].posPrev = cur;
        cur = posPrev;
    } while cur != 0);
    encCtx.lenRes = encCtx.opts[0].posPrev;
    encCtx.backRes = encCtx.opts[0].backPrev;
    encCtx.optsCurIndex = encCtx.opts[0].posPrev;
}

pub fn CmptlzDp(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut position: u32) {
    let mut curIndex: u32 = encCtx.optsCurIndex.cast();
    let mut endIndex: u32 = encCtx.optEndIndex.cast();
    if (endIndex != curIndex).as_bool() {
        encCtx.lenRes = (encCtx.opts[curIndex].posPrev - curIndex).cast();
        encCtx.backRes = encCtx.opts[curIndex].backPrev.cast();
        encCtx.optsCurIndex = encCtx.opts[curIndex].posPrev.cast();
        return;
    }
    let mut lenEnd: u32 = CmptlzDpInit(encCtx.cast(), mf.cast(), position.cast()).cast();
    if (lenEnd == CMPTLZ_UINT32_MAX!()).as_bool() {
        return;
    }
    let mut mainReps: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    c_memcpy_s!(
        mainReps,
        c_sizeofval!(mainReps),
        encCtx.reps,
        c_sizeofval!(encCtx.reps)
    )
    .cast::<Void>();
    let mut cur: u32 = Default::default();
    c_for!(cur = 1; cur < lenEnd; cur.suffix_plus_plus(); {
        encCtx.longestMatchLen = CmptlzMatchFinder(mf.cast(), c_ref!(encCtx.matchesCount).cast(), encCtx.matches.cast()).cast();
        if (encCtx.longestMatchLen >= mf.niceLen).as_bool() {
            break;
        }
        CmptlzDpPre(encCtx.cast(), mainReps.cast(), cur.cast());
        lenEnd = CmptlzDpProcess(encCtx.cast(), mf.cast(), mainReps.cast(), lenEnd.cast(), (position + cur).cast(), cur.cast()).cast();
    });
    CmptlzDpReverse(encCtx.cast(), cur.cast());
    return;
}

pub fn CmptHeadWrite(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut protData: Ptr<u8>,
    mut propsSize: Ptr<usize>,
) -> i32 {
    if (protData == NULL!()).as_bool() {
        CMPTLZ_LOG!(CMPT_ERROR_DATA!(), cstr!("protData is NULL"));
        return CMPT_ENC_ERROR_HEAD!();
    }
    if (*propsSize < CMPTLZ_PROPS_SIZE!()).as_bool() {
        CMPTLZ_LOG!(
            CMPT_ERROR_DATA!(),
            cstr!("propsSize need 5 bytes, get {}"),
            *propsSize
        );
        return CMPT_ENC_ERROR_HEAD!();
    }
    CmptlzWriteLE32Bit((protData + 1).cast(), encCtx.dicSize.cast());
    protData[0] = ((encCtx.posBits * CMPTLZ_POS_STATE_MAX!() + encCtx.litPos)
        * CMPTLZ_LIT_CTX_MAX!()
        + encCtx.litCtx)
        .cast();
    *propsSize = CMPTLZ_PROPS_SIZE!();
    return 0;
}

pub fn CmptlzParamNormalize(mut props: Ptr<CmptlzEncParam>) {
    let mut level: i32 = props.level.cast();
    if (level < 0 || level > 9).as_bool() {
        level = 5;
    }
    props.level = level.cast();
    if (props.dictSize < CMPTLZ_MIN_DICTSIZE!() || props.dictSize > CMPTLZ_MAX_DICTSIZE!())
        .as_bool()
    {
        CMPTLZ_SET_DICTSIZE_BY_LEVEL!(level, props.dictSize);
    }
    if (props.fastBytes < 5 || props.fastBytes > CMPT_MF_LONGEST_MATCH!()).as_bool() {
        CMPTLZ_SET_FB_BY_LEVEL!(level, props.fastBytes);
    }
    if (props.litCtx < 0 || props.litCtx > CMPTLZ_LC_MAX!()).as_bool() {
        props.litCtx = 3;
    }
    if (props.litPos < 0 || props.litPos > CMPTLZ_LP_MAX!()).as_bool() {
        props.litPos = 0;
    }
    if (props.posBits < 0 || props.posBits > CMPTLZ_PB_MAX!()).as_bool() {
        props.posBits = 2;
    }
    props.numThreads = 1;
}

pub fn CmptlzSetParam(mut encCtx: Ptr<CmptLzEncCtx>, mut props: Ptr<CmptlzEncParam>) {
    let mut param: CmptlzEncParam = *props;
    CmptlzParamNormalize(c_ref!(param).cast());
    encCtx.dicSize = param.dictSize.cast();
    encCtx.numFastBytes = param.fastBytes.cast();
    encCtx.litCtx = param.litCtx.cast();
    encCtx.litPos = param.litPos.cast();
    encCtx.posBits = param.posBits.cast();
    let mut i: u32 = 7;
    while i < 32 {
        if (encCtx.dicSize <= (1 << i).cast::<u32>()).as_bool() {
            break;
        }
        i += 1;
    }
    encCtx.distTableSize = (i * 2).cast();
}

pub fn CmptlzPriceInit(mut encCtx: Ptr<CmptLzEncCtx>) {
    CmptPriceGenRootTable(encCtx.cast());
    CmptPriceGenDistTable(encCtx.cast());
    CmptPriceGenAlignTable(encCtx.cast());
}

pub fn CmptlzEncPrepare(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();
    encCtx.encNeedFinish = false;
    encCtx.cmptlzResponse = 0;
    encCtx.nowpos64 = 0;
    encCtx.state = 0;
    encCtx.pbMask = (1 << encCtx.posBits) - 1;
    encCtx.lpMask = (0x100 << encCtx.litPos) - (0x100 >> encCtx.litCtx);
    encCtx.posMask = (1 << encCtx.posBits) - 1;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.reps[i] = 0;
    });
    encCtx.optsCurIndex = 0;
    encCtx.optEndIndex = 0;
    c_for!(i = 0; i < CMPT_DP_OPTMAX!(); i.suffix_plus_plus(); {
        encCtx.opts[i].price = CMPT_INFINITY_PRICE!();
    });
    c_for!(i = 0; i < CMPTLZ_NUM_STATES!(); i.suffix_plus_plus(); {
        c_for!(j = 0; j < CMPTLZ_NUM_PB_STATES_MAX!(); j.suffix_plus_plus(); {
            encCtx.isMatch[i][j] = CMPTLZ_PROB_INIT!();
            encCtx.isRep0Long[i][j] = CMPTLZ_PROB_INIT!();
        });
        encCtx.isRep[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG0[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG1[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG2[i] = CMPTLZ_PROB_INIT!();
    });
    c_for!(i = 0; i < CMPTLZ_DIST_STATE_TOTAL!(); i.suffix_plus_plus(); {
        c_for!(j = 0; j < (1 << CMPTLZ_DIST_SLOT_BITS!()); j.suffix_plus_plus(); {
            encCtx.probDistSlot[i][j] = CMPTLZ_PROB_INIT!();
        });
    });
    c_for!(i = 0; i < CMPT_DIST_LIMIT_2!(); i.suffix_plus_plus(); {
        encCtx.probDistSpecial[i] = CMPTLZ_PROB_INIT!();
    });
    c_for!(i = 0; i < (1 << CMPTLZ_ALIGN_BITS!()); i.suffix_plus_plus(); {
        encCtx.probAlign[i] = CMPTLZ_PROB_INIT!();
    });
    encCtx.litMarcov.lcBits = encCtx.litCtx.cast::<u32>();
    encCtx.litMarcov.posMask = (1 << encCtx.litPos) - 1;
    c_for!(i = 0; i < (1 << CMPTLZ_LCLP_MAX!()); i.suffix_plus_plus(); {
        c_for!(j = 0; j < CMPTLZ_LIT_MAX_SIZE!(); j.suffix_plus_plus(); {
            encCtx.litMarcov.literal[i][j] = CMPTLZ_PROB_INIT!();
        });
    });
    c_for!(i = 0; i < (1 << CMPT_LEN_HIGH_BITS!()); i.suffix_plus_plus(); {
        encCtx.matchLenEncoder.high[i] = CMPTLZ_PROB_INIT!();
        encCtx.repLenEncoder.high[i] = CMPTLZ_PROB_INIT!();
        encCtx.matchLenEncoder.low[i] = CMPTLZ_PROB_INIT!();
        encCtx.repLenEncoder.low[i] = CMPTLZ_PROB_INIT!();
    });
    CmptlzPriceInit(encCtx);
    encCtx.repLenEncoder.tableSize = encCtx.numFastBytes - 1;
    encCtx.matchLenEncoder.tableSize = encCtx.numFastBytes - 1;
    CmptPriceGenLenTable(encCtx, c_ref!(encCtx.matchLenEncoder));
    CmptPriceGenLenTable(encCtx, c_ref!(encCtx.repLenEncoder));
}

pub fn CmptInitCctx(mut alloc: Ptr<CmptLzMemHook>, mut writeEndMark: i32) -> Ptr<Void> {
    let mut handle: Ptr<Void> = (alloc.CmptLzAlloc)(
        CMPTLZ_ENC_CCTX_HANDLE!(),
        c_sizeof!(CmptLzEncCtx).cast::<usize>(),
    );
    if (handle == NULL!()) {
        return NULL!();
    }
    c_memset_s!(handle, c_sizeof!(CmptLzEncCtx), 0, c_sizeof!(CmptLzEncCtx));
    let mut encCtx: Ptr<CmptLzEncCtx> = handle.cast::<Ptr<CmptLzEncCtx>>();
    encCtx.endMarker = writeEndMark;
    encCtx.rcCtx = NULL!();
    encCtx.mfCtx = NULL!();
    return encCtx.cast::<Ptr<Void>>();
}

pub fn CmptMemCmpByOneByte(
    mut buf1: Ptr<u8>,
    mut buf2: Ptr<u8>,
    mut len: u32,
    mut limit: u32,
) -> u32 {
    let mut lenIn: u32 = len.cast();
    while (lenIn < limit).as_bool() && (buf1[lenIn] == buf2[lenIn]).as_bool() {
        lenIn += 1;
    }
    return lenIn.cast();
}

pub fn CmptMemCmpLenSafe(
    mut buf1: Ptr<u8>,
    mut buf2: Ptr<u8>,
    mut len: u32,
    mut limit: u32,
) -> u32 {
    return CmptMemCmpByOneByte(buf1.cast(), buf2.cast(), len.cast(), limit.cast()).cast();
}

pub fn CmptMfAvail(mut mf: Ptr<CmptMfCtx>) -> u32 {
    return (mf.srcLen.cast::<u32>() - mf.readPos).cast();
}

pub fn CmptMfGetPtr(mut mf: Ptr<CmptMfCtx>) -> Ptr<u8> {
    return (mf.srcStart + mf.readPos).cast();
}

pub fn PosSlotHelper(mut n: u32) -> u32 {
    let mut i: u32 = 31;
    if (n & 0xFFFF0000 == 0).as_bool() {
        n <<= 16;
        i = 15;
    }
    if (n & 0xFF000000 == 0).as_bool() {
        n <<= 8;
        i -= 8;
    }
    if (n & 0xF0000000 == 0).as_bool() {
        n <<= 4;
        i -= 4;
    }
    if (n & 0xC0000000 == 0).as_bool() {
        n <<= 2;
        i -= 2;
    }
    if (n & 0x80000000 == 0).as_bool() {
        i -= 1;
    }
    return i.cast();
}

pub fn PosSloter(mut dist: u32) -> u32 {
    if (dist <= 4).as_bool() {
        return dist.cast();
    }
    let mut helper: u32 = PosSlotHelper(dist.cast()).cast();
    return (helper + helper + ((dist >> (helper - 1)) & 1)).cast();
}

pub fn CmptlzMfGenHashTable(mut mf: Ptr<CmptMfCtx>) {
    let mut hashRootTable: Ptr<u32> = mf.hashRootTable.cast();
    let mut poly32: u32 = 0xEDB88320;
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();
    c_for!(i = 0; i < CMPT_MF_HASH_TABLE_SIZE!(); i.suffix_plus_plus(); {
        let mut value: u32 = i.cast();
        c_for!(j = 0; j < 8; j.suffix_plus_plus(); {
            if (value & 1).as_bool() {
                value = (value >> 1) ^ poly32;
            } else {
                value >>= 1;
            }
        });
        hashRootTable[i] = value.cast();
    });
    return;
}

pub fn CmptMfPrepare(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut src: Ptr<u8>,
    mut srcLen: usize,
    mut alloc: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut mf: Ptr<CmptMfCtx> =
        (alloc.CmptLzAlloc)(CMPTLZ_MF_CCTX_HANDLE!(), c_sizeof!(CmptMfCtx).cast())
            .cast::<Ptr<CmptMfCtx>>();
    if (mf == NULL!()) {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    c_memset_s!(mf, c_sizeof!(CmptMfCtx), 0, c_sizeof!(CmptMfCtx)).cast::<Void>();
    encCtx.mfCtx = mf;
    mf.cycleSize = encCtx.dicSize + 1;
    let mut hashMask: u32 = encCtx.dicSize - 1;
    CMPT_HASH_MASK_CALC!(hashMask);
    mf.hashMask = hashMask;
    hashMask += 1;
    hashMask += CMPTLZ_HASH_2_SIZE!();
    hashMask += CMPTLZ_HASH_3_SIZE!();
    mf.hashCount = hashMask;
    mf.sonsCount = mf.cycleSize * 2;
    mf.hash = NULL!();
    mf.son = NULL!();
    mf.hash = (alloc.CmptLzAlloc)(
        CMPTLZ_MF_HASH_HANDLE!(),
        (mf.hashCount as u32 * c_sizeof!(u32)).cast(),
    )
    .cast();
    c_memset_s!(
        mf.hash,
        mf.hashCount as u32 * c_sizeof!(u32),
        0,
        mf.hashCount as u32 * c_sizeof!(u32)
    );
    if (mf.hash == NULL!()) {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    mf.son = (alloc.CmptLzAlloc)(
        CMPTLZ_MF_SON_HANDLE!(),
        (mf.sonsCount as u32 * c_sizeof!(u32)).cast(),
    )
    .cast();
    c_memset_s!(
        mf.son,
        mf.sonsCount as u32 * c_sizeof!(u32),
        0,
        mf.sonsCount as u32 * c_sizeof!(u32)
    );
    if (mf.son == NULL!()) {
        return CMPT_ENC_MF_INIT_FAIL!();
    }
    CmptlzMfGenHashTable(mf);
    mf.srcStart = src;
    mf.srcLen = srcLen;
    mf.offset = mf.cycleSize;
    mf.niceLen = encCtx.numFastBytes;
    mf.depth = CMPT_MF_BASE_DEPTH!() + mf.niceLen / 2;
    return 0;
}

pub fn CmptMfMovePos(mut mf: Ptr<CmptMfCtx>) {
    let mut subvalue: u32 = (CMPTLZ_UINT32_MAX!() - mf.cycleSize).cast();
    let mut i: u32 = Default::default();
    c_for!(i = 0; i < mf.hashCount; i.suffix_plus_plus(); {
        if (mf.hash[i] <= subvalue).as_bool() {
            mf.hash[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.hash[i] -= subvalue;
        }
    });
    c_for!(i = 0; i < mf.sonsCount; i.prefix_plus_plus(); {
        if (mf.son[i] <= subvalue).as_bool() {
            mf.son[i] = CMPT_EMPTY_HASH_VALUE!();
        } else {
            mf.son[i] -= subvalue;
        }
    });
    mf.offset -= subvalue;
}

pub fn CmptBtFind(
    mut mf: Ptr<CmptMfCtx>,
    mut curMatch: u32,
    mut matches: Ptr<CmptlzMatchPair>,
    mut longestLen: u32,
) -> Ptr<CmptlzMatchPair> {
    let mut depth: u32 = mf.depth;
    let mut son: Ptr<u32> = mf.son;
    let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
    let mut niceLen: u32 = mf.niceLen;
    let mut cyclePos: u32 = mf.cyclePos;
    let mut cycleSize: u32 = mf.cycleSize;
    let mut pos: u32 = mf.readPos + mf.offset;
    let mut ptr0: Ptr<u32> = son + (cyclePos << 1) + 1;
    let mut ptr1: Ptr<u32> = son + (cyclePos << 1);
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let mut delta: u32 = pos - curMatch;
        if (depth.suffix_minus_minus() == 0).as_bool() || (delta >= cycleSize).as_bool() {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return matches;
        }
        let mut pair: Ptr<u32> =
            son + ((cyclePos - delta + if delta > cyclePos { cycleSize } else { 0 }) << 1);
        let mut pb: Ptr<u8> = (cur - delta).cast();
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if (pb[len] == cur[len]).as_bool() {
            len = CmptMemCmpLenSafe(pb.cast(), cur.cast(), len + 1, niceLen.cast()).cast();
            if (longestLen < len).as_bool() {
                longestLen = len;
                matches.len = len;
                matches.dist = delta - 1;
                matches = matches + 1;
                if (len == niceLen).as_bool() {
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return matches;
                }
            }
        }
        if (pb[len] < cur[len]).as_bool() {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}

pub fn CmptBtSkip(
    mut mf: Ptr<CmptMfCtx>,
    mut lenLimit: u32,
    mut pos: u32,
    mut cur: Ptr<u8>,
    mut curMatch: u32,
) {
    let mut depth: u32 = mf.depth.cast();
    let mut son: Ptr<u32> = mf.son.cast();
    let mut cyclePos: u32 = mf.cyclePos.cast();
    let mut cycleSize: u32 = mf.cycleSize.cast();
    let mut ptr0: Ptr<u32> = (son + (cyclePos << 1) + 1).cast();
    let mut ptr1: Ptr<u32> = (son + (cyclePos << 1)).cast();
    let mut len0: u32 = 0;
    let mut len1: u32 = 0;
    loop {
        let mut delta: u32 = (pos - curMatch).cast();
        if (depth.suffix_minus_minus() == 0 || delta >= cycleSize).as_bool() {
            *ptr0 = CMPT_EMPTY_HASH_VALUE!();
            *ptr1 = CMPT_EMPTY_HASH_VALUE!();
            return;
        }
        let mut pair: Ptr<u32> =
            (son + ((cyclePos - delta + if delta > cyclePos { cycleSize } else { 0 }) << 1)).cast();
        let mut pb: Ptr<u8> = (cur - delta).cast();
        let mut len: u32 = CMPTLZ_FIND_MIN!(len0, len1);
        if (pb[len] == cur[len]).as_bool() {
            len =
                CmptMemCmpLenSafe(pb.cast(), cur.cast(), (len + 1).cast(), lenLimit.cast()).cast();
            if (len == lenLimit).as_bool() {
                *ptr1 = pair[0].cast();
                *ptr0 = pair[1].cast();
                return;
            }
        }
        if (pb[len] < cur[len]).as_bool() {
            CMPT_MF_LEFT_SON_UPDATE!(ptr1, pair, curMatch, len1, len);
        } else {
            CMPT_MF_RIGHT_SON_UPDATE!(ptr0, pair, curMatch, len0, len);
        }
    }
}

pub fn CmptlzBt4Finder(mut mf: Ptr<CmptMfCtx>, mut matches: Ptr<CmptlzMatchPair>) -> u32 {
    let niceLen: u32 = mf.niceLen;
    let cur: Ptr<u8> = (mf.srcStart + mf.readPos).cast::<Ptr<u8>>();
    let pos: u32 = (mf.readPos + mf.offset);
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut longestLen: u32 = 1;
    let mut matchesCount: u32 = 0;
    CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);
    let mut delta2: u32 = (pos - mf.hash[hash2Value]);
    let mut delta3: u32 = (pos - mf.hash[CMPTLZ_FIX_3_HASH!() + hash3Value]);
    let mut curMatch: u32 = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue];
    CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
    CMPT_HASH_FIND_2_BYTES!(mf, delta2, longestLen, matchesCount, cur, matches);
    CMPT_HASH_FIND_3_BYTES!(mf, delta2, delta3, longestLen, matchesCount, cur, matches);
    if (matchesCount != 0) {
        longestLen = CmptMemCmpLenSafe(cur, (cur - delta2), longestLen, niceLen);
        matches[matchesCount - 1].len = longestLen;
        if (longestLen == niceLen) {
            CmptBtSkip(mf, niceLen, pos, cur, curMatch);
            CMPT_MF_MOVE_POS!(mf);
            return matchesCount;
        }
    }
    if (longestLen < CMPT_MF_MATCH_3_BYTES!()) {
        longestLen = CMPT_MF_MATCH_3_BYTES!();
    }
    matchesCount =
        (CmptBtFind(mf, curMatch, (matches + matchesCount), longestLen) - matches).cast::<u32>();
    CMPT_MF_MOVE_POS!(mf);
    return matchesCount;
}

pub fn CmptlzMatchSkiper(mut mf: Ptr<CmptMfCtx>, mut amount: u32) {
    mf.readAhead += amount;
    let mut pos: u32 = Default::default();
    let mut temp: u32 = Default::default();
    let mut hash2Value: u32 = Default::default();
    let mut hash3Value: u32 = Default::default();
    let mut hashValue: u32 = Default::default();
    let mut curMatch: u32 = Default::default();
    let niceLen: u32 = mf.niceLen;
    c_do!({
        let mut lenLimit: u32 = (mf.srcLen - mf.readPos.cast::<usize>()).cast::<u32>();
        if CMPTLZ_LIKELY!(niceLen <= lenLimit) {
            lenLimit = niceLen;
        } else {
            mf.readPos += 1;
            continue;
        }
        let mut cur: Ptr<u8> = (mf.srcStart + mf.readPos.cast::<usize>());
        pos = mf.readPos + mf.offset;
        CMPT_HASH_4_CALC!(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf.hash[CMPTLZ_FIX_4_HASH!() + hashValue];
        CMPT_HASH_UPDATE!(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf, lenLimit, pos, cur, curMatch);
        CMPT_MF_MOVE_POS!(mf);
    } while amount.prefix_minus_minus() != 0);
}

pub fn CmptlzMatchFinder(
    mut mf: Ptr<CmptMfCtx>,
    mut pCount: Ptr<u32>,
    mut matches: Ptr<CmptlzMatchPair>,
) -> u32 {
    if CMPTLZ_UNLIKELY!((mf.srcLen - mf.readPos.cast::<usize>()) < mf.niceLen.cast::<usize>()) {
        *pCount = 0;
        mf.readPos += 1;
        mf.readAhead += 1;
        return 0;
    }
    let mut count: u32 = CmptlzBt4Finder(mf, matches);
    if (count == 0) {
        *pCount = 0;
        mf.readAhead += 1;
        return 0;
    }
    let mut longestLen: u32 = matches[count - 1].len;
    if (longestLen == mf.niceLen) {
        let mut bytesAvail: u32 = CMPTLZ_FIND_MIN!(
            (mf.srcLen - mf.readPos.cast::<usize>() + 1),
            CMPT_MF_LONGEST_MATCH!()
        )
        .cast();
        let mut p1: Ptr<u8> = (mf.srcStart + mf.readPos - 1).cast::<Ptr<u8>>();
        let mut p2: Ptr<u8> = (p1 - matches[count - 1].dist - 1);
        longestLen = CmptMemCmpLenSafe(p1, p2, longestLen, bytesAvail);
    }
    *pCount = count;
    mf.readAhead += 1;
    return longestLen;
}

pub fn CmptPriceOneBitDirect(mut bit: u32) -> u32 {
    return (bit << CMPT_PRICE_BITS_MOVING_NUM!()).cast();
}

pub fn CmptPriceOneBit(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut bit0Prob: CmptlzProb,
    mut curbit: u32,
) -> u32 {
    let tmp0 = (bit0Prob as u32 ^ ((0 - curbit) as u32 & (CMPTLZ_PROB_MAX_NUM!() - 1)))
        >> CMPT_PRICE_BITS_MOVING_NUM!();
    return encCtx.priceRootTable[tmp0];
}

pub fn CmptPriceBit0(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb) -> u32 {
    return encCtx.priceRootTable[(bit0Prob >> CMPT_PRICE_BITS_MOVING_NUM!()).cast::<usize>()]
        .cast();
}

pub fn CmptPriceBit1(mut encCtx: Ptr<CmptLzEncCtx>, mut bit0Prob: CmptlzProb) -> u32 {
    return encCtx.priceRootTable
        [(bit0Prob ^ (CMPTLZ_PROB_MAX_NUM!() - 1)) >> CMPT_PRICE_BITS_MOVING_NUM!()]
    .cast();
}

pub fn CmptPriceSymbol(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut symbolProbs: Ptr<CmptlzProb>,
    mut symbolBitsNum: u32,
    mut symbol: u32,
) -> u32 {
    let mut price: u32 = 0;
    symbol += (1 << symbolBitsNum);
    c_do!({
        let mut bit: u32 = (symbol & 1);
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[symbol], bit);
    } while symbol != 1);
    return price;
}

pub fn CmptPriceSymbolReverse(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut symbolProbs: Ptr<CmptlzProb>,
    mut symbolBitsNum: u32,
    mut symbol: u32,
) -> u32 {
    let mut price: u32 = 0;
    let mut i: u32 = 1;
    c_do!({
        let mut bit: u32 = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[i], bit).cast::<u32>();
        i = (i << 1) + bit;
    } while symbolBitsNum.prefix_minus_minus() != 0);
    return price;
}

pub fn CmptPriceGenRootTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut rootTable: Ptr<u32> = encCtx.priceRootTable.cast();
    const expandCycleNum: u32 = 4;
    const bitsTotalModeNum: u32 = 11;
    const valueForNormal: u32 = 15;
    const wTopBoarder: u32 = 1 << 16;
    c_for!(let mut i: u32 = 0; i < ((1 << bitsTotalModeNum) >> CMPT_PRICE_BITS_MOVING_NUM!()).cast(); i.suffix_plus_plus(); {
        let mut w: u32 = (i << CMPT_PRICE_BITS_MOVING_NUM!()) + (1 << (CMPT_PRICE_BITS_MOVING_NUM!() - 1));
        let mut dummyNormalizeCnt: u32 = 0;
        c_for!(let mut j: u32 = 0; j < expandCycleNum; j.suffix_plus_plus(); {
            w = w * w;
            dummyNormalizeCnt <<= 1;
            while (w >= wTopBoarder).as_bool() {
                w >>= 1;
                dummyNormalizeCnt += 1;
            }
        });
        rootTable[i] = ((bitsTotalModeNum << expandCycleNum) - valueForNormal - dummyNormalizeCnt).cast();
    });
}

pub fn CmptPriceGenDistTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut distState: u32 = 0;
    c_do!({
        let mut tmpPriceDistSlot: Ptr<u32> = encCtx.priceDistSlotTable[distState].cast();
        c_for!(let mut i: u32 = 0; i < encCtx.distTableSize; i.suffix_plus_plus(); {
            tmpPriceDistSlot[i] = CmptPriceSymbol(encCtx, encCtx.probDistSlot[distState].cast(), CMPTLZ_DIST_SLOT_BITS!(), i);
        });
        c_for!(let mut i: u32 = 14; i < encCtx.distTableSize; i.suffix_plus_plus(); {
            tmpPriceDistSlot[i] += CmptPriceOneBitDirect(((i >> 1) - 1 - CMPTLZ_ALIGN_BITS!()));
        });
        c_for!(let mut i: u32 = 0; i < 4; i.suffix_plus_plus(); {
            encCtx.priceDistTable[distState][i] = tmpPriceDistSlot[i];
        });
        distState.suffix_plus_plus();
    } while distState < CMPTLZ_DIST_STATE_TOTAL!());
    c_for!(let mut i: u32 = 4; i < 128; i.suffix_plus_plus(); {
        let mut distSlot: u32 = PosSloter(i);
        let mut footerBits: u32 = ((distSlot >> 1) - 1);
        let mut base: u32 = ((2 | (distSlot & 1)) << footerBits);
        let mut price: u32 = CmptPriceSymbolReverse(encCtx, encCtx.probDistSpecial.cast::<Ptr<u16>>() + base - distSlot - 1, footerBits, i - base);
        c_for!(distState = 0; distState < 4; distState.suffix_plus_plus(); {
            encCtx.priceDistTable[distState][i] = (price + encCtx.priceDistSlotTable[distState][distSlot]);
        });
    });
    encCtx.matchPriceCount = 0;
}

pub fn CmptPriceGenAlignTable(mut encCtx: Ptr<CmptLzEncCtx>) {
    c_for!(let mut i: u32 = 0; i < (1 << CMPTLZ_ALIGN_BITS!()).cast(); i.suffix_plus_plus(); {
        encCtx.priceAlignTable[i] = CmptPriceSymbolReverse(encCtx.cast(), encCtx.probAlign.cast(), CMPTLZ_ALIGN_BITS!(), i.cast()).cast();
    });
}

pub fn CmptPriceLiteral(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut matchMode: bool,
    mut matchByte: u32,
    mut symbol: u32,
) -> u32 {
    let mut pos: u32 = encCtx.litMarcov.pos;
    let mut prevByte: u32 = encCtx.litMarcov.prevByte;
    let mut litCtx: u32 = encCtx.litMarcov.lcBits;
    let mut lpMask: u32 = encCtx.litMarcov.posMask;
    let mut subCoder: Ptr<CmptlzProb> =
        CMPT_LIT_SUBCODER!(encCtx.litMarcov.literal, litCtx, lpMask, pos, prevByte)
            .cast::<Ptr<CmptlzProb>>();
    let mut price: u32 = 0;
    if !matchMode {
        price = CmptPriceSymbol(encCtx, subCoder, 8, symbol);
    } else {
        let mut offset: u32 = 0x100;
        symbol += 1 << 8;
        c_do!({
            matchByte <<= 1;
            let mut matchBit: u32 = (matchByte & offset);
            let mut subCoderIndex: u32 = (offset + matchBit + (symbol >> 8));
            let mut bit: u32 = ((symbol >> 7) & 1);
            price += CmptPriceOneBit(encCtx, subCoder[subCoderIndex], bit);
            symbol <<= 1;
            offset &= !(matchByte ^ symbol);
        } while symbol < (1 << 16));
    }
    return price;
}

pub fn CmptPriceSet(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut probs: Ptr<CmptlzProb>,
    mut startPrice: u32,
    mut prices: Ptr<u32>,
) {
    let mut i: u32 = 0;
    c_for!(i = 0; i < 8; i += 2; {
        let mut price: u32 = startPrice;
        let mut prob: u32;
        price += CmptPriceOneBit(encCtx, probs[1], (i >> 2));
        price += CmptPriceOneBit(encCtx, probs[2 + (i >> 2)], ((i >> 1) & 1));
        prob = probs[4 + (i >> 1)].cast();
        prices[i] = (price + CmptPriceBit0(encCtx, prob.cast()));
        prices[i + 1] = (price + CmptPriceBit1(encCtx, prob.cast()));
    });
}

pub fn CmptPriceGenLenTable(mut encCtx: Ptr<CmptLzEncCtx>, mut lenEncoder: Ptr<CmptLenEncoder>) {
    let numPosStates: u32 = 1 << encCtx.posBits;
    let mut b: u32;
    let mut prob: u32 = lenEncoder.low[0].cast::<u32>();
    let mut a: u32;
    let mut c: u32;
    let mut posState: u32;
    b = CmptPriceBit1(encCtx, prob.cast());
    a = CmptPriceBit0(encCtx, prob.cast());
    c = (b + CmptPriceBit0(encCtx, lenEncoder.low[1 << CMPT_LEN_LOW_BITS!()].cast()));
    c_for!(posState = 0; posState < numPosStates; posState.suffix_plus_plus(); {
        let mut prices: Ptr<u32> = lenEncoder.prices[posState].cast();
        let mut probs: Ptr<CmptlzProb> = lenEncoder.low.cast::<Ptr<u16>>() + (posState << (1 + CMPT_LEN_LOW_BITS!()));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, probs + (1 << CMPT_LEN_LOW_BITS!()), c, prices + (1 << CMPT_LEN_LOW_BITS!()));
    });
    let mut i: u32 = lenEncoder.tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!()) {
        let mut probs: Ptr<CmptlzProb> = lenEncoder.high.cast();
        let mut prices: Ptr<u32> =
            lenEncoder.prices[0].cast::<Ptr<u32>>() + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!();
        i -= (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!() - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder.low[(1 << CMPT_LEN_LOW_BITS!())].cast());
        c_do!({
            let mut sym: u32 = (i.prefix_minus_minus() + (1 << (CMPT_LEN_HIGH_BITS!() - 1)));
            let mut price: u32 = b;
            c_do!({
                let mut bit: u32 = (sym & 1);
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym].cast(), bit);
            } while sym >= 2);
            prob = probs[(i + (1 << (CMPT_LEN_HIGH_BITS!() - 1)))].cast();
            prices[(i * CMPT_DOUBLE!())] = (price + CmptPriceBit0(encCtx, prob.cast()));
            prices[(i * CMPT_DOUBLE!() + 1)] = (price + CmptPriceBit1(encCtx, prob.cast()));
        } while i != 0);
        let mut num: usize = ((lenEncoder.tableSize
            - (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!())
            * c_sizeofval!(lenEncoder.prices[0][0]))
        .try_into()
        .unwrap();
        c_for!(posState = 1; posState < numPosStates; posState.suffix_plus_plus(); {
             c_memcpy_s!(lenEncoder.prices[posState].cast::<Ptr<u32>>() + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), CMPT_MF_LONGEST_MATCH!() - 1,
                        lenEncoder.prices[0].cast::<Ptr<u32>>() + (1 << CMPT_LEN_LOW_BITS!()) * CMPT_DOUBLE!(), num);
        });
    }
}

pub fn CmptPriceLen(mut lenEncoder: Ptr<CmptLenEncoder>, mut len: u32, mut posState: u32) -> u32 {
    return lenEncoder.prices[posState][len - CMPTLZ_MATCH_LEN_MIN!()].cast();
}

pub fn CmptPriceShortRep(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut state: CmptlzState,
    mut posState: u32,
) -> u32 {
    return (CmptPriceBit0(encCtx.cast(), encCtx.isRepG0[state].cast()).cast::<u32>()
        + CmptPriceBit0(encCtx.cast(), encCtx.isRep0Long[state][posState].cast()).cast::<u32>())
    .cast();
}

pub fn CmptPriceLongRep(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut longRepIndex: u32,
    mut state: CmptlzState,
    mut posState: u32,
) -> u32 {
    let mut price: u32 = 0;
    c_switch!(longRepIndex;
        0 => {
            let tmp0 = state;
            price = CmptPriceBit0(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRep0Long[state][posState]);
            break;
        },
        1 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit0(encCtx, encCtx.isRepG1[state]));
            break;
        },
        2 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit0(encCtx, encCtx.isRepG2[state]));
            break;
        },
        3 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit1(encCtx, encCtx.isRepG2[state]));
            break;
        },
        _ => {
            break;
        },
    );
    return price;
}

pub fn CmptPriceDistWithLen(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut dist: u32,
    mut len: u32,
    mut posState: u32,
) -> u32 {
    let mut distState: u32 = CMPT_GET_DIST_STATE!(len);
    let mut price: u32;
    if (dist < 128) {
        price = encCtx.priceDistTable[distState][dist];
    } else {
        let mut distSlot: u32 = PosSloter(dist);
        price = (encCtx.priceDistSlotTable[distState][distSlot]
            + encCtx.priceAlignTable[dist & ((1 << CMPTLZ_ALIGN_BITS!()) - 1)]);
    }
    price += CmptPriceLen(c_ref!(encCtx.matchLenEncoder), len, posState);
    return price;
}

pub fn CmptRcLitProcess(mut rcCtx: Ptr<CmptRcCtx>, mut prob: Ptr<CmptlzProb>, mut sym: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut curBit: u32;
    sym |= 0x100;
    while (sym < 0x10000) {
        let mut litProbTableIndex: Ptr<CmptlzProb> = (prob + (sym >> 8));
        curBit = (sym >> 7) & 1;
        CMPT_RC_BIT_PROCESS!(
            rcCtx,
            litProbTableIndex,
            curBit,
            bit0Prob,
            range,
            newBound,
            shiftRes
        );
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        sym <<= 1;
    }
    rcCtx.range = range;
    return CMPT_OK!();
}

pub fn CmptRcLitAfterMatch(
    mut rcCtx: Ptr<CmptRcCtx>,
    mut prob: Ptr<CmptlzProb>,
    mut sym: u32,
    mut matchByte: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut offs: u32 = 0x100;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut curBit: u32;
    c_for!(sym |= 0x100; sym < 0x10000;; {
        matchByte <<= 1;
        let mut litProbTableIndex: Ptr<CmptlzProb> = (prob + (offs + (matchByte & offs) + (sym >> 8)));
        curBit = (sym >> 7) & 1;
        sym <<= 1;
        offs &= !(matchByte ^ sym);
        CMPT_RC_BIT_PROCESS!(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    });
    rcCtx.range = range;
    return CMPT_OK!();
}

pub fn CmptRcPrepare(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut dest: Ptr<u8>,
    mut destLen: Ptr<usize>,
    mut alloc: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut rc: Ptr<CmptRcCtx> = ((alloc.CmptLzAlloc)(
        CMPTLZ_RC_CCTX_HANDLE!(),
        c_sizeof!(CmptRcCtx).cast::<usize>(),
    ))
    .cast::<Ptr<CmptRcCtx>>();
    if (rc == NULL!()) {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    c_memset_s!(rc, c_sizeof!(CmptRcCtx), 0, c_sizeof!(CmptRcCtx)).cast::<Void>();
    encCtx.rcCtx = rc;
    rc.bufBase = ((alloc.CmptLzAlloc)(
        CMPTLZ_RC_BUF_HANDLE!(),
        CMPTLZ_RC_BUFFER_SIZE!().cast::<usize>(),
    ))
    .cast::<Ptr<u8>>();
    c_memset_s!(
        rc.bufBase,
        CMPTLZ_RC_BUFFER_SIZE!(),
        0,
        CMPTLZ_RC_BUFFER_SIZE!()
    )
    .cast::<Void>();
    if (rc.bufBase == NULL!()) {
        return CMPT_ENC_RC_INIT_FAIL!();
    }
    rc.outBufLeft = *destLen;
    rc.outBuf = dest;
    rc.buf = rc.bufBase;
    rc.range = 0xFFFFFFFF;
    rc.cacheSize = 0;
    rc.cache = 0;
    rc.low = 0;
    return 0;
}

pub fn CmptRcFlush64Kb(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut flushOutLen: usize = (rcCtx.buf - rcCtx.bufBase).cast();
    let mut res: i32 = c_memcpy_s!(rcCtx.outBuf, rcCtx.outBufLeft, rcCtx.bufBase, flushOutLen);
    if (res != 0).as_bool() {
        return CMPT_ENC_ERROR_WRITE!();
    }
    rcCtx.outBuf += flushOutLen;
    rcCtx.outBufLeft -= flushOutLen;
    rcCtx.buf = rcCtx.bufBase.cast();
    return CMPT_OK!();
}

pub fn CmptRcShiftLow(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut res: i32 = CMPT_OK!();
    let mut lowLow32: u32 = rcCtx.low.cast::<u32>();
    let mut high: u64 = (rcCtx.low >> 32).cast::<u32>().cast::<u64>();
    rcCtx.low = (lowLow32 << 8).cast();
    CMPT_RC_BREAK_CHECK!(rcCtx, rcCtx.buf, res);
    if (lowLow32 < 0xFF000000 || high != 0) {
        let mut buf: Ptr<u8> = rcCtx.buf;
        *buf = (rcCtx.cache + high).cast::<u8>();
        buf += 1;
        rcCtx.buf = buf;
        rcCtx.cache = (lowLow32 >> 24).cast::<u64>();
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
        high += 0xFF;
        loop {
            let mut buf1: Ptr<u8> = rcCtx.buf;
            CMPT_RC_BREAK_SHIFTING!(rcCtx, buf1, res);
            *buf1 = high.cast::<u8>();
            buf1 += 1;
            rcCtx.buf = buf1;
            rcCtx.cacheSize -= 1;
        }
        CMPT_RC_BREAK_SHIFTING!(rcCtx, buf, res);
    } else {
        rcCtx.cacheSize += 1;
    }
    return res;
}

pub fn CmptRcFlushData(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut i: i32;
    let mut res: i32 = CMPT_OK!();
    c_for!(i = 0; i < 5; i.suffix_plus_plus(); {
        res = CmptRcShiftLow(rcCtx);
        if (res != CMPT_OK!()) {
            break;
        }
    });
    return res;
}

pub fn CmptRcLenProcess(
    mut lenEncoder: Ptr<CmptLenEncoder>,
    mut rcCtx: Ptr<CmptRcCtx>,
    mut len: u32,
    mut posState: u64,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut newBound: u32;
    let mut bit0Prob: u32;
    len -= CMPTLZ_MATCH_LEN_MIN!();
    let mut probs: Ptr<CmptlzProb> = lenEncoder.low.cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    if (len >= CMPT_LEN_BOUND!()) {
        CMPT_RC_BIT_1_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        if (shiftRes != CMPT_OK!()) {
            return shiftRes;
        }
        probs += CMPT_LEN_BOUND!();
        CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
        if (len >= CMPT_LEN_BOUND!() * CMPT_DOUBLE!()) {
            CMPT_RC_BIT_1_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
            rcCtx.range = range;
            shiftRes = CmptRcLitProcess(
                rcCtx,
                lenEncoder.high.cast(),
                len - CMPT_LEN_BOUND!() * CMPT_DOUBLE!(),
            );
            if (shiftRes != CMPT_OK!()) {
                return shiftRes;
            }
            return CMPT_OK!();
        }
        len -= CMPT_LEN_BOUND!();
    }
    let mut m: u32;
    let mut bit: u32;
    CMPT_RC_BIT_0_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    if (shiftRes != CMPT_OK!()) {
        return shiftRes;
    }
    probs += (posState << (1 + 3)).cast::<usize>();
    bit = (len >> 2);
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + 1, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()) {
        return shiftRes;
    }
    m = (1 << 1) + bit;
    bit = (len >> 1) & 1;
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()) {
        return shiftRes;
    }
    m = (m << 1) + bit;
    bit = len & 1;
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()) {
        return shiftRes;
    }
    rcCtx.range = range;
    return CMPT_OK!();
}

pub fn CmptRcPosSlotProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut posSlot: u32, mut len: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = encCtx.rcCtx.range;
    let mut sym: u32 = posSlot + (1 << 6);
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut bit: u32;
    let mut probs: Ptr<CmptlzProb> = encCtx.probDistSlot[GET_LEN_TO_POS_STATE!(len)].cast();
    c_do!({
        let mut posSlotProbTableIndex: Ptr<CmptlzProb> = probs + (sym >> CMPTLZ_DIST_SLOT_BITS!());
        bit = (sym >> (CMPTLZ_DIST_SLOT_BITS!() - 1)) & 1;
        sym <<= 1;
        CMPT_RC_BIT_PROCESS!(encCtx.rcCtx, posSlotProbTableIndex, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } while sym < (1 << (CMPTLZ_DIST_SLOT_BITS!() * 2)));
    encCtx.rcCtx.range = range;
    return CMPT_OK!();
}

pub fn CmptRcReverseProcess(
    mut rcCtx: Ptr<CmptRcCtx>,
    mut probs: Ptr<CmptlzProb>,
    mut numBits: u32,
    mut sym: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut bit0Prob: u32 = Default::default();
    let mut newBound: u32 = Default::default();
    let mut bit: u32 = Default::default();
    let mut m: u32 = 1;
    c_do!({
        bit = (sym & 1);
        sym >>= 1;
        CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        m = ((m << 1) | bit);
    } while numBits.prefix_minus_minus() != 0);

    rcCtx.range = range;
    return CMPT_OK!();
}

pub fn CmptRcDistProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut posSlot: u32, mut dist: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut footerBits: u32 = ((posSlot >> 1) - 1);
    if (dist < CMPT_DIST_LIMIT_2!()) {
        let mut base: u32 = ((2 | (posSlot & 1)) << footerBits);
        shiftRes = CmptRcReverseProcess(
            encCtx.rcCtx,
            encCtx.probDistSpecial.cast::<Ptr<u16>>() + base,
            footerBits,
            dist,
        );
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        let mut pos2: u32 = (dist | 0xF) << (32 - footerBits);
        let mut range: u32 = encCtx.rcCtx.range;
        c_do!({
            range >>= 1;
            encCtx.rcCtx.low += (range & (0 - (pos2 >> 31))) as u64;
            pos2 += pos2;
            CMPT_RC_NORMALIZE!(encCtx.rcCtx, range, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        } while pos2 != 0xF0000000);
        let mut m: u32 = 1;
        let mut bit: u32;
        let mut bit0Prob: u32;
        let mut newBound: u32;
        let mut k: i32;
        c_for!(k = 0; k < CMPTLZ_ALIGN_BITS!() - 1; k.suffix_plus_plus(); {
            bit = (dist & 1);
            dist >>= 1;
            CMPT_RC_BIT_PROCESS!(encCtx.rcCtx, encCtx.probAlign.cast::<Ptr<u16>>()  + m, bit, bit0Prob, range, newBound, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            m = (m << 1) + bit;
        });
        bit = (dist & 1);
        CMPT_RC_BIT_PROCESS!(
            encCtx.rcCtx,
            encCtx.probAlign.cast::<Ptr<u16>>() + m,
            bit,
            bit0Prob,
            range,
            newBound,
            shiftRes
        );
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        encCtx.rcCtx.range = range;
    }
    return CMPT_OK!();
}

pub fn CmptlzEncLit(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut mf: Ptr<CmptMfCtx>,
    mut nowpos32: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx;
    let mut posState: u32 = (nowpos32 & encCtx.pbMask.cast::<u32>());
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = rc.range;
    let tmp0 = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[tmp0][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    rc.range = range;
    let mut litProb: Ptr<CmptlzProb> = c_ref!(encCtx.litMarcov.literal[0][0]);
    let mut curByte: u8 = mf.srcStart[mf.readPos - mf.readAhead];
    let tmp0 = mf.readPos - mf.readAhead - 1;
    probs = CMPT_LIT_PROB_GET!(encCtx, litProb, nowpos32, mf.srcStart[tmp0].cast::<u32>());
    let mut state: CmptlzState = encCtx.state;
    CMPT_STATE_UPDATE_WHEN_LIT!(encCtx.state);
    if (state < 7) {
        shiftRes = CmptRcLitProcess(rc, probs, curByte.cast());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        let mut match_byte: u8 = mf.srcStart[mf.readPos - encCtx.reps[0] - 1 - mf.readAhead];
        shiftRes = CmptRcLitAfterMatch(rc, probs, curByte.cast(), match_byte.cast());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}

pub fn CmptlzEncShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask as u32;
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = encCtx.rcCtx.range;
    let tmp0 = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[tmp0][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRep[tmp0]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRepG0[tmp0]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRep0Long[tmp0][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range;
    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_SHORTREP!(state);
    return CMPT_OK!();
}

pub fn CmptlzEncNormalMatch(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut nowpos32: u32,
    mut backRes: u32,
    mut lenRes: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask as u32;
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = encCtx.rcCtx.range;
    let tmp0 = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[tmp0][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRep[tmp0]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range;
    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_MATCH!(state);
    shiftRes = CmptRcLenProcess(
        c_ref!(encCtx.matchLenEncoder),
        encCtx.rcCtx,
        lenRes,
        posState.cast(),
    );
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    backRes -= CMPTLZ_NUM_REPS!();
    encCtx.reps[3] = encCtx.reps[2];
    encCtx.reps[2] = encCtx.reps[1];
    encCtx.reps[1] = encCtx.reps[0];
    encCtx.reps[0] = backRes;
    encCtx.matchPriceCount += 1;
    let mut posSlot: u32 = PosSloter(backRes);
    shiftRes = CmptRcPosSlotProcess(encCtx, posSlot, lenRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    if (backRes >= 4) {
        shiftRes = CmptRcDistProcess(encCtx, posSlot, backRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}

pub fn CmptlzEncLongRep(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut repIndex: u32,
    mut nowpos32: u32,
    mut lenRes: u32,
) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask as u32;
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut realDist: u32;
    range = encCtx.rcCtx.range;
    let tmp0 = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[tmp0][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRep[tmp0]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let tmp0 = encCtx.state;
    probs = c_ref!(encCtx.isRepG0[tmp0]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    c_switch!(repIndex;
        0 => {
            CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRep0Long[tmp0][posState]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1!(encCtx.rcCtx, probs, newBound, range, bit0Prob);
            break;
        },
        1 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRepG1[tmp0]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0!(probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
            break;
        },
        2 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRepG1[tmp0]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRepG2[tmp0]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0!(probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[2];
            encCtx.reps[2] = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
            break;
        },
        3 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRepG1[tmp0]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
            let tmp0 = encCtx.state;
            probs = c_ref!(encCtx.isRepG2[tmp0]);
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1!(encCtx.rcCtx, probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[3];
            encCtx.reps[3] = encCtx.reps[2];
            encCtx.reps[2] = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
            break;
        },
        _ => {
            break;
        },
    );
    CMPT_RC_NORMALIZE!(encCtx.rcCtx, range, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range;
    shiftRes = CmptRcLenProcess(
        c_ref!(encCtx.repLenEncoder),
        encCtx.rcCtx,
        lenRes,
        posState.cast(),
    );
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.repLenPriceCount -= 1;
    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_LONGREP!(state);
    return CMPT_OK!();
}

pub fn CmptlzFreeAll(mut encCtx: Ptr<CmptLzEncCtx>, mut alloc: Ptr<CmptLzMemHook>) {
    if (encCtx == NULL!()) {
        return;
    }
    if (encCtx.mfCtx != NULL!()) {
        if (encCtx.mfCtx.hash != NULL!()) {
            (alloc.CmptLzFree)(CMPTLZ_MF_HASH_HANDLE!(), encCtx.mfCtx.hash.cast());
            encCtx.mfCtx.hash = NULL!();
        }
        if (encCtx.mfCtx.son != NULL!()) {
            (alloc.CmptLzFree)(CMPTLZ_MF_SON_HANDLE!(), encCtx.mfCtx.son.cast());
            encCtx.mfCtx.son = NULL!();
        }
        (alloc.CmptLzFree)(CMPTLZ_MF_CCTX_HANDLE!(), encCtx.mfCtx.cast());
        encCtx.mfCtx = NULL!();
    }
    if (encCtx.rcCtx != NULL!()) {
        if (encCtx.rcCtx.bufBase != NULL!()) {
            (alloc.CmptLzFree)(CMPTLZ_RC_BUF_HANDLE!(), encCtx.rcCtx.bufBase.cast());
            encCtx.rcCtx.bufBase = NULL!();
        }
        (alloc.CmptLzFree)(CMPTLZ_RC_CCTX_HANDLE!(), encCtx.rcCtx.cast());
        encCtx.rcCtx = NULL!();
    }
    (alloc.CmptLzFree)(CMPTLZ_ENC_CCTX_HANDLE!(), encCtx.cast());
    encCtx = NULL!();
}

pub fn CmptlzEncodeIO(
    mut encCtx: Ptr<CmptLzEncCtx>,
    mut dest: Ptr<u8>,
    mut destLen: Ptr<usize>,
    mut src: Ptr<u8>,
    mut srcLen: usize,
    mut alloc: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut res: i32;
    res = CmptMfPrepare(encCtx, src, srcLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG!(res, cstr!("CmptMfPrepare Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    res = CmptRcPrepare(encCtx, dest, destLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG!(res, cstr!("CmptRcPrepare Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    CmptlzEncPrepare(encCtx);
    res = CmptEncodeAll(encCtx);
    if (res != 0) {
        CmptlzFreeAll(encCtx, alloc);
        CMPTLZ_LOG!(res, cstr!("CmptEncode Process Fail!"));
        return res;
    }
    *destLen -= encCtx.rcCtx.outBufLeft;
    if (encCtx.nowpos64 != srcLen.cast::<u64>()) {
        CMPTLZ_LOG!(res, cstr!("CmptEncode FileSize Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return CMPT_ENC_ERROR_FILESIZE!();
    }
    CmptlzFreeAll(encCtx, alloc);
    return res;
}

pub fn CmptlzEncode(
    mut dest: Ptr<u8>,
    mut destLen: Ptr<usize>,
    mut src: Ptr<u8>,
    mut srcLen: usize,
    mut props: Ptr<CmptlzEncParam>,
    mut propsEncoded: Ptr<u8>,
    mut propsSize: Ptr<usize>,
    mut writeEndMark: i32,
    mut alloc: Ptr<CmptLzMemHook>,
) -> i32 {
    let mut res: i32;
    if (alloc == NULL!()) || (alloc.CmptLzAlloc == NULL!()) || (alloc.CmptLzFree == NULL!()) {
        CMPTLZ_LOG!(CMPT_ENC_ERROR_PARAM!(), cstr!("Cmptlz input wrong param!"));
        return CMPT_ENC_ERROR_PARAM!();
    }
    let mut encCtx: Ptr<CmptLzEncCtx> =
        CmptInitCctx(alloc, writeEndMark).cast::<Ptr<CmptLzEncCtx>>();
    if (encCtx == NULL!()) {
        CMPTLZ_LOG!(CMPT_ENC_CTX_INIT_FAIL!(), cstr!("CmptInitCctx Fail!"));
        return CMPT_ENC_CTX_INIT_FAIL!();
    }
    CmptlzSetParam(encCtx, props);
    res = CmptHeadWrite(encCtx, propsEncoded, propsSize);
    if (res != 0) {
        (alloc.CmptLzFree)(CMPTLZ_ENC_CCTX_HANDLE!(), encCtx.cast::<Ptr<u8>>());
        CMPTLZ_LOG!(res, cstr!("CmptHeadWrite Fail!"));
        return res;
    }
    res = CmptlzEncodeIO(encCtx, dest, destLen, src, srcLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG!(res, cstr!("CmptlzEncode I / O Fail!"));
    }
    return res;
}

pub fn CmptlzCompress(
    mut src: Ptr<Void>,
    mut srcSize: usize,
    mut dst: Ptr<Void>,
    mut dstSize: Ptr<usize>,
    mut param: Ptr<CmptlzCompParam>,
) -> i32 {
    if (src == NULL!()).as_bool() && (srcSize != 0).as_bool() {
        return CMPT_ENC_ERROR_PARAM!();
    }
    let mut endMarker: i32 = 0;
    let mut props: CmptlzEncParam = Default::default();
    props.level = param.level.cast();
    props.dictSize = param.dictSize.cast();
    props.litCtx = param.litCtx.cast();
    props.litPos = param.litPos.cast();
    props.posBits = param.posBits.cast();
    props.fastBytes = param.fastBytes.cast();
    props.numThreads = param.numThreads.cast();
    let mut alloc: Ptr<CmptLzMemHook> = param.memHook.cast();
    return CmptlzEncode(
        dst.cast::<Ptr<u8>>(),
        dstSize.cast(),
        src.cast::<Ptr<u8>>(),
        srcSize.cast(),
        c_ref!(props).cast(),
        param.protData.cast(),
        c_ref!(param.protSize).cast(),
        endMarker.cast(),
        alloc.cast(),
    )
    .cast();
}
