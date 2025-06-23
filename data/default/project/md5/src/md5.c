typedef struct TagMd5Ctx
{
    uint32_t aulState[4];
    uint32_t aulCount[2];
    uint8_t aucBuffer[64];
    uint32_t uiPos;
} MD5_CTX;

#define MD5_DIGEST_LEN 16

#define MD5_INPUT_LEN_MAX 0xffffffffffffffffLL

#define MD5_BUFFER_SIZE 64

#define MD5_TEXT_IN_BUFFER_MAX 56

#define MD5_LINEAR_FUNC_F(B, C, D) (((B) & (C)) | ((~(B)) & (D)))

#define MD5_LINEAR_FUNC_G(B, C, D) (((B) & (D)) | ((C) & (~(D))))

#define MD5_LINEAR_FUNC_H(B, C, D) ((B) ^ (C) ^ (D))

#define MD5_LINEAR_FUNC_I(B, C, D) ((C) ^ ((B) | (~(D))))

#define MD5_RECORD_MESSAGE_LEN(context)                                                                                \
    do                                                                                                                 \
    {                                                                                                                  \
        uint32_t __i;                                                                                                  \
        for (__i = 0; __i < sizeof((context)->aulCount) / sizeof((context)->aulCount[0]); __i++)                       \
        {                                                                                                              \
            (context)->aucBuffer[(context)->uiPos] = (uint8_t)((context)->aulCount[__i] & 0xff);                       \
            (context)->uiPos++;                                                                                        \
            (context)->aucBuffer[(context)->uiPos] = (uint8_t)(((context)->aulCount[__i] >> 8) & 0xff);                \
            (context)->uiPos++;                                                                                        \
            (context)->aucBuffer[(context)->uiPos] = (uint8_t)(((context)->aulCount[__i] >> 16) & 0xff);               \
            (context)->uiPos++;                                                                                        \
            (context)->aucBuffer[(context)->uiPos] = (uint8_t)(((context)->aulCount[__i] >> 24) & 0xff);               \
            (context)->uiPos++;                                                                                        \
        }                                                                                                              \
    } while (0)

#define MD5_COMPOSE_DIGEST(digest, md5State)                                                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        uint32_t __i = 0;                                                                                              \
        uint32_t __j = 0;                                                                                              \
        for (; __i < sizeof((md5State)) / sizeof((md5State)[0]); __i++)                                                \
        {                                                                                                              \
            (digest)[__j] = (uint8_t)(md5State)[__i];                                                                  \
            __j++;                                                                                                     \
            (digest)[__j] = (uint8_t)((md5State)[__i] >> 8);                                                           \
            __j++;                                                                                                     \
            (digest)[__j] = (uint8_t)((md5State)[__i] >> 16);                                                          \
            __j++;                                                                                                     \
            (digest)[__j] = (uint8_t)((md5State)[__i] >> 24);                                                          \
            __j++;                                                                                                     \
        }                                                                                                              \
    } while (0)

#define MD5_CYCLE_MOVE(numMoved, moveBit)                                                                              \
    do                                                                                                                 \
    {                                                                                                                  \
        uint32_t __tmpValue;                                                                                           \
        __tmpValue = (numMoved) >> (32 - (moveBit));                                                                   \
        (numMoved) = (numMoved) << (moveBit);                                                                          \
        (numMoved) += __tmpValue;                                                                                      \
    } while (0)

#define MD5_CHANGE_STATE_IN_TURN(state, value)                                                                         \
    do                                                                                                                 \
    {                                                                                                                  \
        (state)[0] = (state)[3];                                                                                       \
        (state)[3] = (state)[2];                                                                                       \
        (state)[2] = (state)[1];                                                                                       \
        (state)[1] = (state)[1] + (value);                                                                             \
    } while (0)

#define MD5_FUNC_F(value, md5State, text, addEnd, moveBit)                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        (value) = MD5_LINEAR_FUNC_F((md5State)[1], (md5State)[2], (md5State)[3]) + (md5State)[0] + (text) + (addEnd);  \
        MD5_CYCLE_MOVE((value), (moveBit));                                                                            \
        MD5_CHANGE_STATE_IN_TURN((md5State), (value));                                                                 \
    } while (0)

#define MD5_FUNC_G(value, md5State, text, addEnd, moveBit)                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        (value) = MD5_LINEAR_FUNC_G((md5State)[1], (md5State)[2], (md5State)[3]) + (md5State)[0] + (text) + (addEnd);  \
        MD5_CYCLE_MOVE((value), (moveBit));                                                                            \
        MD5_CHANGE_STATE_IN_TURN((md5State), (value));                                                                 \
    } while (0)

#define MD5_FUNC_H(value, md5State, text, addEnd, moveBit)                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        (value) = MD5_LINEAR_FUNC_H((md5State)[1], (md5State)[2], (md5State)[3]) + (md5State)[0] + (text) + (addEnd);  \
        MD5_CYCLE_MOVE((value), (moveBit));                                                                            \
        MD5_CHANGE_STATE_IN_TURN((md5State), (value));                                                                 \
    } while (0)

#define MD5_FUNC_I(value, md5State, text, addEnd, moveBit)                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        (value) = MD5_LINEAR_FUNC_I((md5State)[1], (md5State)[2], (md5State)[3]) + (md5State)[0] + (text) + (addEnd);  \
        MD5_CYCLE_MOVE((value), (moveBit));                                                                            \
        MD5_CHANGE_STATE_IN_TURN((md5State), (value));                                                                 \
    } while (0)

#define MD5_F_PROC(tmpValue, tmpState, textFragment)                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[0], 0xd76aa478, 7);                                          \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[1], 0xe8c7b756, 12);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[2], 0x242070db, 17);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[3], 0xc1bdceee, 22);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[4], 0xf57c0faf, 7);                                          \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[5], 0x4787c62a, 12);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[6], 0xa8304613, 17);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[7], 0xfd469501, 22);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[8], 0x698098d8, 7);                                          \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[9], 0x8b44f7af, 12);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[10], 0xffff5bb1, 17);                                        \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[11], 0x895cd7be, 22);                                        \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[12], 0x6b901122, 7);                                         \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[13], 0xfd987193, 12);                                        \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[14], 0xa679438e, 17);                                        \
        MD5_FUNC_F((tmpValue), (tmpState), (textFragment)[15], 0x49b40821, 22);                                        \
    } while (0)

#define MD5_G_PROC(tmpValue, tmpState, textFragment)                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[1], 0xf61e2562, 5);                                          \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[6], 0xc040b340, 9);                                          \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[11], 0x265e5a51, 14);                                        \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[0], 0xe9b6c7aa, 20);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[5], 0xd62f105d, 5);                                          \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[10], 0x02441453, 9);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[15], 0xd8a1e681, 14);                                        \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[4], 0xe7d3fbc8, 20);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[9], 0x21e1cde6, 5);                                          \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[14], 0xc33707d6, 9);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[3], 0xf4d50d87, 14);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[8], 0x455a14ed, 20);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[13], 0xa9e3e905, 5);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[2], 0xfcefa3f8, 9);                                          \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[7], 0x676f02d9, 14);                                         \
        MD5_FUNC_G((tmpValue), (tmpState), (textFragment)[12], 0x8d2a4c8a, 20);                                        \
    } while (0)

#define MD5_H_PROC(tmpValue, tmpState, textFragment)                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[5], 0xfffa3942, 4);                                          \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[8], 0x8771f681, 11);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[11], 0x6d9d6122, 16);                                        \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[14], 0xfde5380c, 23);                                        \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[1], 0xa4beea44, 4);                                          \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[4], 0x4bdecfa9, 11);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[7], 0xf6bb4b60, 16);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[10], 0xbebfbc70, 23);                                        \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[13], 0x289b7ec6, 4);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[0], 0xeaa127fa, 11);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[3], 0xd4ef3085, 16);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[6], 0x04881d05, 23);                                         \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[9], 0xd9d4d039, 4);                                          \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[12], 0xe6db99e5, 11);                                        \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[15], 0x1fa27cf8, 16);                                        \
        MD5_FUNC_H((tmpValue), (tmpState), (textFragment)[2], 0xc4ac5665, 23);                                         \
    } while (0)

#define MD5_I_PROC(tmpValue, tmpState, textFragment)                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[0], 0xf4292244, 6);                                          \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[7], 0x432aff97, 10);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[14], 0xab9423a7, 15);                                        \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[5], 0xfc93a039, 21);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[12], 0x655b59c3, 6);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[3], 0x8f0ccc92, 10);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[10], 0xffeff47d, 15);                                        \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[1], 0x85845dd1, 21);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[8], 0x6fa87e4f, 6);                                          \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[15], 0xfe2ce6e0, 10);                                        \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[6], 0xa3014314, 15);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[13], 0x4e0811a1, 21);                                        \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[4], 0xf7537e82, 6);                                          \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[11], 0xbd3af235, 10);                                        \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[2], 0x2ad7d2bb, 15);                                         \
        MD5_FUNC_I((tmpValue), (tmpState), (textFragment)[9], 0xeb86d391, 21);                                         \
    } while (0)

static void VOS_MD5CalcDigestOfBuff(MD5_CTX *context)
{
    uint32_t i;
    uint32_t tmpValue;
    uint32_t textFragment[16];
    uint32_t tmpState[4];
    uint8_t *tmpText = context->aucBuffer;
    tmpState[0] = context->aulState[0];
    tmpState[1] = context->aulState[1];
    tmpState[2] = context->aulState[2];
    tmpState[3] = context->aulState[3];
    for (i = 0; i < 16; i += 4)
    {
        textFragment[i] = (uint32_t)(tmpText[0]) + ((uint32_t)(tmpText[1]) << 8) + ((uint32_t)(tmpText[2]) << 16) +
                          ((uint32_t)(tmpText[3]) << 24);
        textFragment[i + 1] = (uint32_t)(tmpText[4]) + ((uint32_t)(tmpText[5]) << 8) + ((uint32_t)(tmpText[6]) << 16) +
                              ((uint32_t)(tmpText[7]) << 24);
        textFragment[i + 2] = (uint32_t)(tmpText[8]) + ((uint32_t)(tmpText[9]) << 8) + ((uint32_t)(tmpText[10]) << 16) +
                              ((uint32_t)(tmpText[11]) << 24);
        textFragment[i + 3] = (uint32_t)(tmpText[12]) + ((uint32_t)(tmpText[13]) << 8) +
                              ((uint32_t)(tmpText[14]) << 16) + ((uint32_t)(tmpText[15]) << 24);
        tmpText += 16;
    }
    MD5_F_PROC(tmpValue, tmpState, textFragment);
    MD5_G_PROC(tmpValue, tmpState, textFragment);
    MD5_H_PROC(tmpValue, tmpState, textFragment);
    MD5_I_PROC(tmpValue, tmpState, textFragment);
    context->aulState[0] += tmpState[0];
    context->aulState[1] += tmpState[1];
    context->aulState[2] += tmpState[2];
    context->aulState[3] += tmpState[3];
}

static bool VOS_MD5PadBuff(MD5_CTX *context)
{
    bool needAnotherBuff = (context->uiPos >= MD5_TEXT_IN_BUFFER_MAX);
    context->aucBuffer[context->uiPos] = 0x80;
    context->uiPos++;
    if (needAnotherBuff)
    {
        while (context->uiPos < MD5_BUFFER_SIZE)
        {
            context->aucBuffer[context->uiPos] = 0;
            context->uiPos++;
        }
    }
    else
    {
        while (context->uiPos < MD5_TEXT_IN_BUFFER_MAX)
        {
            context->aucBuffer[context->uiPos] = 0;
            context->uiPos++;
        }
        MD5_RECORD_MESSAGE_LEN(context);
    }
    return needAnotherBuff;
}

void VOS_MD5Init(MD5_CTX *context)
{
    if (context == NULL)
    {
        return;
    }
    (void)memset_s(context, sizeof(MD5_CTX), 0, sizeof(MD5_CTX));
    context->aulState[0] = 0x67452301;
    context->aulState[1] = 0xefcdab89;
    context->aulState[2] = 0x98badcfe;
    context->aulState[3] = 0x10325476;
}

void VOS_MD5Update(MD5_CTX *context, uint8_t *input, uint32_t inputLen)
{
    uint64_t totalInputBits;
    uint32_t inputIndex = 0;
    uint64_t inputBit;
    uint32_t tmpPos;
    uint8_t *contextBuffer = NULL;
    if ((context == NULL) || ((input == NULL) && (inputLen != 0)))
    {
        return;
    }
    inputBit = (uint64_t)inputLen << 3;
    totalInputBits = ((uint64_t)context->aulCount[1] << 32) + context->aulCount[0];
    if ((MD5_INPUT_LEN_MAX - totalInputBits) < inputBit)
    {
        return;
    }
    totalInputBits += inputBit;
    context->aulCount[0] = (uint32_t)totalInputBits;
    context->aulCount[1] = (uint32_t)(totalInputBits >> 32);
    tmpPos = context->uiPos;
    contextBuffer = context->aucBuffer;
    while (inputIndex < inputLen)
    {
        if (tmpPos < MD5_BUFFER_SIZE)
        {
            contextBuffer[tmpPos] = input[inputIndex];
            ++inputIndex;
            ++tmpPos;
            continue;
        }
        VOS_MD5CalcDigestOfBuff(context);
        tmpPos = 0;
    }
    if (tmpPos == MD5_BUFFER_SIZE)
    {
        VOS_MD5CalcDigestOfBuff(context);
        tmpPos = 0;
    }
    context->uiPos = tmpPos;
    return;
}

void VOS_MD5FinalEx(uint8_t digest[], uint32_t bufLen, MD5_CTX *context)
{
    bool needAnotherBuff = 0;
    if ((digest == NULL) || (context == NULL) || (bufLen < MD5_DIGEST_LEN))
    {
        return;
    }
    needAnotherBuff = VOS_MD5PadBuff(context);
    VOS_MD5CalcDigestOfBuff(context);
    if (needAnotherBuff)
    {
        context->uiPos = 0;
        while (context->uiPos < MD5_TEXT_IN_BUFFER_MAX)
        {
            context->aucBuffer[context->uiPos] = 0;
            context->uiPos++;
        }
        MD5_RECORD_MESSAGE_LEN(context);
        VOS_MD5CalcDigestOfBuff(context);
    }
    MD5_COMPOSE_DIGEST(digest, context->aulState);
    (void)memset_s(context, sizeof(MD5_CTX), 0, sizeof(MD5_CTX));
}

void VOS_MD5Final(uint8_t digest[16], MD5_CTX *context)
{
    VOS_MD5FinalEx(digest, MD5_DIGEST_LEN, context);
}

void VOS_MD5CalcEx(uint8_t *output, uint32_t outputLen, const uint8_t *input, uint32_t inputLen)
{
    MD5_CTX context;
    if (outputLen < MD5_DIGEST_LEN)
    {
        return;
    }
    VOS_MD5Init(&context);
    VOS_MD5Update(&context, (uint8_t *)(uintptr_t)input, inputLen);
    VOS_MD5FinalEx(output, outputLen, &context);
}

void VOS_MD5Calc(uint8_t *output, uint8_t *input, uint32_t inputLen)
{
    VOS_MD5CalcEx(output, MD5_DIGEST_LEN, input, inputLen);
}

