/**
 * @file cmptlz_dec.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2022-2023. All rights reserved.
 * @brief CMPTLZ 解压对外接口文件
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
#include "cmptlz_log.h"

static ALWAYS_INLINE void CmptLzDecCheckDictSizeUpdate(CmptLzDecCtx *decCtx)
{
    if (decCtx->checkDicSize == 0 && decCtx->processedPos >= decCtx->prop.dicSize) {
        decCtx->checkDicSize = decCtx->prop.dicSize;
    }
}

static ALWAYS_INLINE void CmptLzDecRemWriteInDict(CmptLzDecCtx *decCtx, size_t dicPosLimit)
{
    size_t dictPos = decCtx->dictPos;
    size_t remainDecLen = decCtx->remainLen;
    size_t dictBufSize = decCtx->dictBufSize;
    size_t remainDicLen = dicPosLimit - dictPos;
    if (remainDicLen < remainDecLen) {
        remainDecLen = remainDicLen;
    }

    if (remainDecLen == 0) {
        return;
    }

    decCtx->processedPos += (uint32_t)remainDecLen;
    decCtx->remainLen -= (uint32_t)remainDecLen;

    unsigned char *dict = decCtx->dict;
    size_t rep0 = decCtx->reps[0];
    while (remainDecLen != 0) {
        remainDecLen--;
        dict[dictPos] = dict[dictPos - rep0 + (dictPos < rep0 ? dictBufSize : 0)];
        dictPos++;
    }
    decCtx->dictPos = dictPos;

    CmptLzDecCheckDictSizeUpdate(decCtx);
}

static ALWAYS_INLINE void CmptLzDecGetProbsInit(CmptLzDecCtx *decCtx)
{
    uint32_t idx;
    uint32_t numProbs = CmptLzGetNumProbs(&(decCtx->prop));
    CmptLzDecProb *decProbs = decCtx->probs;

    for (idx = 0; idx < numProbs; idx++) {
        decProbs[idx] = CMPTLZ_PROB_LG >> 1;
    }
    decCtx->state = 0;
}

static ALWAYS_INLINE void CmptLzRangeCodeInit(CmptLzDecCtx *decCtx)
{
    uint32_t rangeCode = (uint32_t)(decCtx->tempBuf[1]) << 24; // 初始化rangecode值，左移24位
    rangeCode |= (uint32_t)(decCtx->tempBuf[2]) << 16; // 第一次解压时初始化一共要处理9个字节，根据第2个字节计算，左移16位
    rangeCode |= (uint32_t)(decCtx->tempBuf[3]) << 8;  // 第一次解压时初始化一共要处理9个字节，根据第3个字节计算，左移8位
    rangeCode |= (uint32_t)(decCtx->tempBuf[4]);  // 第一次解压时初始化一共要处理9个字节，rangecode初始化前五个字节(0, 1，2，3，4)
    decCtx->code = rangeCode;
    decCtx->range = 0xFFFFFFFF;
}

static ALWAYS_INLINE int CmptLzDecCtxPrepare(CmptLzDecCtx *decCtx,
    const unsigned char *pSrcIn, size_t srcInLen, EnCmptLzStatus *finStatus)
{
    size_t readCodeLen = CMPTLZ_RANGE_CODE_SIZE - decCtx->tempBufSize;
    readCodeLen = (srcInLen < readCodeLen) ? srcInLen : readCodeLen;
    while (readCodeLen-- > 0) {
        decCtx->tempBuf[decCtx->tempBufSize++] = *pSrcIn++;
    }

    if (decCtx->tempBufSize != 0 && decCtx->tempBuf[0] != 0) {
        decCtx->tempBufSize = 0;
        *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
        return CMPT_ERROR_DATA;
    }
    if (decCtx->tempBufSize < CMPTLZ_RANGE_CODE_SIZE) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    CmptLzRangeCodeInit(decCtx);

    // see CmptLzDecInit. bigger than (max match len + 1) means contex need init probs
    if (decCtx->remainLen > CMPTLZ_MATCH_MAX_LEN + 1) {
        CmptLzDecGetProbsInit(decCtx);
        decCtx->reps[0] = 1; // 第一次解压时初始化一共要处理9个字节，初始化rep四个字节（0，1，2，3）
        decCtx->reps[1] = 1; // 第一次解压时初始化一共要处理9个字节，初始化rep四个字节（0，1，2，3）
        decCtx->reps[2] = 1; // 第一次解压时初始化一共要处理9个字节，初始化rep四个字节（0，1，2，3）
        decCtx->reps[3] = 1; // 第一次解压时初始化一共要处理9个字节，初始化rep四个字节（0，1，2，3）
    }

    decCtx->remainLen = 0;

    return CMPT_OK;
}

/* decode into the dict until dict is full or srcIn is run-out */
int CmptLzDecDecodeToDic(CmptLzDecCtx *decCtx, size_t dicPosLimit,
    const unsigned char *pSrcIn, size_t *pStrInLen, EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus)
{
    int res;
    bool carefulDecDone = false;
    size_t srcDecLenTmp;
    size_t srcDecLen = 0;                   // decoded src length
    size_t srcInLen = *pStrInLen;           // left src length

    if (decCtx->remainLen > CMPTLZ_MATCH_MAX_LEN) { // need init probs and range code
        size_t oldTempBufSize = decCtx->tempBufSize;
        res = CmptLzDecCtxPrepare(decCtx, pSrcIn, srcInLen, finStatus);
        srcDecLenTmp = (decCtx->tempBufSize - oldTempBufSize);
        if ((res != CMPT_OK) || (*finStatus == CMPTLZ_STATUS_NEEDS_MORE_INPUT)) {
            *pStrInLen = srcDecLenTmp;
            return res;
        }
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        decCtx->tempBufSize = 0;
    }

    if (decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) { // endMarker
        if (decCtx->code != 0) {
            return CMPT_ERROR_DATA;
        }
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }

    if (decCtx->remainLen != 0) {
        CmptLzDecRemWriteInDict(decCtx, dicPosLimit);
    }

    // temBufSize not equals to 0 means that some character remained in the tempBuf since last decode
    // we should join it with character this time to decode one itact packet.
    if (decCtx->tempBufSize != 0) {
        res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, &srcDecLenTmp);
        *pStrInLen = srcDecLenTmp;
        if (res == CMPT_ERROR_DATA) {
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
            return CMPT_ERROR_DATA;
        } else if (res == CMPTLZ_DEC_INPUT_EOF) {  // cant even decode one packet thie time, just return EOF
            *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
            return CMPT_OK;
        } else {
            srcDecLen += srcDecLenTmp;
            pSrcIn += srcDecLenTmp;
            srcInLen -= srcDecLenTmp;
        }
    }

    while ((decCtx->dictPos < dicPosLimit) && (carefulDecDone == false)) {
        decCtx->buf = pSrcIn;
        if (srcInLen <= CMPTLZ_REQUIRED_INPUT_MAX) {
            res = CmptLzDecCarefulProcess(decCtx, dicPosLimit, pSrcIn + srcInLen);
            carefulDecDone = true;
        } else {
            res = CmptLzDecDirectProcess(decCtx, dicPosLimit, pSrcIn + srcInLen - CMPTLZ_REQUIRED_INPUT_MAX);
        }
        srcDecLenTmp = (size_t)(decCtx->buf - pSrcIn) + decCtx->tempBufSize;
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;

        if (res == CMPT_ERROR_DATA) {
            *pStrInLen = srcDecLen;
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
            return CMPT_ERROR_DATA;
        }
    }

    *pStrInLen = srcDecLen;
    if ((decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) && (decCtx->code == 0)) {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }
    if (decCtx->dictPos < dicPosLimit) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    if ((decCtx->remainLen == 0) && (decCtx->code == 0)) {
        *finStatus = CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
        return CMPT_OK;
    }
    if (finMode == CMPTLZ_FINISH_ANY) {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
        return CMPT_OK;
    }
    if (decCtx->remainLen != 0) {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
        return CMPT_ERROR_DATA;
    }

    srcDecLenTmp = 0;
    res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, &srcDecLenTmp);
    srcDecLen += srcDecLenTmp;
    *pStrInLen = srcDecLen;
    if (res == CMPTLZ_DEC_INPUT_EOF) {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    if ((decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) && (decCtx->code == 0)) {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }

    *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
    return CMPT_ERROR_DATA;
}

int CmptLzDecDecodeToBuf(CmptLzDecCtx *decCtx, CmptLzDecIn *pDecIn, CmptLzDecOut *pDecOut,
    EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus)
{
    int res = CMPT_OK;
    size_t dictPos; // 表示上一轮次已经解压缩的字符串的字典位置。
    size_t dictPosLimit; // 表示当前轮次解压缩字符串可以达到的最远字典位置
    size_t srcCostSize; // 表示当前轮次解压缩可以消耗的压缩字符串的最大长度

    size_t leftSrcSize = pDecIn->strInLen;
    size_t leftDestSize = pDecOut->destOutLen;
    const unsigned char *pSrcIn = pDecIn->pSrcIn;
    unsigned char *pDestOut = pDecOut->pDestOut;
    EnCmptLzFinMode tmpFinMode;

    do {
        if (decCtx->dictPos == decCtx->dictBufSize) {
            decCtx->dictPos = 0;
        }
        dictPos = decCtx->dictPos;
        if (leftDestSize > decCtx->dictBufSize - dictPos) {
            dictPosLimit = decCtx->dictBufSize; // 这里代表limit
            tmpFinMode = CMPTLZ_FINISH_ANY;
        } else { // 字典大小即为limit
            dictPosLimit = dictPos + leftDestSize;
            tmpFinMode = finMode;
        } // dictPosLimit不足一个字典大小，用pos + outbufferLeft表示
        srcCostSize = leftSrcSize;
        res = CmptLzDecDecodeToDic(decCtx, dictPosLimit, pSrcIn, &srcCostSize, tmpFinMode, finStatus);
        if (res != CMPT_OK) {
            break;
        }
        pSrcIn += srcCostSize;
        leftSrcSize -= srcCostSize;
        dictPosLimit = decCtx->dictPos - dictPos; // 这里代表本轮解压出的字节长度
        leftDestSize -= dictPosLimit;
        if (dictPosLimit == 0) {
            break;
        }
        if (memcpy_s(pDestOut, (pDecOut->destOutLen - leftDestSize), decCtx->dict + dictPos, dictPosLimit) != EOK) {
            return CMPT_ERROR_MEM;
        }
        pDestOut += dictPosLimit;
    } while (leftDestSize != 0);

    pDecIn->strInCostLen = pDecIn->strInLen - leftSrcSize;
    pDecOut->destOutFillLen = pDecOut->destOutLen - leftDestSize;

    return res;
}

int CmptlzDecompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzDecParam *param)
{
    if (src == NULL || dst == NULL || dstSize == NULL) {
        CMPTLZ_LOG(CMPT_ERROR_UNSUPPORTED, "The input parameter NULL is incorrect.");
        return CMPT_ERROR_UNSUPPORTED;
    }

    /* 0x7fffffff 由于安全函数限制，超过2G的直接不支持 */
    if (srcSize > 0x7fffffff || *dstSize > 0x7fffffff) {
        CMPTLZ_LOG(CMPT_ERROR_UNSUPPORTED, "dstSize:0x%zx srcSize:0x%zx", *dstSize, srcSize);
        return CMPT_ERROR_UNSUPPORTED;
    }
    
    if (param == NULL || param->memHook == NULL || param->protData == NULL || param->protSize != CMPTLZ_PROPS_SIZE) {
        CMPTLZ_LOG(CMPT_ERROR_UNSUPPORTED, "The compress param NULL is incorrect.");
        return CMPT_ERROR_UNSUPPORTED;
    }

    CmptLzDecIn decIn = {.pSrcIn = src, .strInLen = srcSize, .strInCostLen = 0};
    CmptLzDecOut decOut = {.pDestOut = dst, .destOutLen = *dstSize, .destOutFillLen = 0};
    EnCmptLzStatus enFinStat = CMPTLZ_STATUS_BUT;
    int ret = CmptLzDecode(&decIn, &decOut, param->protData, CMPTLZ_FINISH_ANY, &enFinStat, param->memHook);

    *dstSize = decOut.destOutFillLen;
    return ret;
}