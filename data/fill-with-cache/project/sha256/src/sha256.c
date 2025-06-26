typedef struct
{
    uint32_t h[8];
    uint32_t N[2];
    uint32_t block[SHA256_BLOCK_SIZE / sizeof(uint32_t)];
    uint32_t blocklen;
    uint32_t outlen;
    uint32_t computed : 1;
    uint32_t corrupted : 1;
} VOS_SHA256_CTX;

static const uint32_t K256[64] = {
    0x428a2f98UL, 0x71374491UL, 0xb5c0fbcfUL, 0xe9b5dba5UL, 0x3956c25bUL, 0x59f111f1UL, 0x923f82a4UL, 0xab1c5ed5UL,
    0xd807aa98UL, 0x12835b01UL, 0x243185beUL, 0x550c7dc3UL, 0x72be5d74UL, 0x80deb1feUL, 0x9bdc06a7UL, 0xc19bf174UL,
    0xe49b69c1UL, 0xefbe4786UL, 0x0fc19dc6UL, 0x240ca1ccUL, 0x2de92c6fUL, 0x4a7484aaUL, 0x5cb0a9dcUL, 0x76f988daUL,
    0x983e5152UL, 0xa831c66dUL, 0xb00327c8UL, 0xbf597fc7UL, 0xc6e00bf3UL, 0xd5a79147UL, 0x06ca6351UL, 0x14292967UL,
    0x27b70a85UL, 0x2e1b2138UL, 0x4d2c6dfcUL, 0x53380d13UL, 0x650a7354UL, 0x766a0abbUL, 0x81c2c92eUL, 0x92722c85UL,
    0xa2bfe8a1UL, 0xa81a664bUL, 0xc24b8b70UL, 0xc76c51a3UL, 0xd192e819UL, 0xd6990624UL, 0xf40e3585UL, 0x106aa070UL,
    0x19a4c116UL, 0x1e376c08UL, 0x2748774cUL, 0x34b0bcb5UL, 0x391c0cb3UL, 0x4ed8aa4aUL, 0x5b9cca4fUL, 0x682e6ff3UL,
    0x748f82eeUL, 0x78a5636fUL, 0x84c87814UL, 0x8cc70208UL, 0x90befffaUL, 0xa4506cebUL, 0xbef9a3f7UL, 0xc67178f2UL,
};

#define SHA256_BLOCK_SIZE 64

#define SHA256_DIGEST_SIZE 32

#define SHA256_OK 0

#define SHA256_ERROR ((uint32_t)(~0))

#define BITS_PRE_BYTE 8

#define SHIFTS_PER_BYTE 3

#define BITSIZE(t) (sizeof(t) * BITS_PRE_BYTE)

#define PUT_UINT32_BE(v, p, i)                                                                                         \
    do                                                                                                                 \
    {                                                                                                                  \
        (p)[(i) + 0] = (uint8_t)((v) >> 24);                                                                           \
        (p)[(i) + 1] = (uint8_t)((v) >> 16);                                                                           \
        (p)[(i) + 2] = (uint8_t)((v) >> 8);                                                                            \
        (p)[(i) + 3] = (uint8_t)((v) >> 0);                                                                            \
    } while (0)

#define GET_UINT32_BE(p, i)                                                                                            \
    (((uint32_t)(p)[(i) + 0] << 24) | ((uint32_t)(p)[(i) + 1] << 16) | ((uint32_t)(p)[(i) + 2] << 8) |                 \
     ((uint32_t)(p)[(i) + 3] << 0))

#define VOS_ROTR32(x, uiBlcLen) (((x) << (32 - (uiBlcLen))) | ((x) >> (uiBlcLen)))


#define VOS_ROUND(a, b, c, d, e, f, g, h, i, k, W)                                                                     \
    do                                                                                                                 \
    {                                                                                                                  \
        (h) += (VOS_ROTR32((e), 6) ^ VOS_ROTR32((e), 11) ^ VOS_ROTR32((e), 25)) + ((g) ^ ((e) & ((f) ^ (g)))) + (k) +  \
               (W)[(i)];                                                                                               \
        (d) += (h);                                                                                                    \
        (h) += (VOS_ROTR32((a), 2) ^ VOS_ROTR32((a), 13) ^ VOS_ROTR32((a), 22)) + (((a) & ((b) | (c))) | ((b) & (c))); \
    } while (0)

void vosSha256Begin(VOS_SHA256_CTX *pstCtx)
{
    if (pstCtx == NULL)
    {
        return;
    }
    (void)memset_s(pstCtx, sizeof(VOS_SHA256_CTX), 0, sizeof(VOS_SHA256_CTX));
    pstCtx->h[0] = 0x6a09e667UL;
    pstCtx->h[1] = 0xbb67ae85UL;
    pstCtx->h[2] = 0x3c6ef372UL;
    pstCtx->h[3] = 0xa54ff53aUL;
    pstCtx->h[4] = 0x510e527fUL;
    pstCtx->h[5] = 0x9b05688cUL;
    pstCtx->h[6] = 0x1f83d9abUL;
    pstCtx->h[7] = 0x5be0cd19UL;
    pstCtx->outlen = SHA256_DIGEST_SIZE;
}

static uint32_t vosSha256CtxPrepare(VOS_SHA256_CTX *pstCtx, uint32_t uiLen)
{
    uint32_t uiCntFirst;
    uint32_t uiCntSec;
    uiCntFirst = (pstCtx->N[0] + (uiLen << SHIFTS_PER_BYTE)) & 0xffffffffUL;
    if (uiCntFirst < pstCtx->N[0])
    {
        ++pstCtx->N[1];
        if (pstCtx->N[1] == 0)
        {
            pstCtx->corrupted = 1;
            return SHA256_ERROR;
        }
    }
    uiCntSec = pstCtx->N[1] + (uiLen >> (BITSIZE(uint32_t) - SHIFTS_PER_BYTE));
    if (uiCntSec < pstCtx->N[1])
    {
        pstCtx->corrupted = 1;
        return SHA256_ERROR;
    }
    pstCtx->N[1] = uiCntSec;
    pstCtx->N[0] = uiCntFirst;
    return SHA256_OK;
}

static uint32_t vosSha256LastPadding(const uint8_t *pucData, uint32_t uiLen, VOS_SHA256_CTX *pstCtx,
                                     uint32_t *puiPaddingLen)
{
    errno_t err;
    uint32_t uiBlcLen = pstCtx->blocklen;
    uint8_t *pucBlock = (uint8_t *)pstCtx->block;
    if ((uiLen >= SHA256_BLOCK_SIZE) || (uiLen + uiBlcLen >= SHA256_BLOCK_SIZE))
    {
        err = memcpy_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, pucData, SHA256_BLOCK_SIZE - uiBlcLen);
        if (err != EOK)
        {
            pstCtx->corrupted = 1;
            return SHA256_ERROR;
        }
        vosSha256CompressMul(pstCtx, pucBlock, 1);
        *puiPaddingLen = SHA256_BLOCK_SIZE - uiBlcLen;
        pstCtx->blocklen = 0;
        (void)memset_s(pucBlock, SHA256_BLOCK_SIZE, 0, SHA256_BLOCK_SIZE);
    }
    else
    {
        err = memcpy_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, pucData, uiLen);
        if (err != EOK)
        {
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
    if (uiBlcLen > 0)
    {
        vosSha256CompressMul(pstCtx, pucSrc, uiBlcLen);
        uiBlcLen *= SHA256_BLOCK_SIZE;
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }
    if (uiLenTmp != 0)
    {
        pstCtx->blocklen = (uint32_t)uiLenTmp;
        err = memcpy_s((uint8_t *)pstCtx->block, SHA256_BLOCK_SIZE, pucSrc, uiLenTmp);
        if (err != EOK)
        {
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
    if ((pucSrc == NULL) || (uiLenTmp == 0) || (pstCtx == NULL) || (pstCtx->corrupted == 1) ||
        (pstCtx->computed == 1) || (vosSha256CtxPrepare(pstCtx, uiLenTmp) != SHA256_OK))
    {
        return;
    }
    if (pstCtx->blocklen != 0)
    {
        if (vosSha256LastPadding(pucSrc, uiLenTmp, pstCtx, &uiBlcLen) == SHA256_OK)
        {
            pucSrc += uiBlcLen;
            uiLenTmp -= uiBlcLen;
        }
        else
        {
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
    if (pstCtx == NULL)
    {
        return;
    }
    pucBlock = (uint8_t *)pstCtx->block;
    uiBlcLen = pstCtx->blocklen;
    if ((pstCtx->corrupted == 1) || (uiOutSize < pstCtx->outlen))
    {
        (void)memset_s(pstCtx, sizeof(VOS_SHA256_CTX), 0, sizeof(VOS_SHA256_CTX));
        return;
    }
    if (pstCtx->computed == 0)
    {
        pucBlock[uiBlcLen++] = 0x80;
        if (uiBlcLen > (SHA256_BLOCK_SIZE - 8))
        {
            (void)memset_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, 0, SHA256_BLOCK_SIZE - uiBlcLen);
            uiBlcLen = 0;
            vosSha256CompressMul(pstCtx, pucBlock, 1);
        }
        (void)memset_s(pucBlock + uiBlcLen, SHA256_BLOCK_SIZE - uiBlcLen, 0, SHA256_BLOCK_SIZE - 8 - uiBlcLen);
        pucBlock += SHA256_BLOCK_SIZE - 8;
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
    if (pucOut != NULL)
    {
        for (uiIndex = 0; uiIndex < uiBlcLen; uiIndex++)
        {
            PUT_UINT32_BE(pstCtx->h[uiIndex], pucOut, sizeof(uint32_t) * uiIndex);
        }
    }
    return;
}

static void vosSha256CompressBlock(uint32_t state[VOS_SHA256_CTX_HASH_LEN], const uint8_t block[SHA256_BLOCK_SIZE])
{
    uint32_t W[64];
    uint32_t i, j;
    uint32_t a, b, c, d, e, f, g, h;
    for (i = 0; i < 16; i++)
    {
        W[i] = GET_UINT32_BE(block, 4 * (i));
    }
    for (i = 16; i < 64; i++)
    {
        W[i] = W[i - 16] + W[i - 7] + (VOS_ROTR32(W[i - 15], 7) ^ VOS_ROTR32(W[i - 15], 18) ^ (W[i - 15] >> 3)) +
               (VOS_ROTR32(W[i - 2], 17) ^ VOS_ROTR32(W[i - 2], 19) ^ (W[i - 2] >> 10));
    }
    j = 0;
    a = state[j++];
    b = state[j++];
    c = state[j++];
    d = state[j++];
    e = state[j++];
    f = state[j++];
    g = state[j++];
    h = state[j];
    for (i = 0; i < 64; i += 8)
    {
        j = 0;
        VOS_ROUND(a, b, c, d, e, f, g, h, i + (j++), K256[i + 0], W);
        VOS_ROUND(h, a, b, c, d, e, f, g, i + (j++), K256[i + 1], W);
        VOS_ROUND(g, h, a, b, c, d, e, f, i + (j++), K256[i + 2], W);
        VOS_ROUND(f, g, h, a, b, c, d, e, i + (j++), K256[i + 3], W);
        VOS_ROUND(e, f, g, h, a, b, c, d, i + (j++), K256[i + 4], W);
        VOS_ROUND(d, e, f, g, h, a, b, c, i + (j++), K256[i + 5], W);
        VOS_ROUND(c, d, e, f, g, h, a, b, i + (j++), K256[i + 6], W);
        VOS_ROUND(b, c, d, e, f, g, h, a, i + j, K256[i + 7], W);
    }
    j = 0;
    state[j++] += a;
    state[j++] += b;
    state[j++] += c;
    state[j++] += d;
    state[j++] += e;
    state[j++] += f;
    state[j++] += g;
    state[j] += h;
}

static void vosSha256CompressMul(VOS_SHA256_CTX *pstCtx, const uint8_t *pucInput, uint32_t uiNum)
{
    uint32_t uiNumTmp = uiNum;
    const uint8_t *pucBlock = pucInput;
    while ((uiNumTmp--) != 0)
    {
        vosSha256CompressBlock(pstCtx->h, pucBlock);
        pucBlock += SHA256_BLOCK_SIZE;
    }
}

void VOS_Sha256Calc(const uint8_t *pucInput, uint32_t uiInputLen, uint8_t *pucOutput, uint32_t uiOutputLen)
{
    VOS_SHA256_CTX stCtx;
    vosSha256Begin(&stCtx);
    vosSha256Hash(pucInput, uiInputLen, &stCtx);
    vosSha256End(pucOutput, uiOutputLen, &stCtx);
}

