/**
 * @file cmptlz_enc_struct.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩结构体头文件
 * @author Anonym
 * @date 2024-01-09
 */

#ifndef CMPTLZ_ENC_STRUCT_H
#define CMPTLZ_ENC_STRUCT_H

#include "cmptlz_enc.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef uint16_t CmptlzProb;

#define CMPTLZ_NUM_REPS 4
#define CMPTLZ_NUM_STATES 12
#define CMPTLZ_MATCH_LEN_MIN 2

#define CMPTLZ_PB_MAX 4
#define CMPTLZ_LC_MAX 8
#define CMPTLZ_LP_MAX 4
#define CMPTLZ_LCLP_MAX 4
#define CMPTLZ_NUM_PB_STATES_MAX (1 << CMPTLZ_PB_MAX)
#define CMPTLZ_LIT_MAX_SIZE 0x300
#define CMPTLZ_PROB_MAX_NUM 2048
#define CMPTLZ_PROB_INIT 1024
#define CMPTLZ_RC_BUFFER_SIZE (1 << 16)

/* dist */
#define CMPT_DIST_LIMIT_1 4
#define CMPT_DIST_LIMIT_2 128
#define CMPTLZ_DIST_STATE_TOTAL 4
#define CMPTLZ_ALIGN_BITS 4
#define CMPTLZ_DIST_SLOT_BITS 6

/* price */
#define CMPT_INFINITY_PRICE ((uint32_t)1 << 30)
// #define CMPTLZ_DIST_STATE_TOTAL 4
// #define CMPTLZ_ALIGN_BITS 4
// #define CMPTLZ_DIST_SLOT_BITS 6
#define CMPT_PRICE_BITS_MOVING_NUM 4
#define CMPT_PRIICE_TABLE_SIZE (CMPTLZ_PROB_MAX_NUM >> CMPT_PRICE_BITS_MOVING_NUM)
#define CMPT_PRICE_COUNT 64
/* len */
#define CMPT_DOUBLE 2
#define CMPT_LEN_LOW_BITS 3
#define CMPT_LEN_MID_BITS 3
#define CMPT_LEN_HIGH_BITS 8
#define CMPT_LEN_BOUND 8
/* match finder */
#define CMPT_MF_LONGEST_MATCH 273
#define CMPTLZ_MEM_COMPARE_SAFE_DIST 8
#define CMPT_MF_LONGEST_MATCH_SAFE (CMPT_MF_LONGEST_MATCH + CMPTLZ_MEM_COMPARE_SAFE_DIST)
#define CMPT_MF_HASH_TABLE_SIZE 256
#define CMPT_MF_BASE_DEPTH 16
#define CMPT_MF_MATCH_2_BYTES 2
#define CMPT_MF_MATCH_3_BYTES 3
/* dp */
#define CMPT_DP_OPTMAX (1 << 11)
/* block */
#define CMPT_ONE_BLOCK_MAX_SIZE (1 << 17)

typedef struct {
    int level;
    uint32_t dictSize;
    int litCtx;
    int litPos;
    int posBits;
    int fastBytes;
    int numThreads;
} CmptlzEncParam;

typedef struct {
    uint32_t pos;
    uint32_t prevByte;
    CmptlzProb literal[1 << CMPTLZ_LCLP_MAX][CMPTLZ_LIT_MAX_SIZE];
    uint32_t lcBits; // The number of literal context bits (high bits of previous literal)
    uint32_t posMask; // Literal pos Mask != encCtx pos Mask
} LitMarcov;

typedef struct {
    uint32_t range;
    uint64_t cache;
    uint64_t low;
    uint64_t cacheSize;
    uint8_t *buf;
    uint8_t *bufBase;
    uint8_t *outBuf;
    size_t outBufLeft;
} CmptRcCtx;

typedef struct TagCmptMatchFinder CmptMfCtx;
struct TagCmptMatchFinder {
    /* input */
    const uint8_t *srcStart;
    size_t srcLen;
    /* dp */
    uint32_t hashRootTable[256]; // 如果对内存有要求，可以把哈希值表拿出去
    uint32_t mfStart;
    uint32_t niceLen;
    uint32_t readAhead;
    uint32_t readPos;
    uint32_t cyclePos;
    uint32_t cycleSize;
    uint32_t offset;
    uint32_t *hash;
    uint32_t *son;
    uint32_t depth;
    uint32_t hashCount;
    uint32_t sonsCount;
    uint32_t hashMask;
};

typedef struct {
    CmptlzProb low[256];
    CmptlzProb high[1 << CMPT_LEN_HIGH_BITS];
    uint32_t prices[CMPTLZ_NUM_PB_STATES_MAX][(1 << CMPT_LEN_HIGH_BITS) +
        (1 << CMPT_LEN_MID_BITS) + (1 << CMPT_LEN_LOW_BITS)];
    uint32_t tableSize;
} CmptLenEncoder;

typedef enum {
    LIT_LIT,
    MATCH_LIT_LIT,
    REP_LIT_LIT,
    SHORTREP_LIT_LIT,
    MATCH_LIT,
    REP_LIT,
    SHORTREP_LIT,
    LIT_MATCH,
    LIT_LONGREP,
    LIT_SHORTREP,
    NOTLIT_MATCH,
    NOTLIT_REP,
} CmptlzState;

typedef struct {
    uint32_t len;
    uint32_t dist;
} CmptlzMatchPair;

typedef struct {
    CmptlzState state;
    uint32_t price;
    uint32_t posPrev;
    uint32_t backPrev;
    uint32_t backs[CMPTLZ_NUM_REPS];
} CmptlzOpt;

/**
 * @ingroup cmptlz
 * @brief cmptlz压缩上下文
 */
struct TagCmptLzEncCtx {
    /* param */
    int level;
    int litCtx;
    int litPos;
    int posBits;
    uint32_t dicSize;
    int endMarker;
    uint32_t numFastBytes;
    /* block */
    bool encNeedFinish;
    uint64_t nowpos64;
    uint32_t cmptlzResponse;
    /* Marcov */
    CmptlzState state;
    LitMarcov litMarcov;
    uint32_t reps[CMPTLZ_NUM_REPS];
    /* prob */
    CmptlzProb isRep[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG0[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG1[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG2[CMPTLZ_NUM_STATES];
    CmptlzProb isMatch[CMPTLZ_NUM_STATES][CMPTLZ_NUM_PB_STATES_MAX];
    CmptlzProb isRep0Long[CMPTLZ_NUM_STATES][CMPTLZ_NUM_PB_STATES_MAX];
    CmptlzProb probDistSlot[CMPTLZ_DIST_STATE_TOTAL][1 << CMPTLZ_DIST_SLOT_BITS];
    CmptlzProb probDistSpecial[CMPT_DIST_LIMIT_2];
    CmptlzProb probAlign[1 << CMPTLZ_ALIGN_BITS];
    /* mask */
    uint32_t posMask;
    uint64_t pbMask;
    uint64_t lpMask;
    /* RC */
    CmptRcCtx *rcCtx;
    /* MF */
    CmptMfCtx *mfCtx;
    CmptlzMatchPair matches[CMPT_MF_LONGEST_MATCH + 1];
    uint32_t matchesCount;
    uint32_t longestMatchLen;
    /* DP */
    uint32_t backRes;
    uint32_t lenRes;
    uint32_t optEndIndex;
    uint32_t optsCurIndex;
    CmptlzOpt opts[CMPT_DP_OPTMAX];
    /* Len */
    CmptLenEncoder matchLenEncoder;
    CmptLenEncoder repLenEncoder;
    int repLenPriceCount;
    /* Price */
    int matchPriceCount;
    uint32_t priceRootTable[CMPT_PRIICE_TABLE_SIZE];
    uint32_t priceDistSlotTable[CMPTLZ_DIST_STATE_TOTAL][1 << CMPTLZ_DIST_SLOT_BITS];
    uint32_t priceDistTable[CMPTLZ_DIST_STATE_TOTAL][1 << 7];
    uint32_t priceAlignTable[1 << CMPTLZ_ALIGN_BITS];
    uint32_t distTableSize;
};

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_ENC_STRUCT_H */