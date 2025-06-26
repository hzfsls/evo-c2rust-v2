/**
 * @file cmptlz_enc_symbol.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 对rc.c进行封装，从packet层次推进压缩过程
 * @author Anonym
 * @date 2024-01-09
 */
#ifndef CMPTLZ_ENC_SYMBOL_H
#define CMPTLZ_ENC_SYMBOL_H

#include "cmptlz_enc_inner.h"
#include "cmptlz_enc_rc.h"

#ifdef __cplusplus
extern "C" {
#endif

#define CMPT_LIT_PROB_GET(encCtx, litProb, pos, prevByte) \
    (litProb + (uint32_t)3 * (((((pos) << 8) + \
        (prevByte)) & encCtx->lpMask) << encCtx->litMarcov.lcBits))

static ALWAYS_INLINE int CmptlzEncLit(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t nowpos32)
{
    int shiftRes = CMPT_OK;
    CmptRcCtx *rc = encCtx->rcCtx;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = rc->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(rc, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    rc->range = range;
    CmptlzProb *litProb = &encCtx->litMarcov.literal[0][0];
    const uint8_t curByte = mf->srcStart[mf->readPos - mf->readAhead];
    probs = CMPT_LIT_PROB_GET(encCtx, litProb, nowpos32, mf->srcStart[mf->readPos - mf->readAhead - 1]);
    CmptlzState state = encCtx->state;
    CMPT_STATE_UPDATE_WHEN_LIT(encCtx->state);
    if (state < 7) { // 7参考状态表
        shiftRes = CmptRcLitProcess(rc, probs, curByte);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    } else {
        const uint8_t match_byte = mf->srcStart[mf->readPos - encCtx->reps[0] - 1 - mf->readAhead];
        shiftRes = CmptRcLitAfterMatch(rc, probs, curByte, match_byte);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

static ALWAYS_INLINE int CmptlzEncShortRep(CmptLzEncCtx *encCtx, uint32_t nowpos32)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = encCtx->rcCtx->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // ismatch ==  1
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isrep == 1
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRepG0[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG0 == 0
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRep0Long[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRep0Long == 0
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    encCtx->rcCtx->range = range;

    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_SHORTREP(state);
    return CMPT_OK;
}

static ALWAYS_INLINE int CmptlzEncNormalMatch(CmptLzEncCtx *encCtx,
    uint32_t nowpos32, uint32_t backRes, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = encCtx->rcCtx->range;

    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // ismatch ==  1
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRep ==  0
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    encCtx->rcCtx->range = range;
    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_MATCH(state);

    /* 编len */
    shiftRes = CmptRcLenProcess(&encCtx->matchLenEncoder, encCtx->rcCtx, lenRes, posState);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    backRes -= CMPTLZ_NUM_REPS;
    encCtx->reps[3] = encCtx->reps[2];
    encCtx->reps[2] = encCtx->reps[1];
    encCtx->reps[1] = encCtx->reps[0];
    encCtx->reps[0] = backRes;

    encCtx->matchPriceCount++;
    /* 编dist注意从0开始的 */
    uint32_t posSlot = PosSloter(backRes);
    shiftRes = CmptRcPosSlotProcess(encCtx, posSlot, lenRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    if (backRes >= 4) { // 区间号0，1，2，3不需要额外编码
        shiftRes = CmptRcDistProcess(encCtx, posSlot, backRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

static ALWAYS_INLINE int CmptlzEncLongRep(CmptLzEncCtx *encCtx, uint32_t repIndex, uint32_t nowpos32, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    /* 先编prefix */
    uint32_t range, bit0Prob, newBound;
    uint32_t realDist;
    range = encCtx->rcCtx->range;

    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // ismatch ==  1
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isrep == 1
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

    probs = &encCtx->isRepG0[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);

    switch (repIndex) {
        case 0:
            CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG0 == 0
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRep0Long[encCtx->state][posState];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1(encCtx->rcCtx, probs, newBound, range, bit0Prob); // isRep0Long == 1
            break;
        case 1:
            CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG0 == 1
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRepG1[encCtx->state];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0(probs, newBound, range, bit0Prob); // isRepG1 == 0

            realDist = encCtx->reps[1]; // 取出真实dist
            encCtx->reps[1] = encCtx->reps[0];
            encCtx->reps[0] = realDist;
            break;
        case 2:
            CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG0 == 1
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRepG1[encCtx->state];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG1 == 1
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRepG2[encCtx->state];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0(probs, newBound, range, bit0Prob); // isRepG2 == 0

            realDist = encCtx->reps[2];
            encCtx->reps[2] = encCtx->reps[1];
            encCtx->reps[1] = encCtx->reps[0];
            encCtx->reps[0] = realDist;
            break;
        case 3:
            CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG0 == 1
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRepG1[encCtx->state];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes); // isRepG1 == 1
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);

            probs = &encCtx->isRepG2[encCtx->state];
            CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1(encCtx->rcCtx, probs, newBound, range, bit0Prob); // isRepG2 == 1
            realDist = encCtx->reps[3];
            encCtx->reps[3] = encCtx->reps[2];
            encCtx->reps[2] = encCtx->reps[1];
            encCtx->reps[1] = encCtx->reps[0];
            encCtx->reps[0] = realDist;
            break;
        default:
            break;
    }
    CMPT_RC_NORMALIZE(encCtx->rcCtx, range, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    encCtx->rcCtx->range = range;

    /* 再编len */
    shiftRes = CmptRcLenProcess(&encCtx->repLenEncoder, encCtx->rcCtx, lenRes, posState);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    --encCtx->repLenPriceCount;

    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_LONGREP(state);
    return CMPT_OK;
}

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_ENC_SYMBOL_H */