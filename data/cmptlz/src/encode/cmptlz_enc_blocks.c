/**
 * @file cmptlz_enc_blocks.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩块处理文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_inner.h"
#include "cmptlz_enc_symbol.h"

static void CmptlzEndMarker(void)
{
    /* 默认模式不添加文件尾 */
    return;
}

static int CmptlzFlush(CmptLzEncCtx *encCtx)
{
    encCtx->encNeedFinish = true;
    if (encCtx->endMarker != 0) {
        CmptlzEndMarker();
    }

    CmptRcFlushData(encCtx->rcCtx);
    return CmptRcFlush64Kb(encCtx->rcCtx);
}

static ALWAYS_INLINE void CmptPriceCheck(CmptLzEncCtx *encCtx)
{
    // 需要判断俩表要不要重置
    if (encCtx->matchPriceCount >= CMPT_PRICE_COUNT) {
        CmptPriceGenDistTable(encCtx);
        CmptPriceGenAlignTable(encCtx);
        CmptPriceGenLenTable(encCtx, &encCtx->matchLenEncoder);
    }
    if (encCtx->repLenPriceCount <= 0) {
        encCtx->repLenPriceCount = CMPT_PRICE_COUNT;
        CmptPriceGenLenTable(encCtx, &encCtx->repLenEncoder);
    }
}

static ALWAYS_INLINE int CmptEncShortOrRep0(CmptLzEncCtx *encCtx, uint32_t nowpos32, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    if (lenRes == 1) { // shortrep
        shiftRes = CmptlzEncShortRep(encCtx, nowpos32);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    } else { // longrep0
        shiftRes = CmptlzEncLongRep(encCtx, 0, nowpos32, lenRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

CMPTLZ_STATIC int CmptEncodeOneBlock(CmptLzEncCtx *encCtx)
{
    CmptMfCtx *mf = encCtx->mfCtx;
    uint32_t nowpos32 = encCtx->nowpos64;
    uint32_t startpos = nowpos32;
    uint32_t backRes, lenRes;
    int shiftRes = CMPT_OK;
    while (true) {
        CmptlzDp(encCtx, mf, nowpos32);
        backRes = encCtx->backRes;
        lenRes = encCtx->lenRes;
#ifdef CMPTLZ_PRINTF_ENC_PROCESS
        printf(" now in CmptEncodeOneBlock process, backRes is %u, lenRes is %u\n", backRes, lenRes);
        printf(" nowpos32 is %u\n", nowpos32);
#endif /* CMPTLZ_PRINTF_ENC_PROCESS */

        switch (backRes) {
            case CMPTLZ_UINT32_MAX:
                shiftRes = CmptlzEncLit(encCtx, mf, nowpos32);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
            case 0:
                shiftRes = CmptEncShortOrRep0(encCtx, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
            case 1: // longrep1
                shiftRes = CmptlzEncLongRep(encCtx, 1, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
            case 2: // longrep2
                shiftRes = CmptlzEncLongRep(encCtx, 2, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
            case 3: // longrep3
                shiftRes = CmptlzEncLongRep(encCtx, 3, nowpos32, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
            default:
                shiftRes = CmptlzEncNormalMatch(encCtx, nowpos32, backRes, lenRes);
                CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
                break;
        }

        nowpos32 += lenRes;
        mf->mfStart += lenRes;
        mf->readAhead -= lenRes;

        if (mf->readAhead == 0) {
            /* Alignprice and distprice is filled in CmptPriceCheck */
            CmptPriceCheck(encCtx);
            if (mf->srcLen <= mf->mfStart) {
                break;
            }
            if (nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE) {
                encCtx->nowpos64 += nowpos32 - startpos;
                return 0;
            }
        }
    }
    encCtx->nowpos64 += nowpos32 - startpos;
    return CmptlzFlush(encCtx);
}

CMPTLZ_HIDDEN int CmptEncodeAll(CmptLzEncCtx *encCtx)
{
    CmptRcCtx *rc = encCtx->rcCtx;
    CmptMfCtx *mf = encCtx->mfCtx;
    /* 0 Bytes, goto flush */
    if (mf->srcLen == 0) {
        return CmptlzFlush(encCtx);
    }
    /* first byte */
    if (encCtx->nowpos64 == 0) {
        uint32_t range, bit0Prob, newBound;
        range = rc->range;
        CmptlzProb *probs = &encCtx->isMatch[encCtx->state][0];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        int shiftRes = CMPT_OK;
        CMPT_RC_BIT_0_PROCESS(rc, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        rc->range = range;
        uint8_t curByte = *(mf->srcStart);
        CmptlzProb *litProb = &encCtx->litMarcov.literal[0][0];
        shiftRes = CmptRcLitProcess(rc, litProb, curByte);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        mf->mfStart++;
        encCtx->nowpos64++;
        mf->readPos++;
        if (mf->srcLen == 1) {
            return CmptlzFlush(encCtx);
        }
    }
    /* files smaller than 128 MB are processed in codeoneblock. Larger files are processed cyclically */
    int res;
    while (true) {
        res = CmptEncodeOneBlock(encCtx);
        if (res != 0 || encCtx->encNeedFinish) {
            break;
        }
    }
    return res;
}