use std::ptr;

static CMPT_ERROR_DATA: i32 = -1;
static CMPT_OK: i32 = 0;
static CMPTLZ_LIT_STATES: u32 = 7;

#[repr(C)]
struct CmptLzDecCtx {
    processedPos: u32,
    dictBufSize: usize,
    dictPos: usize,
    dict: *const u8,
    prop: CmptLzProp,
    checkDicSize: u32,
    state: u32,
    reps: [u32; 4],
}

#[repr(C)]
struct CmptLzProp {
    litPos: u32,
    litCtx: u32,
}

type CmptLzDecProb = u16;

unsafe fn CmptLzTryDecLitPacket(
    decCtx: *mut CmptLzDecCtx,
    range: u32,
    rangeCode: u32,
    rangeBound: u32,
    bufTryDec: *const u8,
    pbufLimit: *mut *const u8,
) -> i32 {
    let decCtx = &mut *decCtx;
    let probsMatrix = CmptLzGetProbsMatrix(decCtx);
    let procPos = decCtx.processedPos;
    let litPosMask = ((0x100u32) << decCtx.prop.litPos) - ((0x100u32) >> decCtx.prop.litCtx);
    let dictBufSize = decCtx.dictBufSize;
    let dicPos = decCtx.dictPos;
    let dict = decCtx.dict;
    let mut bufLimit = *pbufLimit;

    if decCtx.dictPos >= decCtx.dictBufSize {
        return CMPT_ERROR_DATA;
    }

    let mut probSlot = CmptLzGetLiteralProb(probsMatrix);
    if procPos != 0 || decCtx.checkDicSize != 0 {
        let prevPos = if dicPos == 0 { dictBufSize } else { dicPos } - 1;
        let prevByte = *dict.add(prevPos);
        probSlot = probSlot.offset(
            (3 * ((((procPos << 8) + prevByte as u32) & litPosMask) << decCtx.prop.litCtx) as isize
        );
    }

    let mut decSym = 1u32;
    if decCtx.state < CMPTLZ_LIT_STATES {
        while decSym < 0x100 {
            let probBit = probSlot.offset(decSym as isize);
            CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        }
    } else {
        let matchByte = *dict.add(
            dicPos.wrapping_sub(decCtx.reps[0] as usize)
                .wrapping_add(if dicPos < decCtx.reps[0] as usize { dictBufSize } else { 0 })
        );
        let mut matchSym = matchByte as u32;
        let mut offset = 0x100u32;
        while decSym < 0x100 {
            matchSym <<= 1;
            let bit = offset;
            offset &= matchSym;
            let probBit = probSlot.offset((offset + bit + decSym) as isize);
            CMPTLZ_MATCH_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    CMPT_OK
}
