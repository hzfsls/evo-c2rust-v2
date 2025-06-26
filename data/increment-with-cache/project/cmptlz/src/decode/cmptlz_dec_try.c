/**
 * @file cmptlz_dec_try.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压校验流程文件
 * @author Anonym
 * @date 2023-07-06
 * @version v0.1.0
 ********************************************************************************************
 * @par History
 * <table>
 * <tr><th>Date        <th>Version   <th>Author      <th>Description
 * <tr><td>2023-07-06  <td>0.1.0     <td>   <td>Init version
 * </table>
 ********************************************************************************************
 */

#include "cmptlz_dec_inner.h"

#define CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound) \
    do { \
        (range) = (rangeBound); \
    } while (0)

#define CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound) \
    do { \
        (range) -= (rangeBound); \
        (rangeCode) -= (rangeBound); \
    } while (0)

#define CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit) \
    do { \
        if ((range) < CMPTLZ_RANGE_DOWN_LIMIT) { \
            if ((bufTryDec) >= (bufLimit)) { \
                return CMPTLZ_DEC_INPUT_EOF; \
            } \
            (range) <<= CMPTLZ_ONE_BYTE_WIDTH; \
            (rangeCode) <<= CMPTLZ_ONE_BYTE_WIDTH; \
            (rangeCode) |= (*(bufTryDec)++); \
        } \
    } while (0)

#define CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probSym) \
    do { \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probSym)); \
        if ((rangeCode) < (rangeBound)) { \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound); \
            (decSym) = ((decSym) << 1); \
        } else { \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound); \
            (decSym) = ((decSym) << 1) + 1; \
        } \
    } while (0)

#define CMPTLZ_MATCH_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probSym) \
    do { \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probSym)); \
        if ((rangeCode) < (rangeBound)) { \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound); \
            (decSym) = ((decSym) << 1); \
            (offset) ^= (bit); \
        } else { \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound); \
            (decSym) = ((decSym) << 1) + 1; \
        } \
    } while (0)

static ALWAYS_INLINE int CmptLzTryDecLenAndDist(CmptLzDecCtx *decCtx, uint32_t mkState,
    uint32_t range, uint32_t rangeCode, uint32_t rangeBound, CmptLzDecProb *probSlot,
    const unsigned char *bufTryDec, const unsigned char **pbufLimit)
{
    uint32_t offset;
    uint32_t bits2BeDec;
    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t posState = CMPTLZ_CALC_POS_STATE(decCtx->processedPos, pbMask);
    const unsigned char *bufLimit = *pbufLimit;
    CmptLzDecProb *probBit;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    CmptLzDecProb *probLen = probSlot + CMPTLZ_LEN_CHOICE;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen);
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET + posState;
        bits2BeDec = 3; // need decode 3 bits
        offset = 0;
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

        probLen = probSlot + CMPTLZ_LEN_CHOICE2;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen);
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE + CMPTLZ_LEN_CHOICE2 + posState;
            bits2BeDec = 3;  // need decode 3 bits
            offset = (CMPTLZ_LOW_LEN_CLASS << 1);
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET;
            bits2BeDec = 8;  // need decode 8 bits
            offset = (CMPTLZ_LOW_LEN_CLASS << 1);
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    /* decSym here means length to be decode */
    uint32_t decSym = 1;
    do {
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    } while (decSym < ((uint32_t)1 << bits2BeDec));
    decSym -= ((uint32_t)1 << bits2BeDec);
    decSym += offset;

    if (mkState >= 4) { // >=4 means the packet dont need decode distance
        *pbufLimit = bufTryDec;
        return CMPT_OK;
    }

    probSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decSym);
    /* decSym here means PosSlot to be decode */
    decSym = 1;
    do {
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    } while (decSym < (1 << CMPTLZ_POS_SLOT_BITS));
    decSym -= (1 << CMPTLZ_POS_SLOT_BITS);

    bits2BeDec = ((decSym >> 1) - 1);
    if (decSym >= CMPTLZ_LOW_POSSLOT) {
        if (decSym < CMPTLZ_HIGH_POSSLOT) {
            probSlot = CmptLzGetSpecPosProb(probsMatrix) + (CmptLzGetBaseDistByPosSlot(decSym) << bits2BeDec);
        } else {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS;
            do {
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1); // right shift 31 to get maganificant bit
            } while (--bits2BeDec);
            probSlot = CmptLzGetAilgnProb(probsMatrix);
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS;
        }

        /* decSym here means distance to be decode */
        decSym = 1;
        offset = 1;
        do {
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probBit);
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
        } while (--bits2BeDec);
    }

    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK;
}

static ALWAYS_INLINE int CmptLzTryDecLitPacket(CmptLzDecCtx *decCtx, uint32_t range, uint32_t rangeCode,
    uint32_t rangeBound, const unsigned char *bufTryDec, const unsigned char **pbufLimit)
{
    CmptLzDecProb *probBit;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    uint32_t procPos = decCtx->processedPos;
    uint32_t litPosMask = ((uint32_t)0x100 << decCtx->prop.litPos) - ((uint32_t)0x100 >> decCtx->prop.litCtx);

    size_t dictBufSize = decCtx->dictBufSize;
    size_t dicPos = decCtx->dictPos;
    const unsigned char *dict = decCtx->dict;
    const unsigned char *bufLimit = *pbufLimit;

    if (decCtx->dictPos >= decCtx->dictBufSize) {
        return CMPT_ERROR_DATA;
    }

    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0 || decCtx->checkDicSize != 0) {
        probSlot += (uint32_t)3 * ((((procPos << 8) + // 3 and 8 means prob slot get
            dict[(dicPos == 0 ? dictBufSize : dicPos) - 1]) & litPosMask) << decCtx->prop.litCtx);
    }
    /* decSym means the literal */
    uint32_t decSym = 1;
    if (decCtx->state < CMPTLZ_LIT_STATES) {
        do {
            probBit = probSlot + decSym;
            CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        } while (decSym < 0x100);
    } else {
        uint32_t bit;
        uint32_t matchSym = dict[dicPos - decCtx->reps[0] + ((dicPos < decCtx->reps[0]) ? dictBufSize : 0)];
        uint32_t offset = 0x100;
        do {
            matchSym <<= 1;
            bit = offset;
            offset &= matchSym;
            probBit = probSlot + (offset + bit + decSym);
            CMPTLZ_MATCH_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        } while (decSym < 0x100);
    }

    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK;
}

/* unsigned char **pbufLimit
   In: Limit of total tempBuf array
   Out: BufLimit of length needed to decode one packet */
int CmptLzTryDecOnePacket(CmptLzDecCtx *decCtx, const unsigned char *bufTryDec, const unsigned char **pbufLimit)
{
    uint32_t rangeBound = 0;
    uint32_t range = decCtx->range;
    uint32_t rangeCode = decCtx->code;
    uint32_t mkState = decCtx->state;
    const unsigned char *bufLimit = *pbufLimit;

    CmptLzDecProb *probSlot;
    CmptLzDecProb *probSlot1;
    CmptLzDecProb *probSlot2;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t posState = CMPTLZ_CALC_POS_STATE(decCtx->processedPos, pbMask);

    probSlot1 = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot1);
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx, range, rangeCode, rangeBound, bufTryDec, pbufLimit);
    }
    /* code below means the packet is match* */
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

    probSlot2 = CmptLzGetIsRepProb(probsMatrix) + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot2);
    if (rangeCode < rangeBound) {
        /* this packet is match, try to decode len and dist later
            probSlot here decides the prob of match len */
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
        mkState = 0;
    } else {
        if (decCtx->dictPos >= decCtx->dictBufSize) {
            return CMPT_ERROR_DATA;
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

        /* this packet is rep, try to decode its type and later decode its len */
        probSlot = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

            probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound); // decode a short Rep
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
                *pbufLimit = bufTryDec;
                return CMPT_OK;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound); // decode a Rep0 packet
            }
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

            probSlot = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound); // decode a Rep1 packet
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);

                probSlot = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
                if (rangeCode < rangeBound) {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound); // decode a Rep2 packet
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound); // decode a Rep3 packet
                }
            }
        }
        /* probSlot here decides the prob of rep len */
        probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
        mkState = CMPTLZ_MKSTATE_NUM;
    }
    return CmptLzTryDecLenAndDist(
        decCtx, mkState, range, rangeCode, rangeBound, probSlot, bufTryDec, pbufLimit);
}

int CmptLzDecCarefulProcess(
    CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit)
{
    int res = CMPT_OK;
    uint32_t remainLen;
    const unsigned char *bufLimitTmp;
    const unsigned char *pSrcIn;

    /* do not stop trying decode packet until we meet EOF or srcIn is not enough */
    do {
        bufLimitTmp = bufLimit;
        pSrcIn = decCtx->buf;
        /* source string may be not long enough to dec, so we try to decode packet before truely do it.
            we decode string as much as possible, if it is not long enougth, return EOF symbol. */
        res = CmptLzTryDecOnePacket(decCtx, pSrcIn, &bufLimitTmp);
        if (res == CMPTLZ_DEC_INPUT_EOF) {
            break;
        }
        res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimitTmp);
        if ((res != CMPT_OK) || (decCtx->buf != bufLimitTmp)) {
            return CMPT_ERROR_DATA;
        }
        if (decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) {
            break;
        }
    } while (decCtx->dictPos < dicPosLimit);

    if ((res == CMPTLZ_DEC_INPUT_EOF) && (decCtx->buf < bufLimit)) {
        remainLen = (uint32_t)(bufLimit - decCtx->buf);
        decCtx->tempBufSize = remainLen;
        for (uint32_t idx = 0; idx < remainLen; idx++) {
            decCtx->tempBuf[idx] = decCtx->buf[idx];
        }
    }

    return CMPT_OK;
}

int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx, size_t dicPosLimit,
    const unsigned char *pSrcIn, size_t srcInLen, size_t *psrcCostLen)
{
    int res;
    size_t lookAheadLen = 0;
    uint32_t newTempBufSize = decCtx->tempBufSize;
    unsigned char *oldTmpBuf = &(decCtx->tempBuf[decCtx->tempBufSize]);

    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX && lookAheadLen < srcInLen) {
        decCtx->tempBuf[newTempBufSize++] = pSrcIn[lookAheadLen++];
    }

    const unsigned char *bufLimit = decCtx->tempBuf + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, &(decCtx->tempBuf[0]), &bufLimit);
    if  (res == CMPTLZ_DEC_INPUT_EOF) {
        *psrcCostLen = lookAheadLen;
        decCtx->tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF;
    }

    if (res == CMPT_ERROR_DATA) {
        return res;
    }

    decCtx->buf = &(decCtx->tempBuf[0]);
    /* bufLimit constrains that the direct process api will only decode one packet */
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if ((res != CMPT_OK) || (bufLimit != decCtx->buf) || (bufLimit <= oldTmpBuf)) {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA;
    }
    *psrcCostLen = (size_t)(bufLimit - oldTmpBuf);
    decCtx->tempBufSize = 0;
    return res;
}

