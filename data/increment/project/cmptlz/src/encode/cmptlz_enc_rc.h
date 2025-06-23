/**
 * @file cmptlz_enc_rc.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 区间编码头文件
 * @author Anonym
 * @date 2024-01-09
 */
#ifndef CMPTLZ_ENC_RC_H
#define CMPTLZ_ENC_RC_H

#include "cmptlz_enc_inner.h"
#include "cmptlz_enc_price.h"

#ifdef __cplusplus
extern "C" {
#endif

#define CMPT_RC_MIN_RANGE (1 << 24)
#define CMPT_NUM_LEN_POS_STATE 4
#define GET_LEN_TO_POS_STATE(len) (((len) < CMPT_NUM_LEN_POS_STATE + 1) ? (len) - 2 : \
    CMPT_NUM_LEN_POS_STATE - 1)

#define CMPT_RC_BREAK_CHECK(rcCtx, buf, res) \
    do { \
        if ((buf) == (rcCtx->bufBase + CMPTLZ_RC_BUFFER_SIZE)) { \
            (res) = CmptRcFlush64Kb(rcCtx); \
            CMPTLZ_RETURN_IF_NOT_OK(res); \
        } \
    } while (0)

#define CMPT_RC_BREAK_SHIFTING(rcCtx, buf, res) \
    do { \
        CMPT_RC_BREAK_CHECK(rcCtx, buf, res); \
        if ((rcCtx)->cacheSize == 0) { \
            return CMPT_OK; \
        } \
    } while (0)

#define CMPT_RC_NORMALIZE(rcCtx, range, shiftRes) \
    do { \
        if ((range) < CMPT_RC_MIN_RANGE) { \
            (range) <<= 8;   \
            (shiftRes) = CmptRcShiftLow(rcCtx); \
        } \
    } while (0)

#define CMPT_RC_GET_NEWBOUND(prob, bit0Prob, range, newBound) \
    do { \
        (bit0Prob) = *(prob); \
        newBound = ((range) >> 11) * (bit0Prob); \
    } while (0)

#define CMPT_RC_BIT_PROCESS(rcCtx, prob, bit, bit0Prob, range, newBound, shiftRes) { \
    do { \
        uint32_t mask = 0 - (uint32_t)(bit); \
        CMPT_RC_GET_NEWBOUND(prob, bit0Prob, range, newBound); \
        (range) &= mask;  \
        mask &= (newBound); \
        (range) -= mask; \
        (rcCtx)->low += mask; \
        mask = (uint32_t)(bit) - 1; \
        (range) += (newBound) & mask; \
        mask &= (CMPTLZ_PROB_MAX_NUM - ((1 << 5) - 1)); \
        mask += ((1 << 5) - 1); \
        (bit0Prob) += (int)(mask - (bit0Prob)) >> 5; \
        *(prob) = (CmptlzProb)(bit0Prob); \
        CMPT_RC_NORMALIZE(rcCtx, range, shiftRes); \
    } while (0); \
}

#define CMPT_RC_BIT_0(prob, newBound, range, bit0Prob) \
    do { \
        (range) = (newBound); \
        *(prob) = (CmptlzProb)((bit0Prob) + ((CMPTLZ_PROB_MAX_NUM - (bit0Prob)) >> 5)); \
    } while (0)

#define CMPT_RC_BIT_1(rcCtx, prob, newBound, range, bit0Prob) \
    do { \
        (range) -= (newBound); \
        (rcCtx)->low += (newBound); \
        *(prob) = (CmptlzProb)((bit0Prob) - ((bit0Prob) >> 5)); \
    } while (0)

#define CMPT_RC_BIT_0_PROCESS(rcCtx, prob, newBound, range, bit0Prob, shiftRes) \
    do { \
        CMPT_RC_BIT_0(prob, newBound, range, bit0Prob); \
        CMPT_RC_NORMALIZE(rcCtx, range, shiftRes); \
    } while (0)

#define CMPT_RC_BIT_1_PROCESS(rcCtx, prob, newBound, range, bit0Prob, shiftRes) \
    do { \
        CMPT_RC_BIT_1(rcCtx, prob, newBound, range, bit0Prob); \
        CMPT_RC_NORMALIZE(rcCtx, range, shiftRes); \
    } while (0)

CMPTLZ_HIDDEN int CmptRcShiftLow(CmptRcCtx *rcCtx);
ALWAYS_NO_INLINE int CmptRcFlush64Kb(CmptRcCtx *rcCtx);
CMPTLZ_HIDDEN int CmptRcFlushData(CmptRcCtx *rcCtx);
CMPTLZ_HIDDEN void CmptRcCtxInit(CmptRcCtx *rcCtx);
CMPTLZ_HIDDEN int CmptRcLenProcess(CmptLenEncoder *lenEncoder, CmptRcCtx *rcCtx, uint32_t len, uint64_t posState);
CMPTLZ_HIDDEN int CmptRcPosSlotProcess(CmptLzEncCtx *encCtx, uint32_t posSlot, uint32_t len);
CMPTLZ_HIDDEN int CmptRcDistProcess(CmptLzEncCtx *encCtx, uint32_t posSlot, uint32_t dist);

static ALWAYS_INLINE int CmptRcLitProcess(CmptRcCtx *rcCtx, CmptlzProb *prob, uint32_t sym)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range, bit0Prob, newBound, curBit;

    for (sym |= 0x100; sym < 0x10000; sym <<= 1) {
        CmptlzProb *litProbTableIndex = prob + (sym >> 8); // 右移8位
        curBit = (sym >> 7) & 1; // 右移7位
        CMPT_RC_BIT_PROCESS(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    rcCtx->range = range;
    return CMPT_OK;
}

static ALWAYS_INLINE int CmptRcLitAfterMatch(CmptRcCtx *rcCtx, CmptlzProb *prob, uint32_t sym, uint32_t matchByte)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range, offs = 0x100, bit0Prob, newBound, curBit;
    for (sym |= 0x100; sym < 0x10000;) {
        matchByte <<= 1;
        CmptlzProb *litProbTableIndex = prob + (offs + (matchByte & offs) + (sym >> 8)); // 右移8位
        curBit = (sym >> 7) & 1; // 右移7位
        sym <<= 1;
        offs &= ~(matchByte ^ sym);
        CMPT_RC_BIT_PROCESS(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    rcCtx->range = range;
    return CMPT_OK;
}

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_ENC_CmptRcH */
