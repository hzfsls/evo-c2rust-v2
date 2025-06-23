/**
 * @file cmptlz_enc_inner.h
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩内部头文件
 * @author Anonym
 * @date 2024-01-09
 */

#ifndef CMPTLZ_ENC_INNER_H
#define CMPTLZ_ENC_INNER_H

#include "cmptlz_enc_struct.h"
#include "cmptlz_def.h"
#include "securec.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ---------------------------------main--------------------------------------------- */
CMPTLZ_HIDDEN int CmptEncodeAll(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN int CmptHeadWrite(CmptLzEncCtx *encCtx, uint8_t *propsEncoded, size_t *propsSize);
/* ---------------------------------Init--------------------------------------------- */
CMPTLZ_HIDDEN void CmptlzSetParam(CmptLzEncCtx *encCtx, const CmptlzEncParam *props);
CMPTLZ_HIDDEN void CmptlzParamNormalize(CmptlzEncParam *props);
CMPTLZ_HIDDEN int CmptRcPrepare(CmptLzEncCtx *encCtx, uint8_t *dest, size_t *destLen, CmptLzMemHook *alloc);
CMPTLZ_HIDDEN int CmptMfPrepare(CmptLzEncCtx *encCtx, const uint8_t *src, size_t srcLen, CmptLzMemHook *alloc);
CMPTLZ_HIDDEN void CmptlzEncPrepare(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN void CmptlzPriceInit(CmptLzEncCtx *encCtx);
CMPTLZ_HIDDEN void *CmptInitCctx(CmptLzMemHook *alloc, int writeEndMark);
/* ---------------------------dynamic programming------------------------------------- */
#define CMPTLZ_UINT32_MAX (uint32_t)(-1)
#define CMPTLZ_FIND_MIN(x, y) ((x) < (y) ? (x) : (y))
#define CMPTLZ_FIND_MAX(x, y) ((x) > (y) ? (x) : (y))
void CmptlzDp(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t position);
/* ---------------------------------match finder-------------------------------------- */
#define NOT_EQUAL_2_BYTES(a, b) ((a)[0] != (b)[0] || (a)[1] != (b)[1])
CMPTLZ_HIDDEN int CmptlzMfInit(CmptLzEncCtx *encCtx, CmptLzMemHook *alloc);

#define CMPTLZ_RETURN_IF_NOT_OK(res) \
    do { \
        if (CMPTLZ_UNLIKELY(res != CMPT_OK)) { \
            return res; \
        } \
    } while (0)

static ALWAYS_INLINE uint32_t CmptMemCmpByOneByte(const uint8_t *buf1,
    const uint8_t *buf2, uint32_t len, uint32_t limit)
{
    uint32_t lenIn = len;
    while ((lenIn < limit) && (buf1[lenIn] == buf2[lenIn])) {
        lenIn++;
    }
    return lenIn;
}

static ALWAYS_INLINE uint32_t CmptMemCmpLenSafe(const uint8_t *buf1,
    const uint8_t *buf2, uint32_t len, uint32_t limit)
{
    return CmptMemCmpByOneByte(buf1, buf2, len, limit);
}

static ALWAYS_INLINE uint32_t CmptMemCmpLen(const uint8_t *buf1, const uint8_t *buf2, uint32_t len, uint32_t limit)
{
    return CmptMemCmpByOneByte(buf1, buf2, len, limit);
}

static ALWAYS_INLINE uint32_t CmptMfAvail(const CmptMfCtx *mf)
{
    return mf->srcLen - mf->readPos;
}

static ALWAYS_INLINE const uint8_t *CmptMfGetPtr(const CmptMfCtx *mf)
{
    return mf->srcStart + mf->readPos;
}

uint32_t CmptlzMatchFinder(CmptMfCtx *mf, uint32_t *count_ptr, CmptlzMatchPair *matches);
void CmptlzMatchSkiper(CmptMfCtx *mf, uint32_t amount);

/* ---------------------------------state-------------------------------------------- */
#define CMPT_GET_DIST_STATE(len) \
    (((len) < 4 + CMPTLZ_MATCH_LEN_MIN) \
		? (len) - CMPTLZ_MATCH_LEN_MIN \
		: 4 - 1)

#define CMPT_STATE_UPDATE_WHEN_LIT(state) \
    (state) = (((state) <= SHORTREP_LIT_LIT) \
            ? LIT_LIT \
            : (((state) <= LIT_SHORTREP) \
                ? (state) - 3 \
                : (state) - 6))

#define CMPT_STATE_UPDATE_WHEN_MATCH(state) \
    (state) = (((state) < 7) ? LIT_MATCH : NOTLIT_MATCH)

#define CMPT_STATE_UPDATE_WHEN_LONGREP(state) \
    (state) = (((state) < 7) ? LIT_LONGREP : NOTLIT_REP)

#define CMPT_STATE_UPDATE_WHEN_SHORTREP(state) \
    (state) = (((state) < 7) ? LIT_SHORTREP : NOTLIT_REP)

/* ------------------------------------posSloter-------------------------------------- */
static ALWAYS_INLINE uint32_t PosSlotHelper(uint32_t n)
{
    #if (defined(__GNUC__) && (__GNUC__ >= 3))
        return 31 - (uint32_t)__builtin_clz(n);
    #else
        uint32_t i = 31;
        if ((n & 0xFFFF0000) == 0) {
            n <<= 16;
            i = 15;
        }
        if ((n & 0xFF000000) == 0) {
            n <<= 8;
            i -= 8;
        }
        if ((n & 0xF0000000) == 0) {
            n <<= 4;
            i -= 4;
        }
        if ((n & 0xC0000000) == 0) {
            n <<= 2;
            i -= 2;
        }
        if ((n & 0x80000000) == 0)
            --i;
        return i;
    #endif
}

static ALWAYS_INLINE uint32_t PosSloter(uint32_t dist)
{
    if (dist <= 4) {
        return dist;
    }
    uint32_t helper = PosSlotHelper(dist);
    return (helper + helper + ((dist >> (helper - 1)) & 1));
}
#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* CMPTLZ_ENC_INNER_H */