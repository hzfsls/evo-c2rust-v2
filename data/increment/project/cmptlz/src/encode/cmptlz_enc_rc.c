/**
 * @file cmptlz_enc_rc.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩range coding区间编码文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_enc_rc.h"

#define CMPT_RC_LEN_LIMIT_1 8
#define CMPT_RC_LEN_LIMIT_2 16

CMPTLZ_HIDDEN int CmptRcPrepare(CmptLzEncCtx *encCtx, uint8_t *dest, size_t *destLen, CmptLzMemHook *alloc)
{
    /* rcCtx alloc */
    CmptRcCtx *rc = alloc->CmptLzAlloc(CMPTLZ_RC_CCTX_HANDLE, sizeof(CmptRcCtx));
    if (rc == NULL) {
        return CMPT_ENC_RC_INIT_FAIL;
    }
    memset_s(rc, sizeof(CmptRcCtx), 0, sizeof(CmptRcCtx));
    /* passing */
    encCtx->rcCtx = rc;
    /* bufbase alloc */
    rc->bufBase = alloc->CmptLzAlloc(CMPTLZ_RC_BUF_HANDLE, CMPTLZ_RC_BUFFER_SIZE);
    memset_s(rc->bufBase, CMPTLZ_RC_BUFFER_SIZE, 0, CMPTLZ_RC_BUFFER_SIZE);
    if (rc->bufBase == NULL) {
        return CMPT_ENC_RC_INIT_FAIL;
    }
    /* output */
    rc->outBufLeft = *destLen;
    rc->outBuf = dest;
    /* rc init */
    rc->buf = rc->bufBase;
    rc->range = 0xFFFFFFFF;
    rc->cacheSize = 0;
    rc->cache = 0;
    rc->low = 0;
    return 0;
}

/* 刷出去 1 << 16 即64K Bytes的数据 */
ALWAYS_NO_INLINE int CmptRcFlush64Kb(CmptRcCtx *rcCtx)
{
    size_t flushOutLen = rcCtx->buf - rcCtx->bufBase;
    int res = memcpy_s(rcCtx->outBuf, rcCtx->outBufLeft, rcCtx->bufBase, flushOutLen);
    if (res != 0) {
        return CMPT_ENC_ERROR_WRITE; // 往outbuffer拷贝错误
    }
    rcCtx->outBuf += flushOutLen;
    rcCtx->outBufLeft -= flushOutLen;
    rcCtx->buf = rcCtx->bufBase;
    return CMPT_OK;
}

ALWAYS_NO_INLINE int CmptRcShiftLow(CmptRcCtx *rcCtx)
{
    int res = CMPT_OK;
    uint32_t lowLow32 = (uint32_t)rcCtx->low;
    uint64_t high = (uint32_t)(rcCtx->low >> 32); // 高32位
    rcCtx->low = (uint32_t)(lowLow32 << 8); // low左移8位
    CMPT_RC_BREAK_CHECK(rcCtx, rcCtx->buf, res); // 防止第一次进入时到结尾导致越界
    if (lowLow32 < 0xFF000000 || high != 0) {
        uint8_t *buf = rcCtx->buf;
        *(buf) = (uint8_t)(rcCtx->cache + high);
        buf++;
        rcCtx->buf = buf;
        rcCtx->cache = (uint8_t)(lowLow32 >> 24); // 32-24为高8位
        CMPT_RC_BREAK_SHIFTING(rcCtx, buf, res);
        high += 0xFF;
        while (1) {
            uint8_t *buf1 = rcCtx->buf;
            CMPT_RC_BREAK_SHIFTING(rcCtx, buf1, res);
            *(buf1++) = (uint8_t)(high);
            rcCtx->buf = buf1;
            rcCtx->cacheSize--;
        }
        CMPT_RC_BREAK_SHIFTING(rcCtx, buf, res);
    } else { // 即0xFF00 0000 <= rcCtx->low <= 0xFFFF FFFF
        rcCtx->cacheSize++;
    }
    return res;
}

CMPTLZ_HIDDEN int CmptRcFlushData(CmptRcCtx *rcCtx)
{
    int i;
    int res;
    for (i = 0; i < 5; i++) { // 循环5次
        res = CmptRcShiftLow(rcCtx);
        if (res != CMPT_OK) {
            break;
        }
    }
    return res;
}

void CmptRcCtxInit(CmptRcCtx *rcCtx)
{
    rcCtx->range = 0xFFFFFFFF;
    rcCtx->cache = 0;
    rcCtx->low = 0;
    rcCtx->cacheSize = 0;
    rcCtx->buf = rcCtx->bufBase;
}

int CmptRcLenProcess(CmptLenEncoder *lenEncoder, CmptRcCtx *rcCtx, uint32_t len, uint64_t posState)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range;
    uint32_t newBound, bit0Prob;
    len -= CMPTLZ_MATCH_LEN_MIN;

    CmptlzProb *probs = lenEncoder->low;
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    if (len >= CMPT_LEN_BOUND) { // 本来就是减过2进来的，所以范围是10到17
        CMPT_RC_BIT_1_PROCESS(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs += CMPT_LEN_BOUND;
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        if (len >= CMPT_LEN_BOUND * CMPT_DOUBLE) { // 18到273
            CMPT_RC_BIT_1_PROCESS(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            rcCtx->range = range;
            shiftRes = CmptRcLitProcess(rcCtx, lenEncoder->high, len - CMPT_LEN_BOUND * CMPT_DOUBLE);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            return CMPT_OK;
        }
        len -= CMPT_LEN_BOUND;
    }

    uint32_t m, bit;
    CMPT_RC_BIT_0_PROCESS(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs += (posState << (1 + 3)); // 左移 1 + 3 位
    bit = (len >> 2); // 右移 2 位
    CMPT_RC_BIT_PROCESS(rcCtx, probs + 1, bit, bit0Prob, range, newBound, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    m = (1 << 1) + bit;
    bit = (len >> 1) & 1;
    CMPT_RC_BIT_PROCESS(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    m = (m << 1) + bit;
    bit = len & 1;
    CMPT_RC_BIT_PROCESS(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    rcCtx->range = range;
    return CMPT_OK;
}

ALWAYS_INLINE int CmptRcPosSlotProcess(CmptLzEncCtx *encCtx,
    uint32_t posSlot, uint32_t len)
{
    int shiftRes = CMPT_OK;
    uint32_t range = encCtx->rcCtx->range;
    uint32_t sym = posSlot + (1 << 6);
    uint32_t bit0Prob, newBound;
    uint32_t bit;
    CmptlzProb *probs = encCtx->probDistSlot[GET_LEN_TO_POS_STATE(len)];
    do {
        CmptlzProb *posSlotProbTableIndex = probs + (sym >> CMPTLZ_DIST_SLOT_BITS);
        bit = (sym >> (CMPTLZ_DIST_SLOT_BITS - 1)) & 1;
        sym <<= 1;
        CMPT_RC_BIT_PROCESS(encCtx->rcCtx, posSlotProbTableIndex, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    while (sym < (1 << (CMPTLZ_DIST_SLOT_BITS * 2)));   // 编6 bit，sym初始为1倍，循环终止条件写2倍
    encCtx->rcCtx->range = range;
    return CMPT_OK;
}

/* 类比lit，从高向低，reverse从低到高 */
static ALWAYS_INLINE int CmptRcReverseProcess(CmptRcCtx *rcCtx, CmptlzProb *probs,
    uint32_t numBits, uint32_t sym)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range;
    uint32_t bit0Prob, newBound;
    uint32_t bit;
    uint32_t m = 1;
    do {
        bit = sym & 1;
        sym >>= 1;
        CMPT_RC_BIT_PROCESS(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        m = (m << 1) | bit;
    }
    while (--numBits);

    rcCtx->range = range;
    return CMPT_OK;
}

int CmptRcDistProcess(CmptLzEncCtx *encCtx, uint32_t posSlot, uint32_t dist)
{
    int shiftRes = CMPT_OK;
    // 首先进入这里面一定dist >= 4
    uint32_t footerBits = ((posSlot >> 1) - 1);
    if (dist < CMPT_DIST_LIMIT_2) {
        uint32_t base = ((2 | (posSlot & 1)) << footerBits);
        shiftRes = CmptRcReverseProcess(encCtx->rcCtx, encCtx->probDistSpecial + base,
            footerBits, dist);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    } else {
        uint32_t pos2 = (dist | 0xF) << (32 - footerBits);
        uint32_t range = encCtx->rcCtx->range;
        do {
            range >>= 1;
            encCtx->rcCtx->low += range & (0 - (pos2 >> 31)); // 右移31位
            pos2 += pos2;
            CMPT_RC_NORMALIZE(encCtx->rcCtx, range, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        }
        while (pos2 != 0xF0000000);
        // 不用CmptRcReverseProcess,因这里4个bit的最后1bit比较特殊
        uint32_t m = 1;
        uint32_t bit;
        uint32_t bit0Prob, newBound;
        int k;
        for (k = 0; k < CMPTLZ_ALIGN_BITS - 1; k++) { // 前3个bits
            bit = dist & 1;
            dist >>= 1;
            CMPT_RC_BIT_PROCESS(encCtx->rcCtx, encCtx->probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            m = (m << 1) + bit;
        }
        bit = dist & 1; // 最后1bit不一样
        CMPT_RC_BIT_PROCESS(encCtx->rcCtx, encCtx->probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        encCtx->rcCtx->range = range;
    }
    return CMPT_OK;
}
