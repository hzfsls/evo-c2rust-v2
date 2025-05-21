/**
 * @file cmptlz_enc.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2024-2024. All rights reserved.
 * @brief CMPTLZ 压缩入口文件
 * @author Anonym
 * @date 2024-01-09
 */

#include "cmptlz_log.h"
#include "cmptlz_enc_inner.h"

CMPTLZ_HIDDEN void CmptlzFreeAll(CmptLzEncCtx *encCtx, CmptLzMemHook *alloc)
{
    if (encCtx == NULL) { // 已free
        return;
    }
    /* free mf */
    if (encCtx->mfCtx != NULL) {
        if (encCtx->mfCtx->hash != NULL) {
            alloc->CmptLzFree(CMPTLZ_MF_HASH_HANDLE, encCtx->mfCtx->hash);
            encCtx->mfCtx->hash = NULL;
        }
        if (encCtx->mfCtx->son != NULL) {
            alloc->CmptLzFree(CMPTLZ_MF_SON_HANDLE, encCtx->mfCtx->son);
            encCtx->mfCtx->son = NULL;
        }
        alloc->CmptLzFree(CMPTLZ_MF_CCTX_HANDLE, encCtx->mfCtx);
        encCtx->mfCtx = NULL;
    }
    /* free rc */
    if (encCtx->rcCtx != NULL) {
        if (encCtx->rcCtx->bufBase != NULL) {
            alloc->CmptLzFree(CMPTLZ_RC_BUF_HANDLE, encCtx->rcCtx->bufBase);
            encCtx->rcCtx->bufBase = NULL;
        }
        alloc->CmptLzFree(CMPTLZ_RC_CCTX_HANDLE, encCtx->rcCtx);
        encCtx->rcCtx = NULL;
    }
    /* free enc */
    alloc->CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE, encCtx);
    encCtx = NULL;
}

int CmptlzEncodeIO(CmptLzEncCtx *encCtx, uint8_t *dest, size_t *destLen,
    const uint8_t *src, size_t srcLen, CmptLzMemHook *alloc)
{
    int res;
    /* instream & mf set */
    res = CmptMfPrepare(encCtx, src, srcLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG(res, "CmptMfPrepare Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    /* outStream & rc set */
    res = CmptRcPrepare(encCtx, dest, destLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG(res, "CmptRcPrepare Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    /* encCtx set */
    CmptlzEncPrepare(encCtx);
    /* encode each block */
    res = CmptEncodeAll(encCtx);
    /* check Response */
    if (res != 0) {
        CmptlzFreeAll(encCtx, alloc);
        CMPTLZ_LOG(res, "CmptEncode Process Fail!");
        return res;
    }
    /* output compressed size */
    *destLen -= encCtx->rcCtx->outBufLeft;
    /* check fileSize */
    if (encCtx->nowpos64 != srcLen) {
        CMPTLZ_LOG(res, "CmptEncode FileSize Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return CMPT_ENC_ERROR_FILESIZE;
    }
    /* free all */
    CmptlzFreeAll(encCtx, alloc);
    return res;
}

int CmptlzEncode(uint8_t *dest, size_t *destLen, const uint8_t *src, size_t srcLen,
    const CmptlzEncParam *props, uint8_t *propsEncoded, size_t *propsSize,
    int writeEndMark, CmptLzMemHook *alloc)
{
    int res;
    if (alloc == NULL || alloc->CmptLzAlloc == NULL || alloc->CmptLzFree == NULL) {
        CMPTLZ_LOG(CMPT_ENC_ERROR_PARAM, "Cmptlz input wrong param!");
        return CMPT_ENC_ERROR_PARAM;
    }
    /* Encode context initialization */
    CmptLzEncCtx *encCtx = (CmptLzEncCtx *)CmptInitCctx(alloc, writeEndMark);
    if (encCtx == NULL) {
        CMPTLZ_LOG(CMPT_ENC_CTX_INIT_FAIL, "CmptInitCctx Fail!");
        return CMPT_ENC_CTX_INIT_FAIL;
    }
    CmptlzSetParam(encCtx, props);
    res = CmptHeadWrite(encCtx, propsEncoded, propsSize);
    if (res != 0) {
        alloc->CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE, encCtx);
        CMPTLZ_LOG(res, "CmptHeadWrite Fail!");
        return res;
    }
    res = CmptlzEncodeIO(encCtx, dest, destLen, src, srcLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG(res, "CmptlzEncode I / O Fail!");
    }
    return res;
}

int CmptlzCompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzCompParam *param)
{
    if ((src == NULL) && (srcSize != 0)) {
        return CMPT_ENC_ERROR_PARAM;
    }
    /* 默认不写文件尾 */
    const int endMarker = 0;
    /* 外部用户设置参数 */
    CmptlzEncParam props;
    props.level = param->level;
    props.dictSize = param->dictSize;
    props.litCtx = param->litCtx;
    props.litPos = param->litPos;
    props.posBits = param->posBits;
    props.fastBytes = param->fastBytes;
    props.numThreads = param->numThreads;
    CmptLzMemHook *alloc = param->memHook;
    return CmptlzEncode((uint8_t *)dst, dstSize, (const uint8_t *)src, srcSize,
        &props, param->protData, &param->protSize, endMarker, alloc);
}