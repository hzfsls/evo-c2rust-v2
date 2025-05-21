/**
 * @file CmptLz.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压功能对外头文件
 * @author Anonym
 * @date 2023-07-04
 * @version v0.1.0
 ********************************************************************************************
 * @par History
 * <table>
 * <tr><th>Date        <th>Version   <th>Author      <th>Description
 * <tr><td>2023-07-04  <td>0.1.0     <td>   <td>Init version
 * </table>
 ********************************************************************************************
 */

#ifndef CMPTLZ_INNER_H
#define CMPTLZ_INNER_H

#include "securec.h"
#include "cmptlz_def.h"
#include "cmptlz_dec.h"
#include "cmptlz_base.h"

#ifdef __cplusplus
extern "C" {
#endif

#define CMPTLZ_DEC_INPUT_EOF 1

#define CMPTLZ_DICT_MIN_LEN (1 << 12)

#define CMPTLZ_RANGE_CODE_SIZE 5
#define CMPTLZ_REQUIRED_INPUT_MAX 20

#define CMPTLZ_MKSTATE_NUM 12
#define CMPTLZ_LIT_STATES 7

#define CMPTLZ_RANGE_DOWN_LIMIT ((uint32_t)1 << 24)
#define CMPTLZ_ONE_BYTE_WIDTH 8
#define CMPTLZ_PROB_LG_BIT 11
#define CMPTLZ_PROB_LG (1 << CMPTLZ_PROB_LG_BIT)

#define CMPTLZ_PB_STATE_NUM_ALIGN 16
#define CMPTLZ_PB_BITS_MAX 4

#define CMPTLZ_MATCH_MAX_LEN 274

#define CMPTLZ_MATCH_LEN_INIT 274

#define CMPTLZ_CALC_POS_STATE(procPos, pbMask) (((procPos) & (pbMask)) << 4)

/* 解码len使用的相关宏 */
#define CMPTLZ_LOW_LEN_BIT 3
#define CMPTLZ_LOW_LEN_CLASS (1 << CMPTLZ_LOW_LEN_BIT) // 8 values bit-tree

#define CMPTLZ_HIGH_LEN_BIT 8
#define CMPTLZ_HIGH_LEN_CLASS (1 << CMPTLZ_HIGH_LEN_BIT) // 256 values bit-tree

#define CMPTLZ_LOW_LENPROB_OFFSET 0
#define CMPTLZ_HIGH_LENPROB_OFFSET (CMPTLZ_LOW_LENPROB_OFFSET + ((1 << CMPTLZ_PB_BITS_MAX) << (CMPTLZ_LOW_LEN_BIT + 1)))

#define CMPTLZ_LEN_CHOICE CMPTLZ_LOW_LENPROB_OFFSET
#define CMPTLZ_LEN_CHOICE2 (CMPTLZ_LEN_CHOICE + CMPTLZ_LOW_LEN_CLASS)
#define CMPTLZ_LENPROB_NUM (CMPTLZ_HIGH_LENPROB_OFFSET + CMPTLZ_HIGH_LEN_CLASS)

/* 解码dist使用的相关宏 */
#define CMPTLZ_LEN_CONDITION_TO_POSSLOT 4  // min(match_length, 5)
#define CMPTLZ_POS_SLOT_BITS 6  // 64 values bit-tree
#define CMPTLZ_LOW_POSSLOT 4
#define CMPTLZ_HIGH_POSSLOT 14
#define CMPTLZ_FULL_DISTANCE (1 << (CMPTLZ_HIGH_POSSLOT >> 1)) // 128

#define CMPTLZ_LARGE_DIST_LOW_BITS 4
#define CMPTLZ_ALIGN_TABLE_SIZE (1 << CMPTLZ_LARGE_DIST_LOW_BITS)

#define CMPTLZ_OFFSET 1664
#define CMPTLZ_SPEC_POS (-CMPTLZ_OFFSET)
#define CMPTLZ_REP0_LONG (CMPTLZ_SPEC_POS + CMPTLZ_FULL_DISTANCE)
#define CMPTLZ_REP_LEN_CODER (CMPTLZ_REP0_LONG + (CMPTLZ_PB_STATE_NUM_ALIGN << CMPTLZ_PB_BITS_MAX))
#define CMPTLZ_MATCH_LEN_CODER (CMPTLZ_REP_LEN_CODER + CMPTLZ_LENPROB_NUM)
#define CMPTLZ_IS_MATCH (CMPTLZ_MATCH_LEN_CODER + CMPTLZ_LENPROB_NUM)
#define CMPTLZ_ALIGN (CMPTLZ_IS_MATCH + (CMPTLZ_PB_STATE_NUM_ALIGN << CMPTLZ_PB_BITS_MAX))
#define CMPTLZ_ISREP (CMPTLZ_ALIGN + CMPTLZ_ALIGN_TABLE_SIZE)
#define CMPTLZ_ISREPG0 (CMPTLZ_ISREP + CMPTLZ_MKSTATE_NUM)
#define CMPTLZ_ISREPG1 (CMPTLZ_ISREPG0 + CMPTLZ_MKSTATE_NUM)
#define CMPTLZ_ISREPG2 (CMPTLZ_ISREPG1 + CMPTLZ_MKSTATE_NUM)
#define CMPTLZ_POSSLOT (CMPTLZ_ISREPG2 + CMPTLZ_MKSTATE_NUM)
#define CMPTLZ_LITERAL (CMPTLZ_POSSLOT + (CMPTLZ_LEN_CONDITION_TO_POSSLOT << CMPTLZ_POS_SLOT_BITS))
#define NUM_BASE_PROBS (CMPTLZ_LITERAL + CMPTLZ_OFFSET)

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetProbsMatrix(CmptLzDecCtx *decCtx)
{
    /* CmptLzGetProbsMatrix */
    return decCtx->probsPlus1664;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsMatchProb(CmptLzDecProb *probsMatrix)
{
    /* CmptLzGetIsMatchProb */
    return probsMatrix + CMPTLZ_IS_MATCH;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsRepProb(CmptLzDecProb *probsMatrix)
{
    /* CmptLzGetIsRepProb */
    return probsMatrix + CMPTLZ_ISREP;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsRepG0Prob(CmptLzDecProb *probsMatrix)
{
    /* CmptLzGetIsRepG0Prob */
    return probsMatrix + CMPTLZ_ISREPG0;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsRepG1Prob(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREPG1;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsRepG2Prob(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREPG2;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetIsRepG0LongProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_REP0_LONG;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetLiteralProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_LITERAL;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetPosSlotProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_POSSLOT;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetSpecPosProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_SPEC_POS;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetAilgnProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ALIGN;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetRepLenCoderProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_REP_LEN_CODER;
}

static ALWAYS_INLINE CmptLzDecProb *CmptLzGetMatchLenCoderProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_MATCH_LEN_CODER;
}

static inline uint32_t CmptLzGetLenCondition(uint32_t decLen)
{
    return ((decLen < CMPTLZ_LEN_CONDITION_TO_POSSLOT ? decLen :
        CMPTLZ_LEN_CONDITION_TO_POSSLOT - 1) << CMPTLZ_POS_SLOT_BITS);
}

static inline uint32_t CmptLzGetBaseDistByPosSlot(uint32_t posSlot)
{
    return (2 | (posSlot & 1)); /* 2表示二进制10 */
}

static inline uint32_t CmptLzGetNumProbs(CmptLzDecProt *decProt)
{
    // 计算概率表大小，0X300表示0X100 << 3 表示共8种状态，每种状态有256个Slot
    return (NUM_BASE_PROBS + ((uint32_t)0x300 << (decProt->litCtx + decProt->litPos)));
}

extern int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx,
    size_t dicPosLimit, const unsigned char *pSrcIn, size_t srcInLen, size_t *psrcCostLen);

extern int CmptLzDecCarefulProcess(
    CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit);

extern int CmptLzDecDirectProcess(
    CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit);

extern int CmptLzDecDecodeToDic(CmptLzDecCtx *decCtx, size_t dicPosLimit,
    const unsigned char *pSrcIn, size_t *pStrInLen, EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_INNER_H */