/**
 * @file cmptlz_dec_process
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压核心流程文件
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

#define CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec) \
    do { \
        if ((range) < CMPTLZ_RANGE_DOWN_LIMIT) { \
            (range) <<= CMPTLZ_ONE_BYTE_WIDTH; \
            (rangeCode) <<= CMPTLZ_ONE_BYTE_WIDTH; \
            (rangeCode) |= (*(bufToDec)++); \
        } \
    } while (0)

#define CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) \
    (rangeBound) = (range >> CMPTLZ_PROB_LG_BIT) * (*(probSlot)); \
    if ((rangeCode) < (rangeBound)) \

/* 解码出0后对区间范围和对应位置的Prob进行更新 */
#define CMPTLZ_RANGE_UPDATE_0(prob, range, rangeBound) \
    do { \
        (range) = (rangeBound); \
        *(prob) = (CmptLzDecProb)((*(prob)) + ((CMPTLZ_PROB_LG - (*(prob))) >> CMPTLZ_RANGE_CODE_SIZE)); \
    } while (0)

/* 解码出1后对区间范围和对应位置的Prob进行更新 */
#define CMPTLZ_RANGE_UPDATE_1(prob, range, rangeCode, rangeBound) \
    do { \
        (range) -= (rangeBound); \
        (rangeCode) -= (rangeBound); \
        *(prob) = (CmptLzDecProb)((*(prob)) - ((*(prob)) >> CMPTLZ_RANGE_CODE_SIZE)); \
    } while (0)

/* 从最高位编码到低位 */
#define CMPTLZ_NORMAL_BIT_DEC(probLit, range, rangeCode, rangeBound, decSym) \
    do { \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probLit)); \
        if ((rangeCode) < (rangeBound)) { \
            CMPTLZ_RANGE_UPDATE_0(probLit, range, rangeBound); \
            (decSym) = ((decSym) << 1); \
        } else { \
            CMPTLZ_RANGE_UPDATE_1(probLit, range, rangeCode, rangeBound); \
            (decSym) = ((decSym) << 1) + 1; \
        } \
    } while (0)

#define CMPTLZ_MATCH_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec) \
    do { \
        (matchSym) <<= 1; \
        (bit) = (offset); \
        (offset) &= (matchSym); \
        (probLit) = (probSlot) + ((offset) + (bit) + (decSym)); \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probLit)); \
        if ((rangeCode) < (rangeBound)) { \
            CMPTLZ_RANGE_UPDATE_0(probLit, range, rangeBound); \
            (decSym) = ((decSym) << 1); \
            (offset) ^= (bit); \
        } else { \
            CMPTLZ_RANGE_UPDATE_1(probLit, range, rangeCode, rangeBound); \
            (decSym) = ((decSym) << 1) + 1; \
        } \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec); \
    } while (0)

#define CMPTLZ_DIST_BIT_DEC(probDist, probSlot, range, rangeCode, rangeBound, decDist, decBit) \
    do { \
        (probDist) = (probSlot) + (decDist); \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probDist)); \
        if ((rangeCode) < (rangeBound)) { \
            CMPTLZ_RANGE_UPDATE_0(probDist, range, rangeBound); \
            (decDist) += (decBit); \
        } else { \
            CMPTLZ_RANGE_UPDATE_1(probDist, range, rangeCode, rangeBound); \
            (decDist) += (decBit) * 2; \
        } \
    } while (0)

#define CMPTLZ_LEN_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, bufToDec) \
    do { \
        CMPTLZ_NORMAL_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym); \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec); \
    } while (0)

#define CMPTLZ_POSSLOT_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, bufToDec) \
    do { \
        CMPTLZ_NORMAL_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym); \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec); \
    } while (0)

#define CMPTLZ_REP4 4
#define CMPTLZ_REP3 3
#define CMPTLZ_REP2 2

static ALWAYS_INLINE void CmptLzDistDecHelper(CmptLzDecCtx *decCtx, uint32_t distDec, const unsigned char *bufToDec,
    uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound,
    uint32_t range, uint32_t rangeCode, uint32_t rangeBound)
{
    decCtx->reps[CMPTLZ_REP3] = decCtx->reps[CMPTLZ_REP2];
    decCtx->reps[CMPTLZ_REP2] = decCtx->reps[1];
    decCtx->reps[1] = decCtx->reps[0];
    decCtx->reps[0] = (distDec + 1);

    decCtx->buf = bufToDec;
    decCtx->state = (decCtx->state < CMPTLZ_LIT_STATES) ? CMPTLZ_LIT_STATES : CMPTLZ_LIT_STATES + CMPTLZ_REP3;
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
}

static size_t CmptLzDistDec(CmptLzDecCtx *decCtx,
    CmptLzDecProb *probsMatrix, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound, uint32_t decLen)
{
    uint32_t assistBits;
    uint32_t posSlot = 1;
    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;
    const unsigned char *bufToDec = decCtx->buf;
    uint32_t distDec;

    CmptLzDecProb *probPosSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decLen);
    /* 先将6个Bit的posSlot进行解码 */
    int i = 0;
    for (i = 0; i < CMPTLZ_POS_SLOT_BITS; i++) {
        CMPTLZ_POSSLOT_BIT_DEC((probPosSlot + posSlot), range, rangeCode, rangeBound, posSlot, bufToDec);
    }
    posSlot -= 64; // posSlot值更新为posSlot - 64

    if (posSlot < CMPTLZ_LOW_POSSLOT) { // posSlot区间号小于4的情况下，匹配距离和区间号是相等的
        distDec = posSlot;
        CmptLzDistDecHelper(decCtx, distDec, bufToDec, pRange, pRangeCode, pRangeBound,
            range, rangeCode, rangeBound);

        if (distDec == (size_t)0xFFFFFFFF) {
            return distDec;
        } else {
            return (distDec + 1);
        }
    }

    uint32_t directBitNum = ((posSlot >> 1) - 1); // posSlot共6位，高5位的值表示该数字的最高位Bit值
    distDec = CmptLzGetBaseDistByPosSlot(posSlot);
    if (posSlot < CMPTLZ_HIGH_POSSLOT) {
        assistBits = 1;
        distDec <<= directBitNum;
        // 低directBitNum位Bit需要解码填充
        distDec += assistBits;
        probPosSlot = CmptLzGetSpecPosProb(probsMatrix);
        do {
            /* tricky:假设此轮解码低第K位Bit的值，显然此时tmpDecDist(值为1<<K)的最高位也是K,所以如果解码出0,则直接加上assistBits,
                效果为第K位向高位进1,baseDist最高位变为K+1,同时K位变为0,由于是逆序编码，相当于此时解码出一个0.之后tmpDecDist左移1位以进行下一轮解码 */
            if CMPTLZ_IS_THE_BIT_0((probPosSlot + distDec), range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0((probPosSlot + distDec), range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                distDec += assistBits;
                assistBits <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_1((probPosSlot + distDec), range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                assistBits <<= 1;
                distDec += assistBits;
            }
        } while (--directBitNum);
        distDec -= assistBits; // 上述规则会导致解码的距离多tmpDecDist需要减去
    } else {
    directBitNum -= CMPTLZ_REP4;
    do {
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        range >>= 1;
        rangeCode -= range;
        assistBits = (0 - ((uint32_t)rangeCode >> 31)); // 将rangeCode右移31位
        distDec = (distDec << 1) + (assistBits + 1);
        rangeCode += range & assistBits;
    } while (--directBitNum);

    CmptLzDecProb *probDist;
    probPosSlot = CmptLzGetAilgnProb(probsMatrix);

    distDec <<= CMPTLZ_LARGE_DIST_LOW_BITS;
    assistBits = 1;

    uint32_t cycleSym = 1;
    for (i = 0; i < 3; i++) { // 3位解压
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec); // Line302退出循环后需要刷新一次range
        CMPTLZ_DIST_BIT_DEC(probDist, probPosSlot, range, rangeCode, rangeBound, assistBits, cycleSym);
        cycleSym <<= 1;
    }
    CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
    probDist = probPosSlot + assistBits;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probDist);
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_0(probDist, range, rangeBound);
        assistBits -= 8; // 解码后需要减去初始值8
    } else {
        CMPTLZ_RANGE_UPDATE_1(probDist, range, rangeCode, rangeBound);
    }
    CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
    distDec |= assistBits;
    }

    CmptLzDistDecHelper(decCtx, distDec, bufToDec, pRange, pRangeCode, pRangeBound,
        range, rangeCode, rangeBound);

    if (distDec == (size_t)0xFFFFFFFF) {
        return distDec;
    } else {
        return (distDec + 1);
    }
}

/* =====================      =======================                                 =======================
 * | state为0且len小于8 |     | state为0,8<=len<=16 |    每种state需要16个槽位解码     | state为Max且len小于8 |
 * | 的prob表共8个槽位  |     | 的prob表,共8个槽位   |    .......................      | 的prob表共8个槽位    | ....
 * =====================     ========================                                 =======================*/
static uint32_t CmptLzLenDec(CmptLzDecCtx *decCtx,
    CmptLzDecProb *probSlot, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound, uint32_t posState)
{
    uint32_t decLen = 1;
    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;
    const unsigned char *bufToDec = decCtx->buf;
    CmptLzDecProb *probLen = probSlot + CMPTLZ_LEN_CHOICE;  /* rep-packet或者match-packet解码len使用的probSlot是不同的 */

    int i = 0;
    if CMPTLZ_IS_THE_BIT_0(probLen, range, rangeCode, rangeBound) {
        CMPTLZ_RANGE_UPDATE_0(probLen, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET + posState;
        for (i = 0; i < CMPTLZ_LOW_LEN_BIT; i++) {
            CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
        }
        decLen -= 8; // 解码后需要减去初始值8
    } else {
        CMPTLZ_RANGE_UPDATE_1(probLen, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

        probLen = probSlot + CMPTLZ_LEN_CHOICE2;
        if CMPTLZ_IS_THE_BIT_0(probLen, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0(probLen, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

            probLen = probSlot + (CMPTLZ_LEN_CHOICE2 + posState);
            for (i = 0; i < CMPTLZ_LOW_LEN_BIT; i++) {
                CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            }
        } else {
            CMPTLZ_RANGE_UPDATE_1(probLen, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET;
            for (i = 0; i < CMPTLZ_HIGH_LEN_BIT; i++) {
                CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            }
            decLen -= CMPTLZ_HIGH_LEN_CLASS;
            decLen += (CMPTLZ_LOW_LEN_CLASS << 1);
        }
    }

    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx->buf = bufToDec;

    return decLen;
}

static uint32_t CmptLzDecByDistAndLen(
    CmptLzDecCtx *decCtx, size_t matchDist, uint32_t matchLen, size_t dicPosLimit)
{
    size_t dicCopyPos;
    size_t dicPos = decCtx->dictPos;
    size_t dictBufSize = decCtx->dictBufSize;
    uint32_t remainDicLen = (uint32_t)(dicPosLimit - dicPos);
    unsigned char *dict = decCtx->dict;

    if (remainDicLen == 0) {
        return CMPT_ERROR_DATA;
    }

    uint32_t decDicLen = ((remainDicLen < matchLen) ? remainDicLen : matchLen); // 此次最多解压decDicLen那么多的字节
    decCtx->processedPos += decDicLen;
    decCtx->dictPos += decDicLen;
    decCtx->remainLen = matchLen - decDicLen;

    /* 找到match位置准备拷贝 */
    if (dicPos < matchDist) {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    } else {
        dicCopyPos = dicPos - matchDist;
    }

    do {
        dict[dicPos++] = dict[dicCopyPos];
        if (++dicCopyPos == dictBufSize) {
            dicCopyPos = 0;
        }
    } while (--decDicLen != 0);

    return CMPT_OK;
}


static ALWAYS_INLINE void CmptLzShortRepDec(CmptLzDecCtx *decCtx)
{
    uint32_t rep0 = decCtx->reps[0];
    unsigned char *dict = decCtx->dict;
    size_t dictPos = decCtx->dictPos;
    size_t dictBufSize = decCtx->dictBufSize;

    dict[dictPos] = dict[dictPos - rep0 + (dictPos < rep0 ? dictBufSize : 0)];
    decCtx->dictPos++;
    decCtx->processedPos++;
    if (decCtx->state < CMPTLZ_LIT_STATES) {
        decCtx->state = 9; // 9表示前一个packet是Lit当前packet是Rep类型
    } else {
        decCtx->state = 11; // 11表示前一个packet是Match当前Patch是Rep类型
    }
}

static ALWAYS_INLINE uint32_t CmptLzRepDec(CmptLzDecCtx *decCtx, uint32_t *pRange,
    uint32_t *pRangeCode, uint32_t *pRangeBound, size_t dicPosLimit, uint32_t posState)
{
    uint32_t repLen;
    uint32_t repDist;
    uint32_t mkState = decCtx->state;
    const unsigned char *bufToDec = decCtx->buf;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;

    probSlot = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
    if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) {
        CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

        probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            *pRange = range;
            *pRangeCode = rangeCode;
            *pRangeBound = rangeBound;
            decCtx->buf = bufToDec;
            CmptLzShortRepDec(decCtx);
            return CMPT_OK;
        } else {
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            repDist = decCtx->reps[0];
        }
    } else {
        CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

        probSlot = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            repDist = decCtx->reps[1];
        } else {
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);

            probSlot = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
            if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                repDist = decCtx->reps[CMPTLZ_REP2];
            } else {
                CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                repDist = decCtx->reps[CMPTLZ_REP3];
                decCtx->reps[CMPTLZ_REP3] = decCtx->reps[CMPTLZ_REP2];
            }
            decCtx->reps[CMPTLZ_REP2] = decCtx->reps[1];
        }
        decCtx->reps[1] = decCtx->reps[0];
        decCtx->reps[0] = repDist;
    }

    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx->buf = bufToDec;

    decCtx->state = (mkState < CMPTLZ_LIT_STATES) ? 8 : 11; // 8和11表示状态机的状态
    probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
    repLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);

    return CmptLzDecByDistAndLen(decCtx, repDist, repLen + 2, dicPosLimit); // 2表示解码后距需要加上初始值2
}

// Match的Packet需要解码Len和Dist
static ALWAYS_INLINE uint32_t CmptLzMatchDec(CmptLzDecCtx *decCtx, uint32_t *pRange, uint32_t *pRangeCode,
    uint32_t *pRangeBound, size_t dicPosLimit, uint32_t posState)
{
    uint32_t matchLen;
    size_t matchDist;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
    matchLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    matchDist = CmptLzDistDec(decCtx, probsMatrix, pRange, pRangeCode, pRangeBound, matchLen);
    if (matchDist > decCtx->dictBufSize) {
        if (matchDist == (size_t)0xFFFFFFFF) {
            decCtx->remainLen = CMPTLZ_MATCH_MAX_LEN;
            decCtx->state -= CMPTLZ_MKSTATE_NUM;
            return CMPT_OK;
        } else {
            return CMPT_ERROR_DATA;
        }
    }
    return CmptLzDecByDistAndLen(decCtx, matchDist, matchLen + 2, dicPosLimit); // 2表示解码后距需要加上初始值2
}

static ALWAYS_INLINE uint32_t CmptLzLitDec(
    CmptLzDecCtx *decCtx, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound)
{
    uint32_t decSym = 1;
    uint32_t mkState = decCtx->state;
    uint32_t procPos = decCtx->processedPos;
    uint32_t checkDicSize = decCtx->checkDicSize;
    uint32_t litCtx = decCtx->prop.litCtx;
    uint32_t litPosMask = ((uint32_t)0x100 << decCtx->prop.litPos) - ((uint32_t)0x100 >> litCtx);

    CmptLzDecProb *probLit;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    const unsigned char *bufToDec = decCtx->buf;
    unsigned char *dict = decCtx->dict;
    size_t dictBufSize = decCtx->dictBufSize;
    size_t dictPos = decCtx->dictPos;

    uint32_t range = *pRange;
    uint32_t rangeBound = *pRangeBound;
    uint32_t rangeCode = *pRangeCode;

    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0 || checkDicSize != 0) {
        probSlot += (uint32_t)CMPTLZ_REP3 * ((((procPos << 8) + // 左移8位
        dict[(dictPos == 0 ? dictBufSize : dictPos) - 1]) & litPosMask) << litCtx);
    }

    int i = 0;
    if (mkState < CMPTLZ_LIT_STATES) {
        mkState -= (mkState < 4) ? mkState : 3; // 4和3表示状态机的下一个状态迁移
        for (i = 0; i < 8; i++) {
            CMPTLZ_NORMAL_BIT_DEC((probSlot + decSym), range, rangeCode, rangeBound, decSym);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        }
    } else {
        uint32_t bit;
        uint32_t offset = 0x100;
        uint32_t rep0 = decCtx->reps[0];
        uint32_t matchSym = dict[dictPos - rep0 + ((dictPos < rep0) ? dictBufSize : 0)];
        mkState -= (mkState < 10) ? CMPTLZ_REP3 : 6; // 6和10表示状态机的状态
        for (i = 0; i < 8; i++) {
            CMPTLZ_MATCH_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec);
        }
    }

    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;

    dict[dictPos++] = (uint8_t)decSym;
    decCtx->processedPos += 1;
    decCtx->state = mkState;
    decCtx->dictPos = dictPos;
    decCtx->buf = bufToDec;

    return CMPT_OK;
}

int CmptLzDecDirectProcess(
    CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit)
{
    uint32_t decRes;
    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t procPos;
    uint32_t mkState;
    uint32_t posState;

    uint32_t range = decCtx->range;
    uint32_t rangeCode = decCtx->code;
    uint32_t rangeBound = 0;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);

    do {
        procPos = decCtx->processedPos;
        mkState = decCtx->state;
        posState = CMPTLZ_CALC_POS_STATE(procPos, pbMask);
        probSlot = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;

        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) { // first bit of packet is 0, must be lit packet
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
            decRes = CmptLzLitDec(decCtx, &range, &rangeCode, &rangeBound);
        } else {  // fist bit of packet is 1, must be match* packet
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);

            probSlot = CmptLzGetIsRepProb(probsMatrix) + mkState;
            // second bit of packet is 0, must be match packet
            if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
                decRes = CmptLzMatchDec(decCtx, &range, &rangeCode, &rangeBound, dicPosLimit, posState);
            } else {  // second bit of packet is 1, must be rep* packet
                CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
                decRes = CmptLzRepDec(decCtx, &range, &rangeCode, &rangeBound, dicPosLimit, posState);
            }
            if (decRes != CMPT_OK) {
                break;
            }
        }
    } while (decCtx->dictPos < dicPosLimit && decCtx->buf < bufLimit && decCtx->remainLen < CMPTLZ_MATCH_MAX_LEN);

    decCtx->range = range;
    decCtx->code = rangeCode;

    return (int)decRes;
}