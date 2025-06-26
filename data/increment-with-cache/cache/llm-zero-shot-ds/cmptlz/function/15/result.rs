use std::mem::MaybeUninit;

static inline int CmptRcLitAfterMatch(CmptRcCtx *rcCtx, CmptlzProb *prob, uint32_t sym, uint32_t matchByte)
    {
        int shiftRes = CMPT_OK;
        uint32_t range = rcCtx->range, offs = 0x100, bit0Prob, newBound, curBit;
        for (sym |= 0x100; sym < 0x10000;)
        {
            matchByte <<= 1;
            CmptlzProb *litProbTableIndex = prob + (offs + (matchByte & offs) + (sym >> 8));
            curBit = (sym >> 7) & 1;
            sym <<= 1;
            offs &= ~(matchByte ^ sym);
            CMPT_RC_BIT_PROCESS(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        }
        rcCtx->range = range;
        return CMPT_OK;
    }
