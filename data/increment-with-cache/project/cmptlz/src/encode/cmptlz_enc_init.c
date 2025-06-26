/**
 * @file cmptlz_enc_init.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩初始化文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_inner.h"
#include "cmptlz_enc_price.h"
#include "cmptlz_enc_rc.h"

/* dictSize */
#define CMPTLZ_MIN_DICTSIZE (1024)
#define CMPTLZ_MAX_DICTSIZE (128 * 1024 * 1024)

/* level */
#define CMPTLZ_SET_DICTSIZE_BY_LEVEL(level, dictSize) \
    do { \
        dictSize = (level <= 5) ? (1 << (level * 2 + 14)) : \
                                            ((level <= 7) ? (1 << 25) : \
                                                            (1 << 26)); \
    } while (0)

#define CMPTLZ_SET_FB_BY_LEVEL(level, fastBytes) (fastBytes = ((level < 7) ? 32 : 64))

CMPTLZ_HIDDEN void CmptlzParamNormalize(CmptlzEncParam *props)
{
    /* level */
    int level = props->level;
    if (level < 0 || level > 9) { // 等级禁止小于0大于9
        level = 5; // 等级5
    }
    props->level = level;

    /* 不合理则由等级设定，合理则可被用户改写 */
    if (props->dictSize < CMPTLZ_MIN_DICTSIZE || props->dictSize > CMPTLZ_MAX_DICTSIZE) {
        CMPTLZ_SET_DICTSIZE_BY_LEVEL(level, props->dictSize);
    }
    if (props->fastBytes < 5 || props->fastBytes > CMPT_MF_LONGEST_MATCH) { // 不可小于5
        CMPTLZ_SET_FB_BY_LEVEL(level, props->fastBytes);
    }
    /* 允许用户合理改写lc,lp,pb */
    if (props->litCtx < 0 || props->litCtx > CMPTLZ_LC_MAX) {
        props->litCtx = 3; // 默认3
    }
    if (props->litPos < 0 || props->litPos > CMPTLZ_LP_MAX) {
        props->litPos = 0; // 默认0
    }
    if (props->posBits < 0 || props->posBits > CMPTLZ_PB_MAX) {
        props->posBits = 2; // 默认2
    }

    props->numThreads = 1;
}

CMPTLZ_HIDDEN void CmptlzSetParam(CmptLzEncCtx *encCtx, const CmptlzEncParam *props)
{
    CmptlzEncParam param = *props;
    // 对用户传入参数做规范化
    CmptlzParamNormalize(&param);
    // 传给encCtx
    encCtx->dicSize = param.dictSize;
    encCtx->numFastBytes = param.fastBytes;
    encCtx->litCtx = param.litCtx;
    encCtx->litPos = param.litPos;
    encCtx->posBits = param.posBits;
    uint32_t i;
    for (i = 7; i < 32; i++) { // 从7遍历到32
        if (encCtx->dicSize <= (uint32_t)(1 << i)) {
            break;
        }
    }
    encCtx->distTableSize = i * 2; // 2倍index
}

CMPTLZ_HIDDEN void CmptlzPriceInit(CmptLzEncCtx *encCtx)
{
    CmptPriceGenRootTable(encCtx);
    CmptPriceGenDistTable(encCtx);
    CmptPriceGenAlignTable(encCtx);
}

CMPTLZ_HIDDEN void CmptlzEncPrepare(CmptLzEncCtx *encCtx)
{
    uint32_t i; // 外循环下标
    uint32_t j; // 内循环下标
    /* block */
    encCtx->encNeedFinish = false;
    encCtx->cmptlzResponse = 0;
    encCtx->nowpos64 = 0;
    /* process */
    encCtx->state = 0;
    encCtx->pbMask = (1 << encCtx->posBits) - 1;
    encCtx->lpMask = ((uint32_t)0x100 << encCtx->litPos) - ((uint32_t)0x100 >> encCtx->litCtx);
    encCtx->posMask = (1 << encCtx->posBits) - 1;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++) {
        encCtx->reps[i] = 0; // cmptlz-rep更新完全与2一致
    }
    /* dp */
    encCtx->optsCurIndex = 0;
    encCtx->optEndIndex = 0;
    for (i = 0; i < CMPT_DP_OPTMAX; i++) {
        encCtx->opts[i].price = CMPT_INFINITY_PRICE;
    }
    /* prefix prob */
    for (i = 0; i < CMPTLZ_NUM_STATES; i++) {
        for (j = 0; j < CMPTLZ_NUM_PB_STATES_MAX; j++) {
            encCtx->isMatch[i][j] = CMPTLZ_PROB_INIT;
            encCtx->isRep0Long[i][j] = CMPTLZ_PROB_INIT;
        }
        encCtx->isRep[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG0[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG1[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG2[i] = CMPTLZ_PROB_INIT;
    }
    /* dist prob */
    for (i = 0; i < CMPTLZ_DIST_STATE_TOTAL; i++) {
        for (j = 0; j < (1 << CMPTLZ_DIST_SLOT_BITS); j++) {
            encCtx->probDistSlot[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    for (i = 0; i < CMPT_DIST_LIMIT_2; i++) {
        encCtx->probDistSpecial[i] = CMPTLZ_PROB_INIT;
    }
    for (i = 0; i < (1 << CMPTLZ_ALIGN_BITS); i++) {
        encCtx->probAlign[i] = CMPTLZ_PROB_INIT;
    }
    /* lit prob */
    encCtx->litMarcov.lcBits = encCtx->litCtx;
    encCtx->litMarcov.posMask = (1U << encCtx->litPos) - 1;

    for (i = 0; i < (1 << CMPTLZ_LCLP_MAX); i++) {
        for (j = 0; j < CMPTLZ_LIT_MAX_SIZE; j++) {
            encCtx->litMarcov.literal[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    /* len prob */
    for (i = 0; i < (1 << CMPT_LEN_HIGH_BITS); i++) {
        encCtx->matchLenEncoder.high[i] = CMPTLZ_PROB_INIT;
        encCtx->repLenEncoder.high[i] = CMPTLZ_PROB_INIT;
        encCtx->matchLenEncoder.low[i] = CMPTLZ_PROB_INIT;
        encCtx->repLenEncoder.low[i] = CMPTLZ_PROB_INIT;
    }
    /* other price */
    CmptlzPriceInit(encCtx); // 先把代价根表建立，再去建其他表
    /* len price */
    encCtx->repLenEncoder.tableSize = encCtx->numFastBytes - 1;
    encCtx->matchLenEncoder.tableSize = encCtx->numFastBytes - 1;
    CmptPriceGenLenTable(encCtx, &encCtx->matchLenEncoder);
    CmptPriceGenLenTable(encCtx, &encCtx->repLenEncoder);
}

CMPTLZ_HIDDEN void *CmptInitCctx(CmptLzMemHook *alloc, int writeEndMark)
{
    void *handle = alloc->CmptLzAlloc(CMPTLZ_ENC_CCTX_HANDLE, sizeof(CmptLzEncCtx));
    if (handle == NULL) {
        return NULL;
    }
    memset_s(handle, sizeof(CmptLzEncCtx), 0, sizeof(CmptLzEncCtx));
    CmptLzEncCtx *encCtx = (CmptLzEncCtx *)handle;
    encCtx->endMarker = writeEndMark;
    encCtx->rcCtx = NULL;
    encCtx->mfCtx = NULL;
    return encCtx;
}