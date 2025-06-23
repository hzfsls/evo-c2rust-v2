/**
 * @file cmptlz_enc_price.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 代价头文件
 * @author Anonym
 * @date 2024-01-09
 */
#ifndef CMPTLZ_ENC_PRICE_H
#define CMPTLZ_ENC_PRICE_H

#include "cmptlz_enc_inner.h"

#ifdef __cplusplus
extern "C" {
#endif

CMPTLZ_HIDDEN void CmptPriceGenRootTable(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN void CmptPriceGenDistTable(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN void CmptPriceGenAlignTable(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN uint32_t CmptPriceLiteral(CmptLzEncCtx *encCtx, bool matchMode, uint32_t matchByte, uint32_t symbol);
CMPTLZ_HIDDEN uint32_t CmptPriceLen(CmptLenEncoder *lenEncoder, uint32_t len, uint32_t posState);
CMPTLZ_HIDDEN void CmptPriceGenLenTable(CmptLzEncCtx *encCtx, CmptLenEncoder *lenEncoder);
CMPTLZ_HIDDEN uint32_t CmptPriceShortRep(CmptLzEncCtx *encCtx, CmptlzState state, uint32_t posState);
CMPTLZ_HIDDEN uint32_t CmptPriceDistWithLen(CmptLzEncCtx *encCtx, uint32_t dist, uint32_t len, uint32_t posState);
CMPTLZ_HIDDEN uint32_t CmptPriceLongRep(CmptLzEncCtx *encCtx,
    uint32_t longRepIndex, CmptlzState state, uint32_t posState);

static ALWAYS_INLINE uint32_t CmptPriceOneBitDirect(uint32_t bit)
{
    return (bit << CMPT_PRICE_BITS_MOVING_NUM);
}

static ALWAYS_INLINE uint32_t CmptPriceOneBit(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob, uint32_t curbit)
{
    return encCtx->priceRootTable[(bit0Prob ^ ((uint32_t)(0 - curbit) &
        (CMPTLZ_PROB_MAX_NUM - 1))) >> CMPT_PRICE_BITS_MOVING_NUM];
}

static ALWAYS_INLINE uint32_t CmptPriceBit0(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob)
{
    return encCtx->priceRootTable[bit0Prob >> CMPT_PRICE_BITS_MOVING_NUM];
}

static ALWAYS_INLINE uint32_t CmptPriceBit1(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob)
{
    return encCtx->priceRootTable[(bit0Prob ^ (CMPTLZ_PROB_MAX_NUM - 1)) >> CMPT_PRICE_BITS_MOVING_NUM];
}

static ALWAYS_INLINE uint32_t CmptPriceSymbol(CmptLzEncCtx *encCtx,
    CmptlzProb *symbolProbs, uint32_t symbolBitsNum, uint32_t symbol)
{
    uint32_t price = 0;
    symbol += (1U << symbolBitsNum);
    do {
        uint32_t bit = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[symbol], bit); // 这个symbol跟概率表更新下标一样
    } while (symbol != 1);
    return price;
}

static ALWAYS_INLINE uint32_t CmptPriceSymbolReverse(CmptLzEncCtx *encCtx,
    CmptlzProb *symbolProbs, uint32_t symbolBitsNum, uint32_t symbol)
{
    uint32_t price = 0;
    uint32_t i = 1;
    do {
        uint32_t bit = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[i], bit);
        i = (i << 1) + bit;
    } while (--symbolBitsNum);

    return price;
}
#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_ENC_PRICE_H */