/**
 * @file sha256.c
 * @copyright Copyright (c) Huawei Technologies Co., Ltd. 2021-2021. All rights reserved.
 * @brief sha256算法对内源文件
 * @details sha256算法对内源文件
 * @author c00464580
 * @date 2021-06-08
 * @version v1.0.2
 ********************************************************************************************
 * @par 修改日志：
 * <table>
 * <tr><th>Date        <th>Version  <th>Author    <th>Description
 * <tr><td>2021-06-08  <td>1.0.0    <td>c00464580 <td>创建初始版本
 * <tr><td>2022-09-22  <td>1.0.1    <td>x00577306 <td>可信能力提升
 * <tr><td>2022-09-22  <td>1.0.2    <td>r30000719 <td>可信能力提升
 * </table>
 *
 ********************************************************************************************
 */

#include "securec.h"
#include "sha256.h"

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#define SHA256_OK 0

#define SHA256_ERROR ((uint32_t)(~0))

#define BITS_PRE_BYTE   8 // bits per byte
#define SHIFTS_PER_BYTE 3 // shifts per byte
#define BITSIZE(t)      (sizeof(t) * BITS_PRE_BYTE)

#ifndef PUT_UINT32_BE
#define PUT_UINT32_BE(v, p, i)                                       \
    do {                                                             \
        (p)[(i) + 0] = (uint8_t)((v) >> 24); /* 24: 4th byte */      \
        (p)[(i) + 1] = (uint8_t)((v) >> 16); /* 16: 3rd byte */      \
        (p)[(i) + 2] = (uint8_t)((v) >>  8); /*  8: 2nd byte */      \
        (p)[(i) + 3] = (uint8_t)((v) >>  0); /*  0: 1st byte */      \
    } while (0)
#endif

#ifndef GET_UINT32_BE
#define GET_UINT32_BE(p, i)                                          \
    (((uint32_t)(p)[(i) + 0] << 24) | /* 24: 4th byte */             \
    ((uint32_t)(p)[(i) + 1] << 16) |  /* 16: 3rd byte */             \
    ((uint32_t)(p)[(i) + 2] <<  8) |  /*  8: 2nd byte */             \
    ((uint32_t)(p)[(i) + 3] <<  0))   /*  0: 1st byte */
#endif

static void vosSha256CompressMul(VOS_SHA256_CTX *pstCtx, const uint8_t *pucInput, uint32_t uiNum);

void vosSha256Begin(VOS_SHA256_CTX *pstCtx)
{
    if (pstCtx == NULL) {
        return;
    }

    /**
     * @RFC 4634 6.1 SHA-224 and SHA-256 Initialization
     * SHA-256, the initial hash value
     */
    (void)memset_s(pstCtx, sizeof(VOS_SHA256_CTX), 0, sizeof(VOS_SHA256_CTX));
    pstCtx->h[0] = 0x6a09e667UL; /* H(0)0 = 0x6a09e667UL */
    pstCtx->h[1] = 0xbb67ae85UL; /* H(0)1 = 0xbb67ae85UL */
    pstCtx->h[2] = 0x3c6ef372UL; /* H(0)2 = 0x3c6ef372UL */
    pstCtx->h[3] = 0xa54ff53aUL; /* H(0)3 = 0xa54ff53aUL */
    pstCtx->h[4] = 0x510e527fUL; /* H(0)4 = 0x510e527fUL */
    pstCtx->h[5] = 0x9b05688cUL; /* H(0)5 = 0x9b05688cUL */
    pstCtx->h[6] = 0x1f83d9abUL; /* H(0)6 = 0x1f83d9abUL */
    pstCtx->h[7] = 0x5be0cd19UL; /* H(0)7 = 0x5be0cd19UL */
    pstCtx->outlen = SHA256_DIGEST_SIZE;
}

static uint32_t vosSha256CtxPrepare(VOS_SHA256_CTX *pstCtx, uint32_t uiLen)
{
    uint32_t uiCntFirst;
    uint32_t uiCntSec;

    uiCntFirst = (pstCtx->N[0] + (uiLen << SHIFTS_PER_BYTE)) & 0xffffffffUL; /* 0xffffffffUL 32bit mask */
    if (uiCntFirst < pstCtx->N[0]) {
        ++pstCtx->N[1];
        if (pstCtx->N[1] == 0) { /* overflow */
            pstCtx->corrupted = 1;
            return SHA256_ERROR;
        }
    }

    uiCntSec = pstCtx->N[1] + (uiLen >> (BITSIZE(uint32_t) - SHIFTS_PER_BYTE));
    if (uiCntSec < pstCtx->N[1]) { /* overflow */
        pstCtx->corrupted = 1;
        return SHA256_ERROR;
    }

    pstCtx->N[1] = uiCntSec;
    pstCtx->N[0] = uiCntFirst;
    return SHA256_OK;
}

static uint32_t vosSha256LastPadding(const uint8_t *pucData, uint32_t uiLen,
    VOS_SHA256_CTX *pstCtx, uint32_t *puiPaddingLen)
{
    errno_t err;
    uint32_t uiBlcLen = pstCtx->blocklen;
    uint8_t *pucBlock = (uint8_t *)pstCtx->block;

    if ((uiLen >= SHA256_BLOCK_SIZE) || (uiLen + uiBlcLen >= SHA256_BLOCK_SIZE)) {
        err = memcpy_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, pucData, SHA256_BLOCK_SIZE - uiBlcLen);
        if (err != EOK) {
            pstCtx->corrupted = 1;
            return SHA256_ERROR;
        }
        vosSha256CompressMul(pstCtx, pucBlock, 1);
        *puiPaddingLen = SHA256_BLOCK_SIZE - uiBlcLen;
        pstCtx->blocklen = 0;
        (void)memset_s(pucBlock, SHA256_BLOCK_SIZE, 0, SHA256_BLOCK_SIZE);
    } else {
        err = memcpy_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, pucData, uiLen);
        if (err != EOK) {
            pstCtx->corrupted = 1;
            return SHA256_ERROR;
        }
        pstCtx->blocklen += (uint32_t)uiLen;
        return SHA256_ERROR;
    }

    return SHA256_OK;
}

static void vosSha256HashByBlcMulti(const uint8_t *pucData, uint32_t uiLen, VOS_SHA256_CTX *pstCtx)
{
    errno_t err;
    uint32_t uiBlcLen;
    uint32_t uiLenTmp = uiLen;
    const uint8_t *pucSrc = pucData;

    uiBlcLen = (uint32_t)(uiLenTmp / SHA256_BLOCK_SIZE);
    if (uiBlcLen > 0) {
        vosSha256CompressMul(pstCtx, pucSrc, uiBlcLen);
        uiBlcLen *= SHA256_BLOCK_SIZE;
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }

    if (uiLenTmp != 0) {
        pstCtx->blocklen = (uint32_t)uiLenTmp;
        err = memcpy_s((uint8_t *)pstCtx->block, SHA256_BLOCK_SIZE, pucSrc, uiLenTmp);
        if (err != EOK) {
            pstCtx->corrupted = 1;
            return;
        }
    }
    return;
}

void vosSha256Hash(const uint8_t *pucData, uint32_t uiLen, VOS_SHA256_CTX *pstCtx)
{
    uint32_t uiBlcLen = 0;
    uint32_t uiLenTmp = uiLen;
    const uint8_t *pucSrc = pucData;

    if ((pucSrc == NULL) || (uiLenTmp == 0) || (pstCtx == NULL) ||
        (pstCtx->corrupted == 1) || (pstCtx->computed == 1) || (vosSha256CtxPrepare(pstCtx, uiLenTmp) != SHA256_OK)) {
        return;
    }

    if (pstCtx->blocklen != 0) {
        if (vosSha256LastPadding(pucSrc, uiLenTmp, pstCtx, &uiBlcLen) == SHA256_OK) {
            pucSrc += uiBlcLen;
            uiLenTmp -= uiBlcLen;
        } else {
            return;
        }
    }

    vosSha256HashByBlcMulti(pucSrc, uiLenTmp, pstCtx);
    return;
}

void vosSha256End(uint8_t *pucOut, uint32_t uiOutSize, VOS_SHA256_CTX *pstCtx)
{
    uint32_t uiIndex;
    uint8_t *pucBlock = NULL;
    uint32_t uiBlcLen;

    if (pstCtx == NULL) {
        return;
    }

    pucBlock = (uint8_t *)pstCtx->block;
    uiBlcLen = pstCtx->blocklen;

    if ((pstCtx->corrupted == 1) || (uiOutSize < pstCtx->outlen)) {
        (void)memset_s(pstCtx, sizeof(VOS_SHA256_CTX), 0, sizeof(VOS_SHA256_CTX));
        return;
    }

    if (pstCtx->computed == 0) {
        pucBlock[uiBlcLen++] = 0x80; /* 0x80 add padding end of block */
        if (uiBlcLen > (SHA256_BLOCK_SIZE - 8)) { /* 8 bytes to save bits of input */
            (void)memset_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, 0, SHA256_BLOCK_SIZE - uiBlcLen);
            uiBlcLen = 0;
            vosSha256CompressMul(pstCtx, pucBlock, 1);
        }
        /* 8 bytes to save bits of input */
        (void)memset_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, 0, SHA256_BLOCK_SIZE - 8 - uiBlcLen);

        pucBlock += SHA256_BLOCK_SIZE - 8; /* 8 bytes to save bits of input */
        PUT_UINT32_BE(pstCtx->N[1], pucBlock, 0);
        pucBlock += sizeof(uint32_t);
        PUT_UINT32_BE(pstCtx->N[0], pucBlock, 0);
        pucBlock += sizeof(uint32_t);
        pucBlock -= SHA256_BLOCK_SIZE;
        vosSha256CompressMul(pstCtx, pucBlock, 1);
        pstCtx->blocklen = 0;
        (void)memset_s(pucBlock, SHA256_BLOCK_SIZE, 0, SHA256_BLOCK_SIZE);
        pstCtx->computed = 1;
    }

    uiBlcLen = ((pstCtx->outlen <= uiOutSize) ? pstCtx->outlen : uiOutSize) / sizeof(uint32_t);
    if (pucOut != NULL) {
        for (uiIndex = 0; uiIndex < uiBlcLen; uiIndex++) {
            PUT_UINT32_BE(pstCtx->h[uiIndex], pucOut, sizeof(uint32_t) * uiIndex);
        }
    }

    return;
}

static const uint32_t K256[64] = {
    0x428a2f98UL, 0x71374491UL, 0xb5c0fbcfUL, 0xe9b5dba5UL, 0x3956c25bUL, 0x59f111f1UL, 0x923f82a4UL, 0xab1c5ed5UL, //
    0xd807aa98UL, 0x12835b01UL, 0x243185beUL, 0x550c7dc3UL, 0x72be5d74UL, 0x80deb1feUL, 0x9bdc06a7UL, 0xc19bf174UL, //
    0xe49b69c1UL, 0xefbe4786UL, 0x0fc19dc6UL, 0x240ca1ccUL, 0x2de92c6fUL, 0x4a7484aaUL, 0x5cb0a9dcUL, 0x76f988daUL, //
    0x983e5152UL, 0xa831c66dUL, 0xb00327c8UL, 0xbf597fc7UL, 0xc6e00bf3UL, 0xd5a79147UL, 0x06ca6351UL, 0x14292967UL, //
    0x27b70a85UL, 0x2e1b2138UL, 0x4d2c6dfcUL, 0x53380d13UL, 0x650a7354UL, 0x766a0abbUL, 0x81c2c92eUL, 0x92722c85UL, //
    0xa2bfe8a1UL, 0xa81a664bUL, 0xc24b8b70UL, 0xc76c51a3UL, 0xd192e819UL, 0xd6990624UL, 0xf40e3585UL, 0x106aa070UL, //
    0x19a4c116UL, 0x1e376c08UL, 0x2748774cUL, 0x34b0bcb5UL, 0x391c0cb3UL, 0x4ed8aa4aUL, 0x5b9cca4fUL, 0x682e6ff3UL, //
    0x748f82eeUL, 0x78a5636fUL, 0x84c87814UL, 0x8cc70208UL, 0x90befffaUL, 0xa4506cebUL, 0xbef9a3f7UL, 0xc67178f2UL, //
};

/* Assumes that x is uint32_t and 0 < uiBlcLen < 32 */
#define VOS_ROTR32(x, uiBlcLen) (((x) << (32 - (uiBlcLen))) | ((x) >> (uiBlcLen)))

#define VOS_ROUND(a, b, c, d, e, f, g, h, i, k, W)                                                          \
    do {                                                                                                    \
        (h) += (VOS_ROTR32((e), 6) ^ VOS_ROTR32((e), 11) ^ VOS_ROTR32((e), 25)) /* constants: 6, 11, 25 */  \
            + ((g) ^ ((e) & ((f) ^ (g)))) + (k) + (W)[(i)];                                                 \
        (d) += (h);                                                                                         \
        (h) += (VOS_ROTR32((a), 2) ^ VOS_ROTR32((a), 13) ^ VOS_ROTR32((a), 22)) /* constants: 2, 13, 22 */  \
            + (((a) & ((b) | (c))) | ((b) & (c)));                                                          \
    } while (0)

static void vosSha256CompressBlock(uint32_t state[VOS_SHA256_CTX_HASH_LEN],
                                   const uint8_t block[SHA256_BLOCK_SIZE])
{
    uint32_t W[64];
    uint32_t i, j;
    uint32_t a, b, c, d, e, f, g, h;

    /* RFC 6.2.1. Prepare the message schedule W: */
    /* For t = 0 to 15 */
    /* Wt = M(i)t */
    for (i = 0; i < 16; i++) { // 16 VOS_ROUNDs to prepare the message schedule
        W[i] = GET_UINT32_BE(block, 4 * (i)); /* 4 */
    }

    /* For t = 16 to 63 */
    /* Wt = SSIG1(W(t-2)) + W(t-7) + SSIG0(t-15) + W(t-16) */
    /* @perf: speed up about 18% than expanded in x86_64 */
    for (i = 16; i < 64; i++) {                                           // 16 ~ 64 VOS_ROUNDs
        W[i] = W[i - 16] + W[i - 7] +                                                   /* @RFC: 7, 16 */
            (VOS_ROTR32(W[i - 15], 7) ^ VOS_ROTR32(W[i - 15], 18) ^ (W[i - 15] >> 3)) + /* @RFC: 16, 7, 15, 18, 3 */
            (VOS_ROTR32(W[i - 2], 17) ^ VOS_ROTR32(W[i - 2], 19) ^ (W[i - 2] >> 10));   /* @RFC: 2, 17, 19, 10 */
    }

    /* RFC 6.2.2. Initialize the working variables: */
    /* a, b, ..., g, h = H(i-1)[0..7] */
    j = 0;
    a = state[j++]; // a = H(i-1)0
    b = state[j++]; // b = H(i-1)1
    c = state[j++]; // c = H(i-1)2
    d = state[j++]; // d = H(i-1)3
    e = state[j++]; // e = H(i-1)4
    f = state[j++]; // f = H(i-1)5
    g = state[j++]; // g = H(i-1)6
    h = state[j]; // h = H(i-1)7

    /* RFC 6.2.3. Perform the main hash computation: */
    for (i = 0; i < 64; i += 8) {                 /* 64 VOS_ROUNDs to do hash computation, 8 VOS_ROUNDs pre loop */
        j = 0;
        VOS_ROUND(a, b, c, d, e, f, g, h, i + (j++), K256[i + 0], W); // 0
        VOS_ROUND(h, a, b, c, d, e, f, g, i + (j++), K256[i + 1], W); // 1
        VOS_ROUND(g, h, a, b, c, d, e, f, i + (j++), K256[i + 2], W); // 2
        VOS_ROUND(f, g, h, a, b, c, d, e, i + (j++), K256[i + 3], W); // 3
        VOS_ROUND(e, f, g, h, a, b, c, d, i + (j++), K256[i + 4], W); // 4
        VOS_ROUND(d, e, f, g, h, a, b, c, i + (j++), K256[i + 5], W); // 5
        VOS_ROUND(c, d, e, f, g, h, a, b, i + (j++), K256[i + 6], W); // 6
        VOS_ROUND(b, c, d, e, f, g, h, a, i + j, K256[i + 7], W); // 7
    }

    /* RFC 6.2.4. Compute the intermediate hash value H(i): */
    /* H(i) = [a, b, ..., g, h] + H(i-1)[0..7] */
    j = 0;
    state[j++] += a; // H(i)0 = a + H(i-1)0
    state[j++] += b; // H(i)1 = b + H(i-1)1
    state[j++] += c; // H(i)2 = c + H(i-1)2
    state[j++] += d; // H(i)3 = d + H(i-1)3
    state[j++] += e; // H(i)4 = e + H(i-1)4
    state[j++] += f; // H(i)5 = f + H(i-1)5
    state[j++] += g; // H(i)6 = g + H(i-1)6
    state[j] += h;   // H(i)7 = h + H(i-1)7
}
#undef VOS_ROTR32
#undef VOS_ROUND

static void vosSha256CompressMul(VOS_SHA256_CTX *pstCtx, const uint8_t *pucInput, uint32_t uiNum)
{
    uint32_t uiNumTmp = uiNum;
    const uint8_t *pucBlock = pucInput;

    while ((uiNumTmp--) != 0) {
        vosSha256CompressBlock(pstCtx->h, pucBlock);
        pucBlock += SHA256_BLOCK_SIZE;
    }
}

void VOS_Sha256Calc(const uint8_t *pucInput, uint32_t uiInputLen,
    uint8_t *pucOutput, uint32_t uiOutputLen)
{
    VOS_SHA256_CTX stCtx;

    vosSha256Begin(&stCtx);
    vosSha256Hash(pucInput, uiInputLen, &stCtx);
    vosSha256End(pucOutput, uiOutputLen, &stCtx);
}

#ifdef __cplusplus
}
#endif /* __cplusplus */
