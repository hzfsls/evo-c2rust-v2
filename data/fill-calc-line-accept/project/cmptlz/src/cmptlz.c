typedef struct { uint32_t v; } __attribute__((packed)) CmptlzUnalignU32;

typedef enum
{
    CMPTLZ_ERROR_DATA = 1,
    CMPTLZ_ERROR_MEM,
    CMPTLZ_ERROR_UNSUPPORTED,
    CMPTLZ_ENC_ERROR_FILESIZE,
    CMPTLZ_ENC_CTX_INIT_FAIL,
    CMPTLZ_ENC_RC_INIT_FAIL,
    CMPTLZ_ENC_MF_INIT_FAIL,
    CMPTLZ_ENC_ERROR_WRITE,
    CMPTLZ_ENC_ERROR_HEAD,
    CMPTLZ_ENC_ERROR_PARAM,
    CMPTLZ_ERROR_BUTT
} EnCmptErrNo;

typedef uint16_t CmptLzDecProb;

typedef enum
{
    CMPTLZ_DICT_MEM = 1,
    CMPTLZ_PROB_MEM,
    CMPTLZ_ENC_CCTX,
    CMPTLZ_MF_CCTX,
    CMPTLZ_MF_HASH,
    CMPTLZ_MF_SON,
    CMPTLZ_RC_CCTX,
    CMPTLZ_RC_BUF,
    CMPTLZ_MEM_TYPE_BUT
} EnCmptLzMemType;

typedef struct
{
    void *(*CmptLzAlloc)(int32_t enMemType, size_t size);
    void (*CmptLzFree)(int32_t enMemType, void *address);
} CmptLzMemHook;

typedef void (*CmptlzLogFunc)(const char *message, size_t size);

typedef enum
{
    CMPTLZ_FINISH_ANY,
    CMPTLZ_FINISH_END
} EnCmptLzFinMode;

typedef enum
{
    CMPTLZ_STATUS_NOT_SPECIFIED,
    CMPTLZ_STATUS_FINISHED_WITH_MARK,
    CMPTLZ_STATUS_NOT_FINISHED,
    CMPTLZ_STATUS_NEEDS_MORE_INPUT,
    CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK,
    CMPTLZ_STATUS_BUT
} EnCmptLzStatus;

typedef struct
{
    unsigned char litCtx;
    unsigned char litPos;
    unsigned char posBits;
    unsigned char alignPad;
    uint32_t dicSize;
} CmptLzDecProt;

typedef struct
{
    CmptLzDecProt prop;
    CmptLzDecProb *probs;
    CmptLzDecProb *probsPlus1664;
    unsigned char *dict;
    size_t dictBufSize;
    size_t dictPos;
    const unsigned char *buf;
    uint32_t range;
    uint32_t code;
    uint32_t processedPos;
    uint32_t checkDicSize;
    uint32_t reps[4];
    uint32_t state;
    uint32_t remainLen;
    uint32_t numProbs;
    unsigned tempBufSize;
    unsigned char tempBuf[CMPTLZ_REQUIRED_INPUT_MAX];
} CmptLzDecCtx;

typedef struct
{
    const unsigned char *pSrcIn;
    size_t strInLen;
    size_t strInCostLen;
} CmptLzDecIn;

typedef struct
{
    unsigned char *pDestOut;
    size_t destOutLen;
    size_t destOutFillLen;
} CmptLzDecOut;

typedef struct
{
    const unsigned char *protData;
    unsigned protSize;
    CmptLzMemHook *memHook;
} CmptlzDecParam;

typedef struct TagCmptLzEncCtx CmptLzEncCtx;

typedef struct
{
    int level;
    uint32_t dictSize;
    int litCtx;
    int litPos;
    int posBits;
    int fastBytes;
    int numThreads;
    unsigned char *protData;
    size_t protSize;
    CmptLzMemHook *memHook;
} CmptlzCompParam;

static CmptlzLogFunc g_cmptlzLogFunc = NULL;

typedef uint16_t CmptlzProb;

typedef struct
{
    int level;
    uint32_t dictSize;
    int litCtx;
    int litPos;
    int posBits;
    int fastBytes;
    int numThreads;
} CmptlzEncParam;

typedef struct
{
    uint32_t pos;
    uint32_t prevByte;
    CmptlzProb literal[1 << CMPTLZ_LCLP_MAX][CMPTLZ_LIT_MAX_SIZE];
    uint32_t lcBits;
    uint32_t posMask;
} LitMarcov;

typedef struct
{
    uint32_t range;
    uint64_t cache;
    uint64_t low;
    uint64_t cacheSize;
    uint8_t *buf;
    uint8_t *bufBase;
    uint8_t *outBuf;
    size_t outBufLeft;
} CmptRcCtx;

typedef struct TagCmptMatchFinder CmptMfCtx;

struct TagCmptMatchFinder
{
    const uint8_t *srcStart;
    size_t srcLen;
    uint32_t hashRootTable[256];
    uint32_t mfStart;
    uint32_t niceLen;
    uint32_t readAhead;
    uint32_t readPos;
    uint32_t cyclePos;
    uint32_t cycleSize;
    uint32_t offset;
    uint32_t *hash;
    uint32_t *son;
    uint32_t depth;
    uint32_t hashCount;
    uint32_t sonsCount;
    uint32_t hashMask;
};

typedef struct
{
    CmptlzProb low[256];
    CmptlzProb high[1 << CMPT_LEN_HIGH_BITS];
    uint32_t prices[CMPTLZ_NUM_PB_STATES_MAX]
                   [(1 << CMPT_LEN_HIGH_BITS) + (1 << CMPT_LEN_MID_BITS) + (1 << CMPT_LEN_LOW_BITS)];
    uint32_t tableSize;
} CmptLenEncoder;

typedef enum
{
    LIT_LIT,
    MATCH_LIT_LIT,
    REP_LIT_LIT,
    SHORTREP_LIT_LIT,
    MATCH_LIT,
    REP_LIT,
    SHORTREP_LIT,
    LIT_MATCH,
    LIT_LONGREP,
    LIT_SHORTREP,
    NOTLIT_MATCH,
    NOTLIT_REP,
} CmptlzState;

typedef struct
{
    uint32_t len;
    uint32_t dist;
} CmptlzMatchPair;

typedef struct
{
    CmptlzState state;
    uint32_t price;
    uint32_t posPrev;
    uint32_t backPrev;
    uint32_t backs[CMPTLZ_NUM_REPS];
} CmptlzOpt;

struct TagCmptLzEncCtx
{
    int level;
    int litCtx;
    int litPos;
    int posBits;
    uint32_t dicSize;
    int endMarker;
    uint32_t numFastBytes;
    bool encNeedFinish;
    uint64_t nowpos64;
    uint32_t cmptlzResponse;
    CmptlzState state;
    LitMarcov litMarcov;
    uint32_t reps[CMPTLZ_NUM_REPS];
    CmptlzProb isRep[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG0[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG1[CMPTLZ_NUM_STATES];
    CmptlzProb isRepG2[CMPTLZ_NUM_STATES];
    CmptlzProb isMatch[CMPTLZ_NUM_STATES][CMPTLZ_NUM_PB_STATES_MAX];
    CmptlzProb isRep0Long[CMPTLZ_NUM_STATES][CMPTLZ_NUM_PB_STATES_MAX];
    CmptlzProb probDistSlot[CMPTLZ_DIST_STATE_TOTAL][1 << CMPTLZ_DIST_SLOT_BITS];
    CmptlzProb probDistSpecial[CMPT_DIST_LIMIT_2];
    CmptlzProb probAlign[1 << CMPTLZ_ALIGN_BITS];
    uint32_t posMask;
    uint64_t pbMask;
    uint64_t lpMask;
    CmptRcCtx *rcCtx;
    CmptMfCtx *mfCtx;
    CmptlzMatchPair matches[CMPT_MF_LONGEST_MATCH + 1];
    uint32_t matchesCount;
    uint32_t longestMatchLen;
    uint32_t backRes;
    uint32_t lenRes;
    uint32_t optEndIndex;
    uint32_t optsCurIndex;
    CmptlzOpt opts[CMPT_DP_OPTMAX];
    CmptLenEncoder matchLenEncoder;
    CmptLenEncoder repLenEncoder;
    int repLenPriceCount;
    int matchPriceCount;
    uint32_t priceRootTable[CMPT_PRIICE_TABLE_SIZE];
    uint32_t priceDistSlotTable[CMPTLZ_DIST_STATE_TOTAL][1 << CMPTLZ_DIST_SLOT_BITS];
    uint32_t priceDistTable[CMPTLZ_DIST_STATE_TOTAL][1 << 7];
    uint32_t priceAlignTable[1 << CMPTLZ_ALIGN_BITS];
    uint32_t distTableSize;
};

#define CMPTLZ_MODULE (0x0A00 + 0x0D)

#define CMPT_OK 0

#define CMPT_ERROR_DATA CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_DATA)

#define CMPT_ERROR_MEM CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_MEM)

#define CMPT_ERROR_UNSUPPORTED CMPTLZ_ERROR_CONVERT(CMPTLZ_ERROR_UNSUPPORTED)

#define CMPT_ENC_ERROR_FILESIZE CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_FILESIZE)

#define CMPT_ENC_CTX_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_CTX_INIT_FAIL)

#define CMPT_ENC_RC_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_RC_INIT_FAIL)

#define CMPT_ENC_MF_INIT_FAIL CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_MF_INIT_FAIL)

#define CMPT_ENC_ERROR_WRITE CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_WRITE)

#define CMPT_ENC_ERROR_HEAD CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_HEAD)

#define CMPT_ENC_ERROR_PARAM CMPTLZ_ERROR_CONVERT(CMPTLZ_ENC_ERROR_PARAM)

#define CMPTLZ_PROPS_SIZE 5

#define CMPTLZ_REQUIRED_INPUT_MAX 20

#define CMPTLZ_PROB_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_PROB_MEM)

#define CMPTLZ_ENC_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_ENC_CCTX)

#define CMPTLZ_MF_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_CCTX)

#define CMPTLZ_MF_HASH_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_HASH)

#define CMPTLZ_MF_SON_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_MF_SON)

#define CMPTLZ_RC_CCTX_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_RC_CCTX)

#define CMPTLZ_RC_BUF_HANDLE CMPTLZ_HANDLE_CONVERT(CMPTLZ_RC_BUF)

#define LOG_BUF_SIZE 1024U

#define CMPTLZ_LIT_CTX_MAX 9

#define CMPTLZ_POS_STATE_MAX 5

#define CMPTLZ_LIT_POS_MAX 5

#define CMPTLZ_DEC_INPUT_EOF 1

#define CMPTLZ_DICT_MIN_LEN (1 << 12)

#define CMPTLZ_RANGE_CODE_SIZE 5

#define CMPTLZ_MKSTATE_NUM 12

#define CMPTLZ_LIT_STATES 7

#define CMPTLZ_RANGE_DOWN_LIMIT ((uint32_t)1 << 24)

#define CMPTLZ_ONE_BYTE_WIDTH 8

#define CMPTLZ_PROB_LG_BIT 11

#define CMPTLZ_PROB_LG (1 << CMPTLZ_PROB_LG_BIT)

#define CMPTLZ_PB_STATE_NUM_ALIGN 16

#define CMPTLZ_PB_BITS_MAX 4

#define CMPTLZ_MATCH_MAX_LEN 274

#define CMPTLZ_LOW_LEN_BIT 3

#define CMPTLZ_LOW_LEN_CLASS (1 << CMPTLZ_LOW_LEN_BIT)

#define CMPTLZ_HIGH_LEN_BIT 8

#define CMPTLZ_HIGH_LEN_CLASS (1 << CMPTLZ_HIGH_LEN_BIT)

#define CMPTLZ_LOW_LENPROB_OFFSET 0

#define CMPTLZ_HIGH_LENPROB_OFFSET (CMPTLZ_LOW_LENPROB_OFFSET + ((1 << CMPTLZ_PB_BITS_MAX) << (CMPTLZ_LOW_LEN_BIT + 1)))

#define CMPTLZ_LEN_CHOICE CMPTLZ_LOW_LENPROB_OFFSET

#define CMPTLZ_LEN_CHOICE2 (CMPTLZ_LEN_CHOICE + CMPTLZ_LOW_LEN_CLASS)

#define CMPTLZ_LENPROB_NUM (CMPTLZ_HIGH_LENPROB_OFFSET + CMPTLZ_HIGH_LEN_CLASS)

#define CMPTLZ_LEN_CONDITION_TO_POSSLOT 4

#define CMPTLZ_POS_SLOT_BITS 6

#define CMPTLZ_LOW_POSSLOT 4

#define CMPTLZ_HIGH_POSSLOT 14

#define CMPTLZ_FULL_DISTANCE (1 << (CMPTLZ_HIGH_POSSLOT >> 1))

#define CMPTLZ_LARGE_DIST_LOW_BITS 4

#define CMPTLZ_ALIGN_TABLE_SIZE (1 << CMPTLZ_LARGE_DIST_LOW_BITS)

#define CMPTLZ_OFFSET 1664

#define CMPTLZ_SPEC_POS (-CMPTLZ_OFFSET)

#define CMPTLZ_REP0_LONG (CMPTLZ_SPEC_POS + CMPTLZ_FULL_DISTANCE)

#define CMPTLZ_REP_LEN_CODER (CMPTLZ_REP0_LONG + (CMPTLZ_PB_STATE_NUM_ALIGN << CMPTLZ_PB_BITS_MAX))

#define CMPTLZ_MATCH_LEN_CODER (CMPTLZ_REP_LEN_CODER + CMPTLZ_LENPROB_NUM)

#define CMPTLZ_IS_MATCH (CMPTLZ_MATCH_LEN_CODER + CMPTLZ_LENPROB_NUM)

#define CMPTLZ_ALIGN (CMPTLZ_IS_MATCH + (CMPTLZ_PB_STATE_NUM_ALIGN << CMPTLZ_PB_BITS_MAX))

#define CMPTLZ_ISREP (CMPTLZ_ALIGN + CMPTLZ_ALIGN_TABLE_SIZE)

#define CMPTLZ_ISREPG0 (CMPTLZ_ISREP + CMPTLZ_MKSTATE_NUM)

#define CMPTLZ_ISREPG1 (CMPTLZ_ISREPG0 + CMPTLZ_MKSTATE_NUM)

#define CMPTLZ_ISREPG2 (CMPTLZ_ISREPG1 + CMPTLZ_MKSTATE_NUM)

#define CMPTLZ_POSSLOT (CMPTLZ_ISREPG2 + CMPTLZ_MKSTATE_NUM)

#define CMPTLZ_LITERAL (CMPTLZ_POSSLOT + (CMPTLZ_LEN_CONDITION_TO_POSSLOT << CMPTLZ_POS_SLOT_BITS))

#define NUM_BASE_PROBS (CMPTLZ_LITERAL + CMPTLZ_OFFSET)

#define CMPTLZ_REP4 4

#define CMPTLZ_REP3 3

#define CMPTLZ_REP2 2

#define CMPTLZ_MIN_DICTSIZE (1024)

#define CMPTLZ_MAX_DICTSIZE (128 * 1024 * 1024)

#define CMPTLZ_UINT32_MAX (uint32_t)(-1)

#define CMPT_EMPTY_HASH_VALUE 0

#define CMPTLZ_HASH_2_SIZE (1 << 10)

#define CMPTLZ_HASH_3_SIZE (1 << 16)

#define CMPTLZ_HASH_2_MASK (CMPTLZ_HASH_2_SIZE - 1)

#define CMPTLZ_HASH_3_MASK (CMPTLZ_HASH_3_SIZE - 1)

#define CMPTLZ_FIX_3_HASH (CMPTLZ_HASH_2_SIZE)

#define CMPTLZ_FIX_4_HASH (CMPTLZ_HASH_2_SIZE + CMPTLZ_HASH_3_SIZE)

#define CMPT_RC_MIN_RANGE (1 << 24)

#define CMPT_NUM_LEN_POS_STATE 4

#define CMPTLZ_NUM_REPS 4

#define CMPTLZ_NUM_STATES 12

#define CMPTLZ_MATCH_LEN_MIN 2

#define CMPTLZ_PB_MAX 4

#define CMPTLZ_LC_MAX 8

#define CMPTLZ_LP_MAX 4

#define CMPTLZ_LCLP_MAX 4

#define CMPTLZ_NUM_PB_STATES_MAX (1 << CMPTLZ_PB_MAX)

#define CMPTLZ_LIT_MAX_SIZE 0x300

#define CMPTLZ_PROB_MAX_NUM 2048

#define CMPTLZ_PROB_INIT 1024

#define CMPTLZ_RC_BUFFER_SIZE (1 << 16)

#define CMPT_DIST_LIMIT_2 128

#define CMPTLZ_DIST_STATE_TOTAL 4

#define CMPTLZ_ALIGN_BITS 4

#define CMPTLZ_DIST_SLOT_BITS 6

#define CMPT_INFINITY_PRICE ((uint32_t)1 << 30)

#define CMPT_PRICE_BITS_MOVING_NUM 4

#define CMPT_PRIICE_TABLE_SIZE (CMPTLZ_PROB_MAX_NUM >> CMPT_PRICE_BITS_MOVING_NUM)

#define CMPT_PRICE_COUNT 64

#define CMPT_DOUBLE 2

#define CMPT_LEN_LOW_BITS 3

#define CMPT_LEN_MID_BITS 3

#define CMPT_LEN_HIGH_BITS 8

#define CMPT_LEN_BOUND 8

#define CMPT_MF_LONGEST_MATCH 273

#define CMPT_MF_HASH_TABLE_SIZE 256

#define CMPT_MF_BASE_DEPTH 16

#define CMPT_MF_MATCH_2_BYTES 2

#define CMPT_MF_MATCH_3_BYTES 3

#define CMPT_DP_OPTMAX (1 << 11)

#define CMPT_ONE_BLOCK_MAX_SIZE (1 << 17)

#define CMPTLZ_WRITE32BIT(ptr, val) (((CmptlzUnalignU32 *)(ptr))->v = (val))


#define CMPTLZ_ERROR_CONVERT(x) (int32_t)((CMPTLZ_MODULE << 16) | (uint32_t)(x))


#define CMPTLZ_HANDLE_CONVERT(x) (int32_t)((CMPTLZ_MODULE << 16) | ((uint32_t)(x) << 8))


#define CMPTLZ_LIKELY(expr) __builtin_expect(expr, true)

#define CMPTLZ_UNLIKELY(expr) __builtin_expect(expr, false)

#define CMPTLZ_LOG(error_code, fmt, args...)                                                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        CmptlzLogWrite((size_t)(error_code), __FUNCTION__, __LINE__, fmt, ##args);                                     \
    } while (0)

#define CMPTLZ_CALC_POS_STATE(procPos, pbMask) (((procPos) & (pbMask)) << 4)

#define CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec)                                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        if ((range) < CMPTLZ_RANGE_DOWN_LIMIT)                                                                         \
        {                                                                                                              \
            (range) <<= CMPTLZ_ONE_BYTE_WIDTH;                                                                         \
            (rangeCode) <<= CMPTLZ_ONE_BYTE_WIDTH;                                                                     \
            (rangeCode) |= (*(bufToDec)++);                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)                                                    \
    (rangeBound) = (range >> CMPTLZ_PROB_LG_BIT) * (*(probSlot));                                                      \
    if ((rangeCode) < (rangeBound))                                                                                    \


#define CMPTLZ_RANGE_UPDATE_0(prob, range, rangeBound)                                                             \
    do {                                                                                                           \
        (range) = (rangeBound);                                                                                    \
        *(prob) = (CmptLzDecProb)((*(prob)) + ((CMPTLZ_PROB_LG - (*(prob))) >> CMPTLZ_RANGE_CODE_SIZE));           \
    } while (0)

#define CMPTLZ_RANGE_UPDATE_1(prob, range, rangeCode, rangeBound)                                                      \
    do                                                                                                                 \
    {                                                                                                                  \
        (range) -= (rangeBound);                                                                                       \
        (rangeCode) -= (rangeBound);                                                                                   \
        *(prob) = (CmptLzDecProb)((*(prob)) - ((*(prob)) >> CMPTLZ_RANGE_CODE_SIZE));                                  \
    } while (0)

#define CMPTLZ_NORMAL_BIT_DEC(probLit, range, rangeCode, rangeBound, decSym)                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probLit));                                                 \
        if ((rangeCode) < (rangeBound))                                                                                \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_0(probLit, range, rangeBound);                                                         \
            (decSym) = ((decSym) << 1);                                                                                \
        }                                                                                                              \
        else                                                                                                           \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_1(probLit, range, rangeCode, rangeBound);                                              \
            (decSym) = ((decSym) << 1) + 1;                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_MATCH_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec)          \
    do                                                                                                                 \
    {                                                                                                                  \
        (matchSym) <<= 1;                                                                                              \
        (bit) = (offset);                                                                                              \
        (offset) &= (matchSym);                                                                                        \
        (probLit) = (probSlot) + ((offset) + (bit) + (decSym));                                                        \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probLit));                                                 \
        if ((rangeCode) < (rangeBound))                                                                                \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_0(probLit, range, rangeBound);                                                         \
            (decSym) = ((decSym) << 1);                                                                                \
            (offset) ^= (bit);                                                                                         \
        }                                                                                                              \
        else                                                                                                           \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_1(probLit, range, rangeCode, rangeBound);                                              \
            (decSym) = ((decSym) << 1) + 1;                                                                            \
        }                                                                                                              \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);                                                            \
    } while (0)

#define CMPTLZ_DIST_BIT_DEC(probDist, probSlot, range, rangeCode, rangeBound, decDist, decBit)                         \
    do                                                                                                                 \
    {                                                                                                                  \
        (probDist) = (probSlot) + (decDist);                                                                           \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probDist));                                                \
        if ((rangeCode) < (rangeBound))                                                                                \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_0(probDist, range, rangeBound);                                                        \
            (decDist) += (decBit);                                                                                     \
        }                                                                                                              \
        else                                                                                                           \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_1(probDist, range, rangeCode, rangeBound);                                             \
            (decDist) += (decBit) * 2;                                                                                 \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_LEN_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, bufToDec)                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        CMPTLZ_NORMAL_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym);                                         \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);                                                            \
    } while (0)

#define CMPTLZ_POSSLOT_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, bufToDec)                               \
    do                                                                                                                 \
    {                                                                                                                  \
        CMPTLZ_NORMAL_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym);                                         \
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);                                                            \
    } while (0)

#define CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound)                                                          \
    do                                                                                                                 \
    {                                                                                                                  \
        (range) = (rangeBound);                                                                                        \
    } while (0)

#define CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound)                                               \
    do                                                                                                                 \
    {                                                                                                                  \
        (range) -= (rangeBound);                                                                                       \
        (rangeCode) -= (rangeBound);                                                                                   \
    } while (0)

#define CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit)                                              \
    do                                                                                                                 \
    {                                                                                                                  \
        if ((range) < CMPTLZ_RANGE_DOWN_LIMIT)                                                                         \
        {                                                                                                              \
            if ((bufTryDec) >= (bufLimit))                                                                             \
            {                                                                                                          \
                return CMPTLZ_DEC_INPUT_EOF;                                                                           \
            }                                                                                                          \
            (range) <<= CMPTLZ_ONE_BYTE_WIDTH;                                                                         \
            (rangeCode) <<= CMPTLZ_ONE_BYTE_WIDTH;                                                                     \
            (rangeCode) |= (*(bufTryDec)++);                                                                           \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probSym)                                       \
    do                                                                                                                 \
    {                                                                                                                  \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probSym));                                                 \
        if ((rangeCode) < (rangeBound))                                                                                \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);                                                     \
            (decSym) = ((decSym) << 1);                                                                                \
        }                                                                                                              \
        else                                                                                                           \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);                                          \
            (decSym) = ((decSym) << 1) + 1;                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_MATCH_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probSym)                                        \
    do                                                                                                                 \
    {                                                                                                                  \
        (rangeBound) = ((range) >> CMPTLZ_PROB_LG_BIT) * (*(probSym));                                                 \
        if ((rangeCode) < (rangeBound))                                                                                \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);                                                     \
            (decSym) = ((decSym) << 1);                                                                                \
            (offset) ^= (bit);                                                                                         \
        }                                                                                                              \
        else                                                                                                           \
        {                                                                                                              \
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);                                          \
            (decSym) = ((decSym) << 1) + 1;                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPTLZ_SET_DICTSIZE_BY_LEVEL(level, dictSize)                                                                  \
    do                                                                                                                 \
    {                                                                                                                  \
        dictSize = (level <= 5) ? (1 << (level * 2 + 14)) : ((level <= 7) ? (1 << 25) : (1 << 26));                    \
    } while (0)

#define CMPTLZ_SET_FB_BY_LEVEL(level, fastBytes) (fastBytes = ((level < 7) ? 32 : 64))

#define CMPTLZ_FIND_MIN(x, y) ((x) < (y) ? (x) : (y))

#define CMPTLZ_FIND_MAX(x, y) ((x) > (y) ? (x) : (y))

#define NOT_EQUAL_2_BYTES(a, b) ((a)[0] != (b)[0] || (a)[1] != (b)[1])

#define CMPTLZ_RETURN_IF_NOT_OK(res)                                                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        if (CMPTLZ_UNLIKELY(res != CMPT_OK))                                                                           \
        {                                                                                                              \
            return res;                                                                                                \
        }                                                                                                              \
    } while (0)

#define CMPT_GET_DIST_STATE(len) (((len) < 4 + CMPTLZ_MATCH_LEN_MIN) ? (len) - CMPTLZ_MATCH_LEN_MIN : 4 - 1)

#define CMPT_STATE_UPDATE_WHEN_LIT(state)                                                                              \
    (state) = (((state) <= SHORTREP_LIT_LIT) ? LIT_LIT : (((state) <= LIT_SHORTREP) ? (state) - 3 : (state) - 6))

#define CMPT_STATE_UPDATE_WHEN_MATCH(state) (state) = (((state) < 7) ? LIT_MATCH : NOTLIT_MATCH)

#define CMPT_STATE_UPDATE_WHEN_LONGREP(state) (state) = (((state) < 7) ? LIT_LONGREP : NOTLIT_REP)

#define CMPT_STATE_UPDATE_WHEN_SHORTREP(state) (state) = (((state) < 7) ? LIT_SHORTREP : NOTLIT_REP)

#define CMPT_HASH_MASK_CALC(hashMask)                                                                                  \
    do                                                                                                                 \
    {                                                                                                                  \
        hashMask |= hashMask >> 1;                                                                                     \
        hashMask |= hashMask >> 2;                                                                                     \
        hashMask |= hashMask >> 4;                                                                                     \
        hashMask |= hashMask >> 8;                                                                                     \
        hashMask >>= 1;                                                                                                \
        hashMask |= 0xFFFF;                                                                                            \
        if (hashMask > (1 << 24))                                                                                      \
        {                                                                                                              \
            hashMask >>= 1;                                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue)                                             \
    do                                                                                                                 \
    {                                                                                                                  \
        temp = mf->hashRootTable[cur[0]] ^ cur[1];                                                                     \
        hash2Value = temp & CMPTLZ_HASH_2_MASK;                                                                        \
        hash3Value = (temp ^ ((uint32_t)(cur[2]) << 8)) & CMPTLZ_HASH_3_MASK;                                          \
        hashValue = (temp ^ ((uint32_t)(cur[2]) << 8) ^ (mf->hashRootTable[cur[3]] << 5)) & mf->hashMask;              \
    } while (0)

#define CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos)                                                   \
    do                                                                                                                 \
    {                                                                                                                  \
        mf->hash[hash2Value] = pos;                                                                                    \
        mf->hash[CMPTLZ_FIX_3_HASH + hash3Value] = pos;                                                                \
        mf->hash[CMPTLZ_FIX_4_HASH + hashValue] = pos;                                                                 \
    } while (0)

#define CMPT_HASH_FIND_2_BYTES(mf, delta2, longestLen, matchesCount, cur, matches)                                     \
    do                                                                                                                 \
    {                                                                                                                  \
        if (delta2 < mf->cycleSize && *(cur - delta2) == *cur)                                                         \
        {                                                                                                              \
            longestLen = CMPT_MF_MATCH_2_BYTES;                                                                        \
            matches[0].len = CMPT_MF_MATCH_2_BYTES;                                                                    \
            matches[0].dist = delta2 - 1;                                                                              \
            matchesCount = 1;                                                                                          \
        }                                                                                                              \
    } while (0)

#define CMPT_HASH_FIND_3_BYTES(mf, delta2, delta3, longestLen, matchesCount, cur, matches)                             \
    do                                                                                                                 \
    {                                                                                                                  \
        if (delta2 != delta3 && delta3 < mf->cycleSize && *(cur - delta3) == *cur)                                     \
        {                                                                                                              \
            longestLen = CMPT_MF_MATCH_3_BYTES;                                                                        \
            matches[matchesCount++].dist = delta3 - 1;                                                                 \
            delta2 = delta3;                                                                                           \
        }                                                                                                              \
    } while (0)

#define CMPT_MF_MOVE_POS(mf)                                                                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        mf->readPos++;                                                                                                 \
        mf->cyclePos++;                                                                                                \
        mf->cyclePos = (mf->cyclePos == mf->cycleSize) ? 0 : mf->cyclePos;                                             \
        if (CMPTLZ_UNLIKELY(mf->readPos + mf->offset == CMPTLZ_UINT32_MAX))                                            \
        {                                                                                                              \
            CmptMfMovePos(mf);                                                                                         \
        }                                                                                                              \
    } while (0)

#define CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len)                                                       \
    do                                                                                                                 \
    {                                                                                                                  \
        *ptr1 = curMatch;                                                                                              \
        ptr1 = pair + 1;                                                                                               \
        curMatch = *ptr1;                                                                                              \
        len1 = len;                                                                                                    \
    } while (0)

#define CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len)                                                      \
    do                                                                                                                 \
    {                                                                                                                  \
        *ptr0 = curMatch;                                                                                              \
        ptr0 = pair;                                                                                                   \
        curMatch = *ptr0;                                                                                              \
        len0 = len;                                                                                                    \
    } while (0)

#define CMPT_LIT_SUBCODER(probs, litCtx, lpMask, pos, prevByte)                                                        \
    ((probs)[(((pos) & (lpMask)) << (litCtx)) + ((uint32_t)(prevByte) >> (8U - (litCtx)))])

#define GET_LEN_TO_POS_STATE(len) (((len) < CMPT_NUM_LEN_POS_STATE + 1) ? (len) - 2 : CMPT_NUM_LEN_POS_STATE - 1)

#define CMPT_RC_BREAK_CHECK(rcCtx, buf, res)                                                                           \
    do                                                                                                                 \
    {                                                                                                                  \
        if ((buf) == (rcCtx->bufBase + CMPTLZ_RC_BUFFER_SIZE))                                                         \
        {                                                                                                              \
            (res) = CmptRcFlush64Kb(rcCtx);                                                                            \
            CMPTLZ_RETURN_IF_NOT_OK(res);                                                                              \
        }                                                                                                              \
    } while (0)

#define CMPT_RC_BREAK_SHIFTING(rcCtx, buf, res)                                                                        \
    do                                                                                                                 \
    {                                                                                                                  \
        CMPT_RC_BREAK_CHECK(rcCtx, buf, res);                                                                          \
        if ((rcCtx)->cacheSize == 0)                                                                                   \
        {                                                                                                              \
            return CMPT_OK;                                                                                            \
        }                                                                                                              \
    } while (0)

#define CMPT_RC_NORMALIZE(rcCtx, range, shiftRes)                                                                      \
    do                                                                                                                 \
    {                                                                                                                  \
        if ((range) < CMPT_RC_MIN_RANGE)                                                                               \
        {                                                                                                              \
            (range) <<= 8;                                                                                             \
            (shiftRes) = CmptRcShiftLow(rcCtx);                                                                        \
        }                                                                                                              \
    } while (0)

#define CMPT_RC_GET_NEWBOUND(prob, bit0Prob, range, newBound)                                                          \
    do                                                                                                                 \
    {                                                                                                                  \
        (bit0Prob) = *(prob);                                                                                          \
        newBound = ((range) >> 11) * (bit0Prob);                                                                       \
    } while (0)

#define CMPT_RC_BIT_PROCESS(rcCtx, prob, bit, bit0Prob, range, newBound, shiftRes)                                     \
    {                                                                                                                  \
        do                                                                                                             \
        {                                                                                                              \
            uint32_t mask = 0 - (uint32_t)(bit);                                                                       \
            CMPT_RC_GET_NEWBOUND(prob, bit0Prob, range, newBound);                                                     \
            (range) &= mask;                                                                                           \
            mask &= (newBound);                                                                                        \
            (range) -= mask;                                                                                           \
            (rcCtx)->low += mask;                                                                                      \
            mask = (uint32_t)(bit) - 1;                                                                                \
            (range) += (newBound) & mask;                                                                              \
            mask &= (CMPTLZ_PROB_MAX_NUM - ((1 << 5) - 1));                                                            \
            mask += ((1 << 5) - 1);                                                                                    \
            (bit0Prob) += (int)(mask - (bit0Prob)) >> 5;                                                               \
            *(prob) = (CmptlzProb)(bit0Prob);                                                                          \
            CMPT_RC_NORMALIZE(rcCtx, range, shiftRes);                                                                 \
        } while (0);                                                                                                   \
    }

#define CMPT_RC_BIT_0(prob, newBound, range, bit0Prob)                                                                 \
    do                                                                                                                 \
    {                                                                                                                  \
        (range) = (newBound);                                                                                          \
        *(prob) = (CmptlzProb)((bit0Prob) + ((CMPTLZ_PROB_MAX_NUM - (bit0Prob)) >> 5));                                \
    } while (0)

#define CMPT_RC_BIT_1(rcCtx, prob, newBound, range, bit0Prob)                                                          \
    do                                                                                                                 \
    {                                                                                                                  \
        (range) -= (newBound);                                                                                         \
        (rcCtx)->low += (newBound);                                                                                    \
        *(prob) = (CmptlzProb)((bit0Prob) - ((bit0Prob) >> 5));                                                        \
    } while (0)

#define CMPT_RC_BIT_0_PROCESS(rcCtx, prob, newBound, range, bit0Prob, shiftRes)                                        \
    do                                                                                                                 \
    {                                                                                                                  \
        CMPT_RC_BIT_0(prob, newBound, range, bit0Prob);                                                                \
        CMPT_RC_NORMALIZE(rcCtx, range, shiftRes);                                                                     \
    } while (0)

#define CMPT_RC_BIT_1_PROCESS(rcCtx, prob, newBound, range, bit0Prob, shiftRes)                                        \
    do                                                                                                                 \
    {                                                                                                                  \
        CMPT_RC_BIT_1(rcCtx, prob, newBound, range, bit0Prob);                                                         \
        CMPT_RC_NORMALIZE(rcCtx, range, shiftRes);                                                                     \
    } while (0)

#define CMPT_LIT_PROB_GET(encCtx, litProb, pos, prevByte)                                                              \
    (litProb + (uint32_t)3 * (((((pos) << 8) + (prevByte)) & encCtx->lpMask) << encCtx->litMarcov.lcBits))

static int CmptlzIsLE(void)
{
    int n = 1;
    return *(char *)(&n);
}

static uint32_t CmptlzSwap32(uint32_t val)
{
    return ((0xff000000 & (val << 24)) | 
            (0x000000ff & (val >> 24)) |
            (0x00ff0000 & (val <<  8)) |
            (0x0000ff00 & (val >>  8)));
}

static void CmptlzWriteLE32Bit(void *addr, uint32_t val)
{
    if (CmptlzIsLE() != 0) {
        CMPTLZ_WRITE32BIT(addr, val);
    } else {
        CMPTLZ_WRITE32BIT(addr, CmptlzSwap32(val));
    }
}

void CmptlzLogWrite(size_t errorCode, const char *funcName, unsigned short line, const char *fmt, ...)
{
    va_list alist;
    char output[LOG_BUF_SIZE];
    int ret;
    size_t len;
    CmptlzLogFunc func = g_cmptlzLogFunc;
    if (func == NULL)
    {
        return;
    }
    ret = snprintf_s(output, LOG_BUF_SIZE, LOG_BUF_SIZE - 1, "\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\n", funcName,
                     line, errorCode);
    if (ret < 0)
    {
        return;
    }
    len = (size_t)ret;
    va_start(alist, fmt);
    ret = vsnprintf_s(output + len, LOG_BUF_SIZE - len, LOG_BUF_SIZE - len - 1, fmt, alist);
    va_end(alist);
    if (ret < 0)
    {
        return;
    }
    func(output, strlen(output) + 1);
}

void CmptlzLogRegister(CmptlzLogFunc func)
{
    g_cmptlzLogFunc = func;
}

static int CmptLzPropsDecode(const unsigned char *protData, unsigned protSize, CmptLzDecProt *decProt)
{
    uint32_t dictSize;
    if (protSize < CMPTLZ_PROPS_SIZE)
    {
        return CMPT_ERROR_UNSUPPORTED;
    }
    else
    {
        dictSize =
            protData[1] | ((uint32_t)protData[2] << 8) | ((uint32_t)protData[3] << 16) | ((uint32_t)protData[4] << 24);
    }
    if (dictSize < CMPTLZ_DICT_MIN_LEN)
    {
        dictSize = CMPTLZ_DICT_MIN_LEN;
    }
    decProt->dicSize = dictSize;
    unsigned char firstData = protData[0];
    if (firstData >= (CMPTLZ_LIT_CTX_MAX * CMPTLZ_POS_STATE_MAX * CMPTLZ_LIT_POS_MAX))
    {
        return CMPT_ERROR_UNSUPPORTED;
    }
    decProt->litCtx = (unsigned char)(firstData % CMPTLZ_LIT_CTX_MAX);
    firstData /= CMPTLZ_LIT_CTX_MAX;
    decProt->posBits = (unsigned char)(firstData / CMPTLZ_POS_STATE_MAX);
    decProt->litPos = (unsigned char)(firstData % CMPTLZ_LIT_POS_MAX);
    return CMPT_OK;
}

void CmptLzDecInit(CmptLzDecCtx *decCtx)
{
    decCtx->dictPos = 0;
    decCtx->tempBufSize = 0;
    decCtx->processedPos = 0;
    decCtx->checkDicSize = 0;
    decCtx->remainLen = CMPTLZ_MATCH_MAX_LEN + 2;
}

static void *CmptLzDecMemAlloc(CmptLzMemHook *memHook, int32_t memHandle, size_t allocSize)
{
    return memHook->CmptLzAlloc(memHandle, allocSize);
}

static void CmptLzDecMemFree(CmptLzMemHook *memHook, int32_t memHandle, void *freeAddress)
{
    memHook->CmptLzFree(memHandle, freeAddress);
}

static void CmptLzDecFreeProbs(CmptLzDecCtx *decCtx, CmptLzMemHook *memHook)
{
    if (decCtx->probs != NULL)
    {
        CmptLzDecMemFree(memHook, CMPTLZ_PROB_HANDLE, decCtx->probs);
        decCtx->probs = NULL;
    }
}

static int CmptLzDecAllocateProbs(CmptLzDecCtx *decCtx, CmptLzDecProt *decProt, CmptLzMemHook *memHook)
{
    uint32_t numProbs = CmptLzGetNumProbs(decProt);
    if (decCtx->probs == NULL)
    {
        decCtx->probs =
            (CmptLzDecProb *)CmptLzDecMemAlloc(memHook, CMPTLZ_PROB_HANDLE, numProbs * sizeof(CmptLzDecProb));
    }
    else
    {
        if (numProbs != decCtx->numProbs)
        {
            CmptLzDecFreeProbs(decCtx, memHook);
            decCtx->probs =
                (CmptLzDecProb *)CmptLzDecMemAlloc(memHook, CMPTLZ_PROB_HANDLE, numProbs * sizeof(CmptLzDecProb));
        }
    }
    if (decCtx->probs == NULL)
    {
        return CMPT_ERROR_MEM;
    }
    decCtx->probsPlus1664 = decCtx->probs + 1664;
    decCtx->numProbs = numProbs;
    return CMPT_OK;
}

void CmptLzDecConstruct(CmptLzDecCtx *decCtx)
{
    decCtx->dict = NULL;
    decCtx->probs = NULL;
}

int CmptLzDecode(CmptLzDecIn *pDecIn, CmptLzDecOut *pDecOut, const unsigned char *protData, EnCmptLzFinMode finMode,
                 EnCmptLzStatus *finStatus, CmptLzMemHook *memHook)
{
    int res;
    size_t inSize = pDecIn->strInLen;
    CmptLzDecProt decProt;
    CmptLzDecCtx decCtx;
    decCtx.numProbs = 0;
    if (inSize < CMPTLZ_PROPS_SIZE)
    {
        return CMPT_ERROR_UNSUPPORTED;
    }
    CmptLzDecConstruct(&decCtx);
    res = CmptLzPropsDecode(protData, CMPTLZ_PROPS_SIZE, &decProt);
    if (res != CMPT_OK)
    {
        return res;
    }
    res = CmptLzDecAllocateProbs(&decCtx, &decProt, memHook);
    if (res != CMPT_OK)
    {
        return res;
    }
    decCtx.prop = decProt;
    decCtx.dict = pDecOut->pDestOut;
    decCtx.dictBufSize = pDecOut->destOutLen;
    CmptLzDecInit(&decCtx);
    *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
    res = CmptLzDecDecodeToDic(&decCtx, pDecOut->destOutLen, pDecIn->pSrcIn, &inSize, finMode, finStatus);
    pDecIn->strInCostLen = inSize;
    pDecOut->destOutFillLen = decCtx.dictPos;
    CmptLzDecFreeProbs(&decCtx, memHook);
    return res;
}

static CmptLzDecProb *CmptLzGetProbsMatrix(CmptLzDecCtx *decCtx)
{
    return decCtx->probsPlus1664;
}

static CmptLzDecProb *CmptLzGetIsMatchProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_IS_MATCH;
}

static CmptLzDecProb *CmptLzGetIsRepProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREP;
}

static CmptLzDecProb *CmptLzGetIsRepG0Prob(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREPG0;
}

static CmptLzDecProb *CmptLzGetIsRepG1Prob(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREPG1;
}

static CmptLzDecProb *CmptLzGetIsRepG2Prob(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ISREPG2;
}

static CmptLzDecProb *CmptLzGetIsRepG0LongProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_REP0_LONG;
}

static CmptLzDecProb *CmptLzGetLiteralProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_LITERAL;
}

static CmptLzDecProb *CmptLzGetPosSlotProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_POSSLOT;
}

static CmptLzDecProb *CmptLzGetSpecPosProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_SPEC_POS;
}

static CmptLzDecProb *CmptLzGetAilgnProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_ALIGN;
}

static CmptLzDecProb *CmptLzGetRepLenCoderProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_REP_LEN_CODER;
}

static CmptLzDecProb *CmptLzGetMatchLenCoderProb(CmptLzDecProb *probsMatrix)
{
    return probsMatrix + CMPTLZ_MATCH_LEN_CODER;
}

static uint32_t CmptLzGetLenCondition(uint32_t decLen)
{
    return ((decLen < CMPTLZ_LEN_CONDITION_TO_POSSLOT ? decLen : CMPTLZ_LEN_CONDITION_TO_POSSLOT - 1)
            << CMPTLZ_POS_SLOT_BITS);
}

static uint32_t CmptLzGetBaseDistByPosSlot(uint32_t posSlot)
{
    return (2 | (posSlot & 1));
}

static uint32_t CmptLzGetNumProbs(CmptLzDecProt *decProt)
{
    return (NUM_BASE_PROBS + ((uint32_t)0x300 << (decProt->litCtx + decProt->litPos)));
}

static void CmptLzDistDecHelper(CmptLzDecCtx *decCtx, uint32_t distDec, const unsigned char *bufToDec, uint32_t *pRange,
                                uint32_t *pRangeCode, uint32_t *pRangeBound, uint32_t range, uint32_t rangeCode,
                                uint32_t rangeBound)
{
    decCtx->reps[CMPTLZ_REP3] = decCtx->reps[CMPTLZ_REP2];
    decCtx->reps[CMPTLZ_REP2] = decCtx->reps[1];
    decCtx->reps[1] = decCtx->reps[0];
    decCtx->reps[0] = (distDec + 1);
    decCtx->buf = bufToDec;
    decCtx->state = (decCtx->state < CMPTLZ_LIT_STATES) ? CMPTLZ_LIT_STATES : CMPTLZ_LIT_STATES + CMPTLZ_REP3;
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
}

static size_t CmptLzDistDec(CmptLzDecCtx *decCtx, CmptLzDecProb *probsMatrix, uint32_t *pRange, uint32_t *pRangeCode,
                            uint32_t *pRangeBound, uint32_t decLen)
{
    uint32_t assistBits;
    uint32_t posSlot = 1;
    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;
    const unsigned char *bufToDec = decCtx->buf;
    uint32_t distDec;
    CmptLzDecProb *probPosSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decLen);
    int i = 0;
    for (i = 0; i < CMPTLZ_POS_SLOT_BITS; i++)
    {
        CMPTLZ_POSSLOT_BIT_DEC((probPosSlot + posSlot), range, rangeCode, rangeBound, posSlot, bufToDec);
    }
    posSlot -= 64;
    if (posSlot < CMPTLZ_LOW_POSSLOT)
    {
        distDec = posSlot;
        CmptLzDistDecHelper(decCtx, distDec, bufToDec, pRange, pRangeCode, pRangeBound, range, rangeCode, rangeBound);
        if (distDec == (size_t)0xFFFFFFFF)
        {
            return distDec;
        }
        else
        {
            return (distDec + 1);
        }
    }
    uint32_t directBitNum = ((posSlot >> 1) - 1);
    distDec = CmptLzGetBaseDistByPosSlot(posSlot);
    if (posSlot < CMPTLZ_HIGH_POSSLOT)
    {
        assistBits = 1;
        distDec <<= directBitNum;
        distDec += assistBits;
        probPosSlot = CmptLzGetSpecPosProb(probsMatrix);
        do
        {
            if CMPTLZ_IS_THE_BIT_0((probPosSlot + distDec), range, rangeCode, rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_0((probPosSlot + distDec), range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                distDec += assistBits;
                assistBits <<= 1;
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_1((probPosSlot + distDec), range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                assistBits <<= 1;
                distDec += assistBits;
            }
        } while (--directBitNum);
        distDec -= assistBits;
    }
    else
    {
        directBitNum -= CMPTLZ_REP4;
        do
        {
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            range >>= 1;
            rangeCode -= range;
            assistBits = (0 - ((uint32_t)rangeCode >> 31));
            distDec = (distDec << 1) + (assistBits + 1);
            rangeCode += range & assistBits;
        } while (--directBitNum);
        CmptLzDecProb *probDist;
        probPosSlot = CmptLzGetAilgnProb(probsMatrix);
        distDec <<= CMPTLZ_LARGE_DIST_LOW_BITS;
        assistBits = 1;
        uint32_t cycleSym = 1;
        for (i = 0; i < 3; i++)
        {
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            CMPTLZ_DIST_BIT_DEC(probDist, probPosSlot, range, rangeCode, rangeBound, assistBits, cycleSym);
            cycleSym <<= 1;
        }
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probDist = probPosSlot + assistBits;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probDist);
        if (rangeCode < rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_0(probDist, range, rangeBound);
            assistBits -= 8;
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_1(probDist, range, rangeCode, rangeBound);
        }
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        distDec |= assistBits;
    }
    CmptLzDistDecHelper(decCtx, distDec, bufToDec, pRange, pRangeCode, pRangeBound, range, rangeCode, rangeBound);
    if (distDec == (size_t)0xFFFFFFFF)
    {
        return distDec;
    }
    else
    {
        return (distDec + 1);
    }
}

static uint32_t CmptLzLenDec(CmptLzDecCtx *decCtx, CmptLzDecProb *probSlot, uint32_t *pRange, uint32_t *pRangeCode,
                             uint32_t *pRangeBound, uint32_t posState)
{
    uint32_t decLen = 1;
    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;
    const unsigned char *bufToDec = decCtx->buf;
    CmptLzDecProb *probLen = probSlot + CMPTLZ_LEN_CHOICE;
    int i = 0;
    if CMPTLZ_IS_THE_BIT_0(probLen, range, rangeCode, rangeBound)
    {
        CMPTLZ_RANGE_UPDATE_0(probLen, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET + posState;
        for (i = 0; i < CMPTLZ_LOW_LEN_BIT; i++)
        {
            CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
        }
        decLen -= 8;
    }
    else
    {
        CMPTLZ_RANGE_UPDATE_1(probLen, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probLen = probSlot + CMPTLZ_LEN_CHOICE2;
        if CMPTLZ_IS_THE_BIT_0(probLen, range, rangeCode, rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_0(probLen, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            probLen = probSlot + (CMPTLZ_LEN_CHOICE2 + posState);
            for (i = 0; i < CMPTLZ_LOW_LEN_BIT; i++)
            {
                CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            }
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_1(probLen, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET;
            for (i = 0; i < CMPTLZ_HIGH_LEN_BIT; i++)
            {
                CMPTLZ_LEN_BIT_DEC((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            }
            decLen -= CMPTLZ_HIGH_LEN_CLASS;
            decLen += (CMPTLZ_LOW_LEN_CLASS << 1);
        }
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx->buf = bufToDec;
    return decLen;
}

static uint32_t CmptLzDecByDistAndLen(CmptLzDecCtx *decCtx, size_t matchDist, uint32_t matchLen, size_t dicPosLimit)
{
    size_t dicCopyPos;
    size_t dicPos = decCtx->dictPos;
    size_t dictBufSize = decCtx->dictBufSize;
    uint32_t remainDicLen = (uint32_t)(dicPosLimit - dicPos);
    unsigned char *dict = decCtx->dict;
    if (remainDicLen == 0)
    {
        return CMPT_ERROR_DATA;
    }
    uint32_t decDicLen = ((remainDicLen < matchLen) ? remainDicLen : matchLen);
    decCtx->processedPos += decDicLen;
    decCtx->dictPos += decDicLen;
    decCtx->remainLen = matchLen - decDicLen;
    if (dicPos < matchDist)
    {
        dicCopyPos = dictBufSize - matchDist + dicPos;
    }
    else
    {
        dicCopyPos = dicPos - matchDist;
    }
    do
    {
        dict[dicPos++] = dict[dicCopyPos];
        if (++dicCopyPos == dictBufSize)
        {
            dicCopyPos = 0;
        }
    } while (--decDicLen != 0);
    return CMPT_OK;
}

static void CmptLzShortRepDec(CmptLzDecCtx *decCtx)
{
    uint32_t rep0 = decCtx->reps[0];
    unsigned char *dict = decCtx->dict;
    size_t dictPos = decCtx->dictPos;
    size_t dictBufSize = decCtx->dictBufSize;
    dict[dictPos] = dict[dictPos - rep0 + (dictPos < rep0 ? dictBufSize : 0)];
    decCtx->dictPos++;
    decCtx->processedPos++;
    if (decCtx->state < CMPTLZ_LIT_STATES)
    {
        decCtx->state = 9;
    }
    else
    {
        decCtx->state = 11;
    }
}

static uint32_t CmptLzRepDec(CmptLzDecCtx *decCtx, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound,
                             size_t dicPosLimit, uint32_t posState)
{
    uint32_t repLen;
    uint32_t repDist;
    uint32_t mkState = decCtx->state;
    const unsigned char *bufToDec = decCtx->buf;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    uint32_t range = *pRange;
    uint32_t rangeCode = *pRangeCode;
    uint32_t rangeBound = *pRangeBound;
    probSlot = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
    if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
    {
        CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            *pRange = range;
            *pRangeCode = rangeCode;
            *pRangeBound = rangeBound;
            decCtx->buf = bufToDec;
            CmptLzShortRepDec(decCtx);
            return CMPT_OK;
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            repDist = decCtx->reps[0];
        }
    }
    else
    {
        CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        probSlot = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            repDist = decCtx->reps[1];
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
            probSlot = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
            if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                repDist = decCtx->reps[CMPTLZ_REP2];
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
                repDist = decCtx->reps[CMPTLZ_REP3];
                decCtx->reps[CMPTLZ_REP3] = decCtx->reps[CMPTLZ_REP2];
            }
            decCtx->reps[CMPTLZ_REP2] = decCtx->reps[1];
        }
        decCtx->reps[1] = decCtx->reps[0];
        decCtx->reps[0] = repDist;
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx->buf = bufToDec;
    decCtx->state = (mkState < CMPTLZ_LIT_STATES) ? 8 : 11;
    probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
    repLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    return CmptLzDecByDistAndLen(decCtx, repDist, repLen + 2, dicPosLimit);
}

static uint32_t CmptLzMatchDec(CmptLzDecCtx *decCtx, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound,
                               size_t dicPosLimit, uint32_t posState)
{
    uint32_t matchLen;
    size_t matchDist;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
    matchLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    
    matchDist = CmptLzDistDec(decCtx, probsMatrix, pRange, pRangeCode, pRangeBound, matchLen);
    
    if (matchDist > decCtx->dictBufSize)
    {
        if (matchDist == (size_t)0xFFFFFFFF)
        {
            decCtx->remainLen = CMPTLZ_MATCH_MAX_LEN;
            decCtx->state -= CMPTLZ_MKSTATE_NUM;
            return CMPT_OK;
        }
        else
        {
            return CMPT_ERROR_DATA;
        }
    }
    return CmptLzDecByDistAndLen(decCtx, matchDist, matchLen + 2, dicPosLimit);
}

static uint32_t CmptLzLitDec(CmptLzDecCtx *decCtx, uint32_t *pRange, uint32_t *pRangeCode, uint32_t *pRangeBound)
{
    uint32_t decSym = 1;
    uint32_t mkState = decCtx->state;
    uint32_t procPos = decCtx->processedPos;
    uint32_t checkDicSize = decCtx->checkDicSize;
    uint32_t litCtx = decCtx->prop.litCtx;
    uint32_t litPosMask = ((uint32_t)0x100 << decCtx->prop.litPos) - ((uint32_t)0x100 >> litCtx);
    CmptLzDecProb *probLit;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    const unsigned char *bufToDec = decCtx->buf;
    unsigned char *dict = decCtx->dict;
    size_t dictBufSize = decCtx->dictBufSize;
    size_t dictPos = decCtx->dictPos;
    uint32_t range = *pRange;
    uint32_t rangeBound = *pRangeBound;
    uint32_t rangeCode = *pRangeCode;
    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0 || checkDicSize != 0)
    {
        probSlot += (uint32_t)CMPTLZ_REP3 *
                    ((((procPos << 8) + dict[(dictPos == 0 ? dictBufSize : dictPos) - 1]) & litPosMask) << litCtx);
    }
    int i = 0;
    if (mkState < CMPTLZ_LIT_STATES)
    {
        mkState -= (mkState < 4) ? mkState : 3;
        for (i = 0; i < 8; i++)
        {
            CMPTLZ_NORMAL_BIT_DEC((probSlot + decSym), range, rangeCode, rangeBound, decSym);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, bufToDec);
        }
    }
    else
    {
        uint32_t bit;
        uint32_t offset = 0x100;
        uint32_t rep0 = decCtx->reps[0];
        uint32_t matchSym = dict[dictPos - rep0 + ((dictPos < rep0) ? dictBufSize : 0)];
        mkState -= (mkState < 10) ? CMPTLZ_REP3 : 6;
        for (i = 0; i < 8; i++)
        {
            CMPTLZ_MATCH_BIT_DEC(probSlot, range, rangeCode, rangeBound, decSym, matchSym, offset, bit, bufToDec);
        }
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    dict[dictPos++] = (uint8_t)decSym;
    decCtx->processedPos += 1;
    decCtx->state = mkState;
    decCtx->dictPos = dictPos;
    decCtx->buf = bufToDec;
    return CMPT_OK;
}

int CmptLzDecDirectProcess(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit)
{
    uint32_t decRes;
    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t procPos;
    uint32_t mkState;
    uint32_t posState;
    uint32_t range = decCtx->range;
    uint32_t rangeCode = decCtx->code;
    uint32_t rangeBound = 0;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    do
    {
        procPos = decCtx->processedPos;
        mkState = decCtx->state;
        posState = CMPTLZ_CALC_POS_STATE(procPos, pbMask);
        probSlot = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;
        CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
        if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
            decRes = CmptLzLitDec(decCtx, &range, &rangeCode, &rangeBound);
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
            probSlot = CmptLzGetIsRepProb(probsMatrix) + mkState;
            if CMPTLZ_IS_THE_BIT_0(probSlot, range, rangeCode, rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_0(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
                decRes = CmptLzMatchDec(decCtx, &range, &rangeCode, &rangeBound, dicPosLimit, posState);
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_1(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE(range, rangeCode, decCtx->buf);
                decRes = CmptLzRepDec(decCtx, &range, &rangeCode, &rangeBound, dicPosLimit, posState);
            }
            if (decRes != CMPT_OK)
            {
                break;
            }
        }
    } while (decCtx->dictPos < dicPosLimit && decCtx->buf < bufLimit && decCtx->remainLen < CMPTLZ_MATCH_MAX_LEN);
    decCtx->range = range;
    decCtx->code = rangeCode;
    return (int)decRes;
}

static int CmptLzTryDecLenAndDist(CmptLzDecCtx *decCtx, uint32_t mkState, uint32_t range, uint32_t rangeCode,
                                  uint32_t rangeBound, CmptLzDecProb *probSlot, const unsigned char *bufTryDec,
                                  const unsigned char **pbufLimit)
{
    uint32_t offset;
    uint32_t bits2BeDec;
    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t posState = CMPTLZ_CALC_POS_STATE(decCtx->processedPos, pbMask);
    const unsigned char *bufLimit = *pbufLimit;
    CmptLzDecProb *probBit;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    CmptLzDecProb *probLen = probSlot + CMPTLZ_LEN_CHOICE;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen);
    if (rangeCode < rangeBound)
    {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET + posState;
        bits2BeDec = 3;
        offset = 0;
    }
    else
    {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        probLen = probSlot + CMPTLZ_LEN_CHOICE2;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen);
        if (rangeCode < rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE + CMPTLZ_LEN_CHOICE2 + posState;
            bits2BeDec = 3;
            offset = (CMPTLZ_LOW_LEN_CLASS << 1);
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET;
            bits2BeDec = 8;
            offset = (CMPTLZ_LOW_LEN_CLASS << 1);
        }
    }
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    uint32_t decSym = 1;
    do
    {
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    } while (decSym < ((uint32_t)1 << bits2BeDec));
    decSym -= ((uint32_t)1 << bits2BeDec);
    decSym += offset;
    if (mkState >= 4)
    {
        *pbufLimit = bufTryDec;
        return CMPT_OK;
    }
    probSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decSym);
    decSym = 1;
    do
    {
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    } while (decSym < (1 << CMPTLZ_POS_SLOT_BITS));
    decSym -= (1 << CMPTLZ_POS_SLOT_BITS);
    bits2BeDec = ((decSym >> 1) - 1);
    if (decSym >= CMPTLZ_LOW_POSSLOT)
    {
        if (decSym < CMPTLZ_HIGH_POSSLOT)
        {
            probSlot = CmptLzGetSpecPosProb(probsMatrix) + (CmptLzGetBaseDistByPosSlot(decSym) << bits2BeDec);
        }
        else
        {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS;
            do
            {
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1);
            } while (--bits2BeDec);
            probSlot = CmptLzGetAilgnProb(probsMatrix);
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS;
        }
        decSym = 1;
        offset = 1;
        do
        {
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probBit);
            if (rangeCode < rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
        } while (--bits2BeDec);
    }
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK;
}

static int CmptLzTryDecLitPacket(CmptLzDecCtx *decCtx, uint32_t range, uint32_t rangeCode, uint32_t rangeBound,
                                 const unsigned char *bufTryDec, const unsigned char **pbufLimit)
{
    CmptLzDecProb *probBit;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    uint32_t procPos = decCtx->processedPos;
    uint32_t litPosMask = ((uint32_t)0x100 << decCtx->prop.litPos) - ((uint32_t)0x100 >> decCtx->prop.litCtx);
    size_t dictBufSize = decCtx->dictBufSize;
    size_t dicPos = decCtx->dictPos;
    const unsigned char *dict = decCtx->dict;
    const unsigned char *bufLimit = *pbufLimit;
    if (decCtx->dictPos >= decCtx->dictBufSize)
    {
        return CMPT_ERROR_DATA;
    }
    probSlot = CmptLzGetLiteralProb(probsMatrix);
    if (procPos != 0 || decCtx->checkDicSize != 0)
    {
        probSlot += (uint32_t)3 * ((((procPos << 8) + dict[(dicPos == 0 ? dictBufSize : dicPos) - 1]) & litPosMask)
                                   << decCtx->prop.litCtx);
    }
    uint32_t decSym = 1;
    if (decCtx->state < CMPTLZ_LIT_STATES)
    {
        do
        {
            probBit = probSlot + decSym;
            CMPTLZ_SINGLE_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        } while (decSym < 0x100);
    }
    else
    {
        uint32_t bit;
        uint32_t matchSym = dict[dicPos - decCtx->reps[0] + ((dicPos < decCtx->reps[0]) ? dictBufSize : 0)];
        uint32_t offset = 0x100;
        do
        {
            matchSym <<= 1;
            bit = offset;
            offset &= matchSym;
            probBit = probSlot + (offset + bit + decSym);
            CMPTLZ_MATCH_BIT_TRY_DEC(range, rangeCode, rangeBound, decSym, probBit);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        } while (decSym < 0x100);
    }
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK;
}

int CmptLzTryDecOnePacket(CmptLzDecCtx *decCtx, const unsigned char *bufTryDec, const unsigned char **pbufLimit)
{
    uint32_t rangeBound = 0;
    uint32_t range = decCtx->range;
    uint32_t rangeCode = decCtx->code;
    uint32_t mkState = decCtx->state;
    const unsigned char *bufLimit = *pbufLimit;
    CmptLzDecProb *probSlot;
    CmptLzDecProb *probSlot1;
    CmptLzDecProb *probSlot2;
    CmptLzDecProb *probsMatrix = CmptLzGetProbsMatrix(decCtx);
    uint32_t pbMask = ((uint32_t)1 << (decCtx->prop.posBits)) - 1;
    uint32_t posState = CMPTLZ_CALC_POS_STATE(decCtx->processedPos, pbMask);
    probSlot1 = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot1);
    if (rangeCode < rangeBound)
    {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx, range, rangeCode, rangeBound, bufTryDec, pbufLimit);
    }
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
    probSlot2 = CmptLzGetIsRepProb(probsMatrix) + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot2);
    if (rangeCode < rangeBound)
    {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
        probSlot = CmptLzGetMatchLenCoderProb(probsMatrix);
        mkState = 0;
    }
    else
    {
        if (decCtx->dictPos >= decCtx->dictBufSize)
        {
            return CMPT_ERROR_DATA;
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
        probSlot = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
        if (rangeCode < rangeBound)
        {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
            probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
            if (rangeCode < rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
                *pbufLimit = bufTryDec;
                return CMPT_OK;
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
            }
        }
        else
        {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
            probSlot = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
            if (rangeCode < rangeBound)
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
            }
            else
            {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE(range, rangeCode, bufTryDec, bufLimit);
                probSlot = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probSlot);
                if (rangeCode < rangeBound)
                {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0(range, rangeBound);
                }
                else
                {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1(range, rangeCode, rangeBound);
                }
            }
        }
        probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
        mkState = CMPTLZ_MKSTATE_NUM;
    }
    return CmptLzTryDecLenAndDist(decCtx, mkState, range, rangeCode, rangeBound, probSlot, bufTryDec, pbufLimit);
}

int CmptLzDecCarefulProcess(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *bufLimit)
{
    int res = CMPT_OK;
    uint32_t remainLen;
    const unsigned char *bufLimitTmp;
    const unsigned char *pSrcIn;
    do
    {
        bufLimitTmp = bufLimit;
        pSrcIn = decCtx->buf;
        res = CmptLzTryDecOnePacket(decCtx, pSrcIn, &bufLimitTmp);
        if (res == CMPTLZ_DEC_INPUT_EOF)
        {
            break;
        }
        res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimitTmp);
        if ((res != CMPT_OK) || (decCtx->buf != bufLimitTmp))
        {
            return CMPT_ERROR_DATA;
        }
        if (decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN)
        {
            break;
        }
    } while (decCtx->dictPos < dicPosLimit);
    if ((res == CMPTLZ_DEC_INPUT_EOF) && (decCtx->buf < bufLimit))
    {
        remainLen = (uint32_t)(bufLimit - decCtx->buf);
        decCtx->tempBufSize = remainLen;
        for (uint32_t idx = 0; idx < remainLen; idx++)
        {
            decCtx->tempBuf[idx] = decCtx->buf[idx];
        }
    }
    return CMPT_OK;
}

int CmptLzDecSinglePacket(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *pSrcIn, size_t srcInLen,
                          size_t *psrcCostLen)
{
    int res;
    size_t lookAheadLen = 0;
    uint32_t newTempBufSize = decCtx->tempBufSize;
    unsigned char *oldTmpBuf = &(decCtx->tempBuf[decCtx->tempBufSize]);
    while (newTempBufSize < CMPTLZ_REQUIRED_INPUT_MAX && lookAheadLen < srcInLen)
    {
        decCtx->tempBuf[newTempBufSize++] = pSrcIn[lookAheadLen++];
    }
    const unsigned char *bufLimit = decCtx->tempBuf + newTempBufSize;
    res = CmptLzTryDecOnePacket(decCtx, &(decCtx->tempBuf[0]), &bufLimit);
    if (res == CMPTLZ_DEC_INPUT_EOF)
    {
        *psrcCostLen = lookAheadLen;
        decCtx->tempBufSize = newTempBufSize;
        return CMPTLZ_DEC_INPUT_EOF;
    }
    if (res == CMPT_ERROR_DATA)
    {
        return res;
    }
    decCtx->buf = &(decCtx->tempBuf[0]);
    res = CmptLzDecDirectProcess(decCtx, dicPosLimit, bufLimit);
    if ((res != CMPT_OK) || (bufLimit != decCtx->buf) || (bufLimit <= oldTmpBuf))
    {
        *psrcCostLen = 0;
        return CMPT_ERROR_DATA;
    }
    *psrcCostLen = (size_t)(bufLimit - oldTmpBuf);
    decCtx->tempBufSize = 0;
    return res;
}

static void CmptLzDecCheckDictSizeUpdate(CmptLzDecCtx *decCtx)
{
    if (decCtx->checkDicSize == 0 && decCtx->processedPos >= decCtx->prop.dicSize)
    {
        decCtx->checkDicSize = decCtx->prop.dicSize;
    }
}

static void CmptLzDecRemWriteInDict(CmptLzDecCtx *decCtx, size_t dicPosLimit)
{
    size_t dictPos = decCtx->dictPos;
    size_t remainDecLen = decCtx->remainLen;
    size_t dictBufSize = decCtx->dictBufSize;
    size_t remainDicLen = dicPosLimit - dictPos;
    if (remainDicLen < remainDecLen)
    {
        remainDecLen = remainDicLen;
    }
    if (remainDecLen == 0)
    {
        return;
    }
    decCtx->processedPos += (uint32_t)remainDecLen;
    decCtx->remainLen -= (uint32_t)remainDecLen;
    unsigned char *dict = decCtx->dict;
    size_t rep0 = decCtx->reps[0];
    while (remainDecLen != 0)
    {
        remainDecLen--;
        dict[dictPos] = dict[dictPos - rep0 + (dictPos < rep0 ? dictBufSize : 0)];
        dictPos++;
    }
    decCtx->dictPos = dictPos;
    CmptLzDecCheckDictSizeUpdate(decCtx);
}

static void CmptLzDecGetProbsInit(CmptLzDecCtx *decCtx)
{
    uint32_t idx;
    uint32_t numProbs = CmptLzGetNumProbs(&(decCtx->prop));
    CmptLzDecProb *decProbs = decCtx->probs;
    for (idx = 0; idx < numProbs; idx++)
    {
        decProbs[idx] = CMPTLZ_PROB_LG >> 1;
    }
    decCtx->state = 0;
}

static void CmptLzRangeCodeInit(CmptLzDecCtx *decCtx)
{
    uint32_t rangeCode = (uint32_t)(decCtx->tempBuf[1]) << 24;
    rangeCode |= (uint32_t)(decCtx->tempBuf[2]) << 16;
    rangeCode |= (uint32_t)(decCtx->tempBuf[3]) << 8;
    rangeCode |= (uint32_t)(decCtx->tempBuf[4]);
    decCtx->code = rangeCode;
    decCtx->range = 0xFFFFFFFF;
}

static int CmptLzDecCtxPrepare(CmptLzDecCtx *decCtx, const unsigned char *pSrcIn, size_t srcInLen,
                               EnCmptLzStatus *finStatus)
{
    size_t readCodeLen = CMPTLZ_RANGE_CODE_SIZE - decCtx->tempBufSize;
    readCodeLen = (srcInLen < readCodeLen) ? srcInLen : readCodeLen;
    while (readCodeLen-- > 0)
    {
        decCtx->tempBuf[decCtx->tempBufSize++] = *pSrcIn++;
    }
    if (decCtx->tempBufSize != 0 && decCtx->tempBuf[0] != 0)
    {
        decCtx->tempBufSize = 0;
        *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
        return CMPT_ERROR_DATA;
    }
    if (decCtx->tempBufSize < CMPTLZ_RANGE_CODE_SIZE)
    {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    CmptLzRangeCodeInit(decCtx);
    if (decCtx->remainLen > CMPTLZ_MATCH_MAX_LEN + 1)
    {
        CmptLzDecGetProbsInit(decCtx);
        decCtx->reps[0] = 1;
        decCtx->reps[1] = 1;
        decCtx->reps[2] = 1;
        decCtx->reps[3] = 1;
    }
    decCtx->remainLen = 0;
    return CMPT_OK;
}

int CmptLzDecDecodeToDic(CmptLzDecCtx *decCtx, size_t dicPosLimit, const unsigned char *pSrcIn, size_t *pStrInLen,
                         EnCmptLzFinMode finMode, EnCmptLzStatus *finStatus)
{
    int res;
    bool carefulDecDone = false;
    size_t srcDecLenTmp;
    size_t srcDecLen = 0;
    size_t srcInLen = *pStrInLen;
    if (decCtx->remainLen > CMPTLZ_MATCH_MAX_LEN)
    {
        size_t oldTempBufSize = decCtx->tempBufSize;
        res = CmptLzDecCtxPrepare(decCtx, pSrcIn, srcInLen, finStatus);
        srcDecLenTmp = (decCtx->tempBufSize - oldTempBufSize);
        if ((res != CMPT_OK) || (*finStatus == CMPTLZ_STATUS_NEEDS_MORE_INPUT))
        {
            *pStrInLen = srcDecLenTmp;
            return res;
        }
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        decCtx->tempBufSize = 0;
    }
    if (decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN)
    {
        if (decCtx->code != 0)
        {
            return CMPT_ERROR_DATA;
        }
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }
    if (decCtx->remainLen != 0)
    {
        CmptLzDecRemWriteInDict(decCtx, dicPosLimit);
    }
    if (decCtx->tempBufSize != 0)
    {
        res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, &srcDecLenTmp);
        *pStrInLen = srcDecLenTmp;
        if (res == CMPT_ERROR_DATA)
        {
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
            return CMPT_ERROR_DATA;
        }
        else if (res == CMPTLZ_DEC_INPUT_EOF)
        {
            *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
            return CMPT_OK;
        }
        else
        {
            srcDecLen += srcDecLenTmp;
            pSrcIn += srcDecLenTmp;
            srcInLen -= srcDecLenTmp;
        }
    }
    while ((decCtx->dictPos < dicPosLimit) && (carefulDecDone == false))
    {
        decCtx->buf = pSrcIn;
        if (srcInLen <= CMPTLZ_REQUIRED_INPUT_MAX)
        {
            res = CmptLzDecCarefulProcess(decCtx, dicPosLimit, pSrcIn + srcInLen);
            carefulDecDone = true;
        }
        else
        {
            res = CmptLzDecDirectProcess(decCtx, dicPosLimit, pSrcIn + srcInLen - CMPTLZ_REQUIRED_INPUT_MAX);
        }
        srcDecLenTmp = (size_t)(decCtx->buf - pSrcIn) + decCtx->tempBufSize;
        srcDecLen += srcDecLenTmp;
        pSrcIn += srcDecLenTmp;
        srcInLen -= srcDecLenTmp;
        if (res == CMPT_ERROR_DATA)
        {
            *pStrInLen = srcDecLen;
            *finStatus = CMPTLZ_STATUS_NOT_SPECIFIED;
            return CMPT_ERROR_DATA;
        }
    }
    *pStrInLen = srcDecLen;
    if ((decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) && (decCtx->code == 0))
    {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }
    if (decCtx->dictPos < dicPosLimit)
    {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    if ((decCtx->remainLen == 0) && (decCtx->code == 0))
    {
        *finStatus = CMPTLZ_STATUS_MAYBE_FINISHED_WITHOUT_MARK;
        return CMPT_OK;
    }
    if (finMode == CMPTLZ_FINISH_ANY)
    {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
        return CMPT_OK;
    }
    if (decCtx->remainLen != 0)
    {
        *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
        return CMPT_ERROR_DATA;
    }
    srcDecLenTmp = 0;
    res = CmptLzDecSinglePacket(decCtx, dicPosLimit, pSrcIn, srcInLen, &srcDecLenTmp);
    srcDecLen += srcDecLenTmp;
    *pStrInLen = srcDecLen;
    if (res == CMPTLZ_DEC_INPUT_EOF)
    {
        *finStatus = CMPTLZ_STATUS_NEEDS_MORE_INPUT;
        return CMPT_OK;
    }
    if ((decCtx->remainLen == CMPTLZ_MATCH_MAX_LEN) && (decCtx->code == 0))
    {
        *finStatus = CMPTLZ_STATUS_FINISHED_WITH_MARK;
        return CMPT_OK;
    }
    *finStatus = CMPTLZ_STATUS_NOT_FINISHED;
    return CMPT_ERROR_DATA;
}

int CmptlzDecompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzDecParam *param)
{
    if (src == NULL || dst == NULL || dstSize == NULL)
    {
        CMPTLZ_LOG(CMPT_ERROR_UNSUPPORTED, "The input parameter NULL is incorrect.");
        return CMPT_ERROR_UNSUPPORTED;
    }
    if (srcSize > 0x7fffffff || *dstSize > 0x7fffffff)
    {
        CMPTLZ_LOG(CMPT_ERROR_UNSUPPORTED, "dstSize:0x%zx srcSize:0x%zx", *dstSize, srcSize);
        return CMPT_ERROR_UNSUPPORTED;
    }
    if (param == NULL || param->memHook == NULL || param->protData == NULL || param->protSize != CMPTLZ_PROPS_SIZE)
    {
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

static void CmptlzEndMarker(void)
{
    return;
}

static int CmptlzFlush(CmptLzEncCtx *encCtx)
{
    encCtx->encNeedFinish = true;
    if (encCtx->endMarker != 0)
    {
        CmptlzEndMarker();
    }
    CmptRcFlushData(encCtx->rcCtx);
    return CmptRcFlush64Kb(encCtx->rcCtx);
}

static void CmptPriceCheck(CmptLzEncCtx *encCtx)
{
    if (encCtx->matchPriceCount >= CMPT_PRICE_COUNT)
    {
        CmptPriceGenDistTable(encCtx);
        CmptPriceGenAlignTable(encCtx);
        CmptPriceGenLenTable(encCtx, &encCtx->matchLenEncoder);
    }
    if (encCtx->repLenPriceCount <= 0)
    {
        encCtx->repLenPriceCount = CMPT_PRICE_COUNT;
        CmptPriceGenLenTable(encCtx, &encCtx->repLenEncoder);
    }
}

static int CmptEncShortOrRep0(CmptLzEncCtx *encCtx, uint32_t nowpos32, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    if (lenRes == 1)
    {
        shiftRes = CmptlzEncShortRep(encCtx, nowpos32);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    else
    {
        shiftRes = CmptlzEncLongRep(encCtx, 0, nowpos32, lenRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

int CmptEncodeOneBlock(CmptLzEncCtx *encCtx)
{
    CmptMfCtx *mf = encCtx->mfCtx;
    uint32_t nowpos32 = encCtx->nowpos64;
    uint32_t startpos = nowpos32;
    uint32_t backRes, lenRes;
    int shiftRes = CMPT_OK;
    while (true)
    {
        CmptlzDp(encCtx, mf, nowpos32);
        backRes = encCtx->backRes;
        lenRes = encCtx->lenRes;
        switch (backRes)
        {
        case CMPTLZ_UINT32_MAX:
            shiftRes = CmptlzEncLit(encCtx, mf, nowpos32);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            break;
        case 0:
            shiftRes = CmptEncShortOrRep0(encCtx, nowpos32, lenRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            break;
        case 1:
            shiftRes = CmptlzEncLongRep(encCtx, 1, nowpos32, lenRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            break;
        case 2:
            shiftRes = CmptlzEncLongRep(encCtx, 2, nowpos32, lenRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            break;
        case 3:
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
        if (mf->readAhead == 0)
        {
            CmptPriceCheck(encCtx);
            if (mf->srcLen <= mf->mfStart)
            {
                break;
            }
            if (nowpos32 - startpos >= CMPT_ONE_BLOCK_MAX_SIZE)
            {
                encCtx->nowpos64 += nowpos32 - startpos;
                return 0;
            }
        }
    }
    encCtx->nowpos64 += nowpos32 - startpos;
    return CmptlzFlush(encCtx);
}

int CmptEncodeAll(CmptLzEncCtx *encCtx)
{
    CmptRcCtx *rc = encCtx->rcCtx;
    CmptMfCtx *mf = encCtx->mfCtx;
    if (mf->srcLen == 0)
    {
        return CmptlzFlush(encCtx);
    }
    if (encCtx->nowpos64 == 0)
    {
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
        if (mf->srcLen == 1)
        {
            return CmptlzFlush(encCtx);
        }
    }
    int res;
    while (true)
    {
        res = CmptEncodeOneBlock(encCtx);
        if (res != 0 || encCtx->encNeedFinish)
        {
            break;
        }
    }
    return res;
}

static void CmptlzDpInitShortRep(CmptLzEncCtx *encCtx, uint32_t repMatchPrice, const uint32_t posState)
{
    const uint32_t shortRepPrice = repMatchPrice + CmptPriceShortRep(encCtx, encCtx->state, posState);
    if (shortRepPrice < encCtx->opts[1].price)
    {
        encCtx->opts[1].price = shortRepPrice;
        encCtx->opts[1].backPrev = 0;
    }
}

static void CmptlzDpInitLongRep(CmptLzEncCtx *encCtx, uint32_t *repLens, const uint32_t repMatchPrice,
                                const uint32_t posState)
{
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++)
    {
        uint32_t repLen = repLens[i];
        if (repLen < CMPTLZ_MATCH_LEN_MIN)
        {
            continue;
        }
        const uint32_t price = repMatchPrice + CmptPriceLongRep(encCtx, i, encCtx->state, posState);
        do
        {
            const uint32_t curAndLenPrice = price + CmptPriceLen(&encCtx->repLenEncoder, repLen, posState);
            if (curAndLenPrice < encCtx->opts[repLen].price)
            {
                encCtx->opts[repLen].price = curAndLenPrice;
                encCtx->opts[repLen].posPrev = 0;
                encCtx->opts[repLen].backPrev = i;
            }
            repLen--;
        } while (repLen >= CMPTLZ_MATCH_LEN_MIN);
    }
}

static void CmptlzDpInitMatch(CmptLzEncCtx *encCtx, uint32_t matchesCount, uint32_t normalMatchPrice, uint32_t posState,
                              uint32_t len)
{
    uint32_t i = 0;
    while (len > encCtx->matches[i].len)
    {
        i++;
    }
    for (;; len++)
    {
        const uint32_t dist = encCtx->matches[i].dist;
        const uint32_t curAndLenPrice = normalMatchPrice + CmptPriceDistWithLen(encCtx, dist, len, posState);
        if (curAndLenPrice < encCtx->opts[len].price)
        {
            encCtx->opts[len].price = curAndLenPrice;
            encCtx->opts[len].posPrev = 0;
            encCtx->opts[len].backPrev = dist + CMPTLZ_NUM_REPS;
        }
        if (len == encCtx->matches[i].len)
        {
            if (++i == matchesCount)
            {
                break;
            }
        }
    }
}

static uint32_t CmptlzDpInit(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t position)
{
    const uint32_t niceLen = mf->niceLen;
    uint32_t lenMain;
    uint32_t matchesCount = 0;
    if (mf->readAhead == 0)
    {
        lenMain = CmptlzMatchFinder(mf, &matchesCount, encCtx->matches);
    }
    else
    {
        lenMain = encCtx->longestMatchLen;
        matchesCount = encCtx->matchesCount;
    }
    const uint8_t *const buf = CmptMfGetPtr(mf) - 1;
    const uint32_t bufAvail = CMPTLZ_FIND_MIN(CmptMfAvail(mf) + 1, CMPT_MF_LONGEST_MATCH);
    if (bufAvail < CMPTLZ_MATCH_LEN_MIN)
    {
        encCtx->backRes = CMPTLZ_UINT32_MAX;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }
    uint32_t repLens[CMPTLZ_NUM_REPS];
    uint32_t repMaxIndex = 0;
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++)
    {
        const uint8_t *const bufBack = buf - encCtx->reps[i] - 1;
        if (NOT_EQUAL_2_BYTES(buf, bufBack))
        {
            repLens[i] = 0;
            continue;
        }
        repLens[i] = CmptMemCmpLenSafe(buf, bufBack, CMPTLZ_MATCH_LEN_MIN, bufAvail);
        if (repLens[i] > repLens[repMaxIndex])
        {
            repMaxIndex = i;
        }
    }
    if (repLens[repMaxIndex] >= niceLen)
    {
        encCtx->backRes = repMaxIndex;
        encCtx->lenRes = repLens[repMaxIndex];
        CmptlzMatchSkiper(mf, repLens[repMaxIndex] - 1);
        return CMPTLZ_UINT32_MAX;
    }
    if (lenMain >= niceLen)
    {
        encCtx->backRes = encCtx->matches[matchesCount - 1].dist + CMPTLZ_NUM_REPS;
        encCtx->lenRes = lenMain;
        CmptlzMatchSkiper(mf, lenMain - 1);
        return CMPTLZ_UINT32_MAX;
    }
    const uint8_t currentByte = *buf;
    const uint8_t matchByte = *(buf - encCtx->reps[0] - 1);
    const uint32_t lenEnd = CMPTLZ_FIND_MAX(lenMain, repLens[repMaxIndex]);
    if ((lenEnd < CMPTLZ_MATCH_LEN_MIN) && (currentByte != matchByte))
    {
        encCtx->backRes = CMPTLZ_UINT32_MAX;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }
    encCtx->opts[0].state = encCtx->state;
    const uint32_t posState = position & encCtx->posMask;
    encCtx->litMarcov.pos = position;
    encCtx->litMarcov.prevByte = *(buf - 1);
    bool isLiteralState = (encCtx->state < 7);
    bool isMatchMode = !isLiteralState;
    encCtx->opts[1].price = CmptPriceBit0(encCtx, encCtx->isMatch[encCtx->state][posState]) +
                            CmptPriceLiteral(encCtx, isMatchMode, matchByte, currentByte);
    encCtx->opts[1].backPrev = CMPTLZ_UINT32_MAX;
    const uint32_t matchPrice = CmptPriceBit1(encCtx, encCtx->isMatch[encCtx->state][posState]);
    const uint32_t repMatchPrice = matchPrice + CmptPriceBit1(encCtx, encCtx->isRep[encCtx->state]);
    if (matchByte == currentByte)
    {
        CmptlzDpInitShortRep(encCtx, repMatchPrice, posState);
    }
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN)
    {
        encCtx->backRes = encCtx->opts[1].backPrev;
        encCtx->lenRes = 1;
        return CMPTLZ_UINT32_MAX;
    }
    encCtx->opts[1].posPrev = 0;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++)
    {
        encCtx->opts[0].backs[i] = encCtx->reps[i];
    }
    uint32_t len = lenEnd;
    do
    {
        encCtx->opts[len].price = CMPT_INFINITY_PRICE;
        len--;
    } while (len >= CMPTLZ_MATCH_LEN_MIN);
    CmptlzDpInitLongRep(encCtx, repLens, repMatchPrice, posState);
    const uint32_t normalMatchPrice = matchPrice + CmptPriceBit0(encCtx, encCtx->isRep[encCtx->state]);
    len = (repLens[0] > CMPTLZ_MATCH_LEN_MIN) ? repLens[0] + 1 : CMPTLZ_MATCH_LEN_MIN;
    if (len <= lenMain)
    {
        CmptlzDpInitMatch(encCtx, matchesCount, normalMatchPrice, posState, len);
    }
    return lenEnd;
}

static void CmptlzDpPre(CmptLzEncCtx *encCtx, uint32_t *mainReps, const uint32_t cur)
{
    uint32_t posPointer = encCtx->opts[cur].posPrev;
    CmptlzState state = encCtx->opts[posPointer].state;
    if (posPointer == cur - 1)
    {
        if (encCtx->opts[cur].backPrev == 0)
        {
            CMPT_STATE_UPDATE_WHEN_SHORTREP(state);
        }
        else
        {
            CMPT_STATE_UPDATE_WHEN_LIT(state);
        }
    }
    else
    {
        uint32_t backPointer;
        backPointer = encCtx->opts[cur].backPrev;
        if (backPointer < CMPTLZ_NUM_REPS)
        {
            CMPT_STATE_UPDATE_WHEN_LONGREP(state);
        }
        else
        {
            CMPT_STATE_UPDATE_WHEN_MATCH(state);
        }
        uint32_t i;
        if (backPointer < CMPTLZ_NUM_REPS)
        {
            mainReps[0] = encCtx->opts[posPointer].backs[backPointer];
            for (i = 1; i <= backPointer; i++)
            {
                mainReps[i] = encCtx->opts[posPointer].backs[i - 1];
            }
            for (; i < CMPTLZ_NUM_REPS; i++)
            {
                mainReps[i] = encCtx->opts[posPointer].backs[i];
            }
        }
        else
        {
            mainReps[0] = backPointer - CMPTLZ_NUM_REPS;
            for (i = 1; i < CMPTLZ_NUM_REPS; i++)
            {
                mainReps[i] = encCtx->opts[posPointer].backs[i - 1];
            }
        }
    }
    encCtx->opts[cur].state = state;
    uint32_t i;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++)
    {
        encCtx->opts[cur].backs[i] = mainReps[i];
    }
}

static void CmptlzDpTryCurAndLit(CmptLzEncCtx *encCtx, const uint32_t curPrice, CmptlzState curState,
                                 const uint32_t posState, const uint32_t cur, const uint8_t latestMatchByte,
                                 const uint8_t curByte)
{
    bool isLiteralState = (curState < 7);
    bool isMatchMode = !isLiteralState;
    const uint32_t curAndLitPrice = curPrice + CmptPriceBit0(encCtx, encCtx->isMatch[curState][posState]) +
                                    CmptPriceLiteral(encCtx, isMatchMode, latestMatchByte, curByte);
    if (curAndLitPrice < encCtx->opts[cur + 1].price)
    {
        encCtx->opts[cur + 1].price = curAndLitPrice;
        encCtx->opts[cur + 1].posPrev = cur;
        encCtx->opts[cur + 1].backPrev = CMPTLZ_UINT32_MAX;
    }
}

static void CmptlzDpTryCurAndShort(CmptLzEncCtx *encCtx, const uint32_t repMatchPrice, const uint32_t cur,
                                   CmptlzState curState, const uint32_t posState)
{
    const uint32_t shortRepPrice = repMatchPrice + CmptPriceShortRep(encCtx, curState, posState);
    if (shortRepPrice < encCtx->opts[cur + 1].price)
    {
        encCtx->opts[cur + 1].price = shortRepPrice;
        encCtx->opts[cur + 1].posPrev = cur;
        encCtx->opts[cur + 1].backPrev = 0;
    }
}

static void CmptlzDpTryCurAndLong(CmptLzEncCtx *encCtx, const uint32_t prefixPrice, const uint32_t cur,
                                  uint32_t mainRepIndex, uint32_t lenEqual, const uint32_t posState)
{
    do
    {
        const uint32_t curLongRepPrice = prefixPrice + CmptPriceLen(&encCtx->repLenEncoder, lenEqual, posState);
        if (curLongRepPrice < encCtx->opts[cur + lenEqual].price)
        {
            encCtx->opts[cur + lenEqual].price = curLongRepPrice;
            encCtx->opts[cur + lenEqual].posPrev = cur;
            encCtx->opts[cur + lenEqual].backPrev = mainRepIndex;
        }
    } while (--lenEqual >= CMPTLZ_MATCH_LEN_MIN);
}

static void CmptlzDpTryCurAndMatch(CmptLzEncCtx *encCtx, uint32_t startLen, uint32_t matchCount,
                                   const uint32_t normalmatch_prefixPrice, const uint32_t cur, const uint32_t posState)
{
    uint32_t i = 0;
    while (startLen > encCtx->matches[i].len)
    {
        i++;
    }
    uint32_t lenTest;
    for (lenTest = startLen;; lenTest++)
    {
        const uint32_t curBack = encCtx->matches[i].dist;
        uint32_t cur_normalmatchPrice =
            normalmatch_prefixPrice + CmptPriceDistWithLen(encCtx, curBack, lenTest, posState);
        if (cur_normalmatchPrice < encCtx->opts[cur + lenTest].price)
        {
            encCtx->opts[cur + lenTest].price = cur_normalmatchPrice;
            encCtx->opts[cur + lenTest].posPrev = cur;
            encCtx->opts[cur + lenTest].backPrev = curBack + CMPTLZ_NUM_REPS;
        }
        if (lenTest == encCtx->matches[i].len)
        {
            if (++i == matchCount)
            {
                break;
            }
        }
    }
}

static uint32_t CmptlzDpProcess(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t *mainReps, uint32_t lenEnd,
                                uint32_t position, const uint32_t cur)
{
    CmptlzState curState = encCtx->opts[cur].state;
    const uint32_t bufAvailFull = CMPTLZ_FIND_MIN(CmptMfAvail(mf) + 1, CMPT_DP_OPTMAX - 1 - cur);
    const uint8_t *buf = CmptMfGetPtr(mf) - 1;
    const uint32_t niceLen = mf->niceLen;
    const uint32_t curPrice = encCtx->opts[cur].price;
    const uint8_t curByte = *buf;
    const uint8_t latestMatchByte = *(buf - mainReps[0] - 1);
    const uint32_t posState = position & encCtx->posMask;
    encCtx->litMarcov.pos = position;
    encCtx->litMarcov.prevByte = *(buf - 1);
    CmptlzDpTryCurAndLit(encCtx, curPrice, curState, posState, cur, latestMatchByte, curByte);
    const uint32_t matchPrice = curPrice + CmptPriceBit1(encCtx, encCtx->isMatch[curState][posState]);
    const uint32_t repMatchPrice = matchPrice + CmptPriceBit1(encCtx, encCtx->isRep[curState]);
    if (curByte == latestMatchByte && !(encCtx->opts[cur + 1].posPrev < cur && encCtx->opts[cur + 1].backPrev == 0))
    {
        CmptlzDpTryCurAndShort(encCtx, repMatchPrice, cur, curState, posState);
    }
    if (bufAvailFull < CMPTLZ_MATCH_LEN_MIN)
    {
        return lenEnd;
    }
    const uint32_t bufAvail = CMPTLZ_FIND_MIN(bufAvailFull, niceLen);
    uint32_t startLen = CMPTLZ_MATCH_LEN_MIN;
    uint32_t mainRepIndex;
    for (mainRepIndex = 0; mainRepIndex < CMPTLZ_NUM_REPS; mainRepIndex++)
    {
        const uint8_t *const bufRepBack = buf - mainReps[mainRepIndex] - 1;
        if (NOT_EQUAL_2_BYTES(buf, bufRepBack))
        {
            continue;
        }
        uint32_t lenEqual;
        lenEqual = CmptMemCmpLenSafe(buf, bufRepBack, CMPTLZ_MATCH_LEN_MIN, bufAvail);
        while (lenEnd < cur + lenEqual)
        {
            lenEnd++;
            encCtx->opts[lenEnd].price = CMPT_INFINITY_PRICE;
        }
        const uint32_t lenEqualMem = lenEqual;
        const uint32_t prefixPrice = repMatchPrice + CmptPriceLongRep(encCtx, mainRepIndex, curState, posState);
        CmptlzDpTryCurAndLong(encCtx, prefixPrice, cur, mainRepIndex, lenEqual, posState);
        lenEqual = lenEqualMem;
        if (mainRepIndex == 0)
        {
            startLen = lenEqual + 1;
        }
    }
    uint32_t newLongestLen = encCtx->longestMatchLen;
    uint32_t matchCount = encCtx->matchesCount;
    if (newLongestLen > bufAvail)
    {
        newLongestLen = bufAvail;
        matchCount = 0;
        while (newLongestLen > encCtx->matches[matchCount].len)
        {
            ++matchCount;
        }
        encCtx->matches[matchCount++].len = newLongestLen;
    }
    if (newLongestLen >= startLen)
    {
        const uint32_t normalmatch_prefixPrice = matchPrice + CmptPriceBit0(encCtx, encCtx->isRep[curState]);
        while (lenEnd < cur + newLongestLen)
        {
            lenEnd++;
            encCtx->opts[lenEnd].price = CMPT_INFINITY_PRICE;
        }
        CmptlzDpTryCurAndMatch(encCtx, startLen, matchCount, normalmatch_prefixPrice, cur, posState);
    }
    return lenEnd;
}

static void CmptlzDpReverse(CmptLzEncCtx *encCtx, uint32_t cur)
{
    encCtx->optEndIndex = cur;
    uint32_t posTmp = encCtx->opts[cur].posPrev;
    uint32_t backTmp = encCtx->opts[cur].backPrev;
    uint32_t posPrev, backCurPacket;
    do
    {
        posPrev = posTmp;
        backCurPacket = backTmp;
        backTmp = encCtx->opts[posPrev].backPrev;
        posTmp = encCtx->opts[posPrev].posPrev;
        encCtx->opts[posPrev].backPrev = backCurPacket;
        encCtx->opts[posPrev].posPrev = cur;
        cur = posPrev;
    } while (cur != 0);
    encCtx->lenRes = encCtx->opts[0].posPrev;
    encCtx->backRes = encCtx->opts[0].backPrev;
    encCtx->optsCurIndex = encCtx->opts[0].posPrev;
}

void CmptlzDp(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t position)
{
    uint32_t curIndex = encCtx->optsCurIndex;
    uint32_t endIndex = encCtx->optEndIndex;
    if (endIndex != curIndex)
    {
        encCtx->lenRes = encCtx->opts[curIndex].posPrev - curIndex;
        encCtx->backRes = encCtx->opts[curIndex].backPrev;
        encCtx->optsCurIndex = encCtx->opts[curIndex].posPrev;
        return;
    }
    uint32_t lenEnd = CmptlzDpInit(encCtx, mf, position);
    if (lenEnd == CMPTLZ_UINT32_MAX)
    {
        return;
    }
    uint32_t mainReps[CMPTLZ_NUM_REPS];
    memcpy_s(mainReps, sizeof(mainReps), encCtx->reps, sizeof(encCtx->reps));
    uint32_t cur;
    for (cur = 1; cur < lenEnd; cur++)
    {
        encCtx->longestMatchLen = CmptlzMatchFinder(mf, &encCtx->matchesCount, encCtx->matches);
        if (encCtx->longestMatchLen >= mf->niceLen)
        {
            break;
        }
        CmptlzDpPre(encCtx, mainReps, cur);
        lenEnd = CmptlzDpProcess(encCtx, mf, mainReps, lenEnd, position + cur, cur);
    }
    CmptlzDpReverse(encCtx, cur);
    return;
}

int CmptHeadWrite(CmptLzEncCtx *encCtx, uint8_t *protData, size_t *propsSize)
{
    if (protData == NULL)
    {
        CMPTLZ_LOG(CMPT_ERROR_DATA, "protData is NULL");
        return CMPT_ENC_ERROR_HEAD;
    }
    if (*propsSize < CMPTLZ_PROPS_SIZE)
    {
        CMPTLZ_LOG(CMPT_ERROR_DATA, "propsSize need 5 bytes, get %zu", *propsSize);
        return CMPT_ENC_ERROR_HEAD;
    }
    CmptlzWriteLE32Bit(protData + 1, encCtx->dicSize);
    protData[0] = (encCtx->posBits * CMPTLZ_POS_STATE_MAX + encCtx->litPos) * CMPTLZ_LIT_CTX_MAX + encCtx->litCtx;
    *propsSize = CMPTLZ_PROPS_SIZE;
    return 0;
}

void CmptlzParamNormalize(CmptlzEncParam *props)
{
    int level = props->level;
    if (level < 0 || level > 9)
    {
        level = 5;
    }
    props->level = level;
    if (props->dictSize < CMPTLZ_MIN_DICTSIZE || props->dictSize > CMPTLZ_MAX_DICTSIZE)
    {
        CMPTLZ_SET_DICTSIZE_BY_LEVEL(level, props->dictSize);
    }
    if (props->fastBytes < 5 || props->fastBytes > CMPT_MF_LONGEST_MATCH)
    {
        CMPTLZ_SET_FB_BY_LEVEL(level, props->fastBytes);
    }
    if (props->litCtx < 0 || props->litCtx > CMPTLZ_LC_MAX)
    {
        props->litCtx = 3;
    }
    if (props->litPos < 0 || props->litPos > CMPTLZ_LP_MAX)
    {
        props->litPos = 0;
    }
    if (props->posBits < 0 || props->posBits > CMPTLZ_PB_MAX)
    {
        props->posBits = 2;
    }
    props->numThreads = 1;
}

void CmptlzSetParam(CmptLzEncCtx *encCtx, const CmptlzEncParam *props)
{
    CmptlzEncParam param = *props;
    CmptlzParamNormalize(&param);
    encCtx->dicSize = param.dictSize;
    encCtx->numFastBytes = param.fastBytes;
    encCtx->litCtx = param.litCtx;
    encCtx->litPos = param.litPos;
    encCtx->posBits = param.posBits;
    uint32_t i;
    for (i = 7; i < 32; i++)
    {
        if (encCtx->dicSize <= (uint32_t)(1 << i))
        {
            break;
        }
    }
    encCtx->distTableSize = i * 2;
}

void CmptlzPriceInit(CmptLzEncCtx *encCtx)
{
    CmptPriceGenRootTable(encCtx);
    CmptPriceGenDistTable(encCtx);
    CmptPriceGenAlignTable(encCtx);
}

void CmptlzEncPrepare(CmptLzEncCtx *encCtx)
{
    uint32_t i;
    uint32_t j;
    encCtx->encNeedFinish = false;
    encCtx->cmptlzResponse = 0;
    encCtx->nowpos64 = 0;
    encCtx->state = 0;
    encCtx->pbMask = (1 << encCtx->posBits) - 1;
    encCtx->lpMask = ((uint32_t)0x100 << encCtx->litPos) - ((uint32_t)0x100 >> encCtx->litCtx);
    encCtx->posMask = (1 << encCtx->posBits) - 1;
    for (i = 0; i < CMPTLZ_NUM_REPS; i++)
    {
        encCtx->reps[i] = 0;
    }
    encCtx->optsCurIndex = 0;
    encCtx->optEndIndex = 0;
    for (i = 0; i < CMPT_DP_OPTMAX; i++)
    {
        encCtx->opts[i].price = CMPT_INFINITY_PRICE;
    }
    for (i = 0; i < CMPTLZ_NUM_STATES; i++)
    {
        for (j = 0; j < CMPTLZ_NUM_PB_STATES_MAX; j++)
        {
            encCtx->isMatch[i][j] = CMPTLZ_PROB_INIT;
            encCtx->isRep0Long[i][j] = CMPTLZ_PROB_INIT;
        }
        encCtx->isRep[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG0[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG1[i] = CMPTLZ_PROB_INIT;
        encCtx->isRepG2[i] = CMPTLZ_PROB_INIT;
    }
    for (i = 0; i < CMPTLZ_DIST_STATE_TOTAL; i++)
    {
        for (j = 0; j < (1 << CMPTLZ_DIST_SLOT_BITS); j++)
        {
            encCtx->probDistSlot[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    for (i = 0; i < CMPT_DIST_LIMIT_2; i++)
    {
        encCtx->probDistSpecial[i] = CMPTLZ_PROB_INIT;
    }
    for (i = 0; i < (1 << CMPTLZ_ALIGN_BITS); i++)
    {
        encCtx->probAlign[i] = CMPTLZ_PROB_INIT;
    }
    encCtx->litMarcov.lcBits = encCtx->litCtx;
    encCtx->litMarcov.posMask = (1U << encCtx->litPos) - 1;
    for (i = 0; i < (1 << CMPTLZ_LCLP_MAX); i++)
    {
        for (j = 0; j < CMPTLZ_LIT_MAX_SIZE; j++)
        {
            encCtx->litMarcov.literal[i][j] = CMPTLZ_PROB_INIT;
        }
    }
    for (i = 0; i < (1 << CMPT_LEN_HIGH_BITS); i++)
    {
        encCtx->matchLenEncoder.high[i] = CMPTLZ_PROB_INIT;
        encCtx->repLenEncoder.high[i] = CMPTLZ_PROB_INIT;
        encCtx->matchLenEncoder.low[i] = CMPTLZ_PROB_INIT;
        encCtx->repLenEncoder.low[i] = CMPTLZ_PROB_INIT;
    }
    CmptlzPriceInit(encCtx);
    encCtx->repLenEncoder.tableSize = encCtx->numFastBytes - 1;
    encCtx->matchLenEncoder.tableSize = encCtx->numFastBytes - 1;
    CmptPriceGenLenTable(encCtx, &encCtx->matchLenEncoder);
    CmptPriceGenLenTable(encCtx, &encCtx->repLenEncoder);
}

void *CmptInitCctx(CmptLzMemHook *alloc, int writeEndMark)
{
    void *handle = alloc->CmptLzAlloc(CMPTLZ_ENC_CCTX_HANDLE, sizeof(CmptLzEncCtx));
    if (handle == NULL)
    {
        return NULL;
    }
    memset_s(handle, sizeof(CmptLzEncCtx), 0, sizeof(CmptLzEncCtx));
    CmptLzEncCtx *encCtx = (CmptLzEncCtx *)handle;
    encCtx->endMarker = writeEndMark;
    encCtx->rcCtx = NULL;
    encCtx->mfCtx = NULL;
    return encCtx;
}

static uint32_t CmptMemCmpByOneByte(const uint8_t *buf1, const uint8_t *buf2, uint32_t len, uint32_t limit)
{
    uint32_t lenIn = len;
    while ((lenIn < limit) && (buf1[lenIn] == buf2[lenIn]))
    {
        lenIn++;
    }
    return lenIn;
}

static uint32_t CmptMemCmpLenSafe(const uint8_t *buf1, const uint8_t *buf2, uint32_t len, uint32_t limit)
{
    return CmptMemCmpByOneByte(buf1, buf2, len, limit);
}

static uint32_t CmptMfAvail(const CmptMfCtx *mf)
{
    return mf->srcLen - mf->readPos;
}

static const uint8_t *CmptMfGetPtr(const CmptMfCtx *mf)
{
    return mf->srcStart + mf->readPos;
}

static uint32_t PosSlotHelper(uint32_t n)
{
    uint32_t i = 31;
    if ((n & 0xFFFF0000) == 0)
    {
        n <<= 16;
        i = 15;
    }
    if ((n & 0xFF000000) == 0)
    {
        n <<= 8;
        i -= 8;
    }
    if ((n & 0xF0000000) == 0)
    {
        n <<= 4;
        i -= 4;
    }
    if ((n & 0xC0000000) == 0)
    {
        n <<= 2;
        i -= 2;
    }
    if ((n & 0x80000000) == 0)
        --i;
    return i;
}

static uint32_t PosSloter(uint32_t dist)
{
    if (dist <= 4)
    {
        return dist;
    }
    uint32_t helper = PosSlotHelper(dist);
    return (helper + helper + ((dist >> (helper - 1)) & 1));
}

static void CmptlzMfGenHashTable(CmptMfCtx *mf)
{
    uint32_t *hashRootTable = mf->hashRootTable;
    const uint32_t poly32 = 0xEDB88320;
    uint32_t i, j;
    for (i = 0; i < CMPT_MF_HASH_TABLE_SIZE; i++)
    {
        uint32_t value = i;
        for (j = 0; j < 8; j++)
        {
            if (value & 1)
            {
                value = (value >> 1) ^ poly32;
            }
            else
            {
                value >>= 1;
            }
        }
        hashRootTable[i] = value;
    }
    return;
}

int CmptMfPrepare(CmptLzEncCtx *encCtx, const uint8_t *src, size_t srcLen, CmptLzMemHook *alloc)
{
    CmptMfCtx *mf = alloc->CmptLzAlloc(CMPTLZ_MF_CCTX_HANDLE, sizeof(CmptMfCtx));
    if (mf == NULL)
    {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    memset_s(mf, sizeof(CmptMfCtx), 0, sizeof(CmptMfCtx));
    encCtx->mfCtx = mf;
    mf->cycleSize = encCtx->dicSize + 1;
    uint32_t hashMask = encCtx->dicSize - 1;
    CMPT_HASH_MASK_CALC(hashMask);
    mf->hashMask = hashMask;
    ++hashMask;
    hashMask += CMPTLZ_HASH_2_SIZE;
    hashMask += CMPTLZ_HASH_3_SIZE;
    mf->hashCount = hashMask;
    mf->sonsCount = mf->cycleSize * 2;
    mf->hash = NULL;
    mf->son = NULL;
    mf->hash = alloc->CmptLzAlloc(CMPTLZ_MF_HASH_HANDLE, mf->hashCount * sizeof(uint32_t));
    memset_s(mf->hash, mf->hashCount * sizeof(uint32_t), 0, mf->hashCount * sizeof(uint32_t));
    if (mf->hash == NULL)
    {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    mf->son = alloc->CmptLzAlloc(CMPTLZ_MF_SON_HANDLE, mf->sonsCount * sizeof(uint32_t));
    memset_s(mf->son, mf->sonsCount * sizeof(uint32_t), 0, mf->sonsCount * sizeof(uint32_t));
    if (mf->son == NULL)
    {
        return CMPT_ENC_MF_INIT_FAIL;
    }
    CmptlzMfGenHashTable(mf);
    mf->srcStart = src;
    mf->srcLen = srcLen;
    mf->offset = mf->cycleSize;
    mf->niceLen = encCtx->numFastBytes;
    mf->depth = CMPT_MF_BASE_DEPTH + mf->niceLen / 2;
    return 0;
}

static void CmptMfMovePos(CmptMfCtx *mf)
{
    const uint32_t subvalue = (CMPTLZ_UINT32_MAX - mf->cycleSize);
    uint32_t i;
    for (i = 0; i < mf->hashCount; i++)
    {
        if (mf->hash[i] <= subvalue)
        {
            mf->hash[i] = CMPT_EMPTY_HASH_VALUE;
        }
        else
        {
            mf->hash[i] -= subvalue;
        }
    }
    for (i = 0; i < mf->sonsCount; ++i)
    {
        if (mf->son[i] <= subvalue)
        {
            mf->son[i] = CMPT_EMPTY_HASH_VALUE;
        }
        else
        {
            mf->son[i] -= subvalue;
        }
    }
    mf->offset -= subvalue;
}

static CmptlzMatchPair *CmptBtFind(CmptMfCtx *mf, uint32_t curMatch, CmptlzMatchPair *matches, uint32_t longestLen)
{
    uint32_t depth = mf->depth;
    uint32_t *const son = mf->son;
    const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
    const uint32_t niceLen = mf->niceLen;
    const uint32_t cyclePos = mf->cyclePos;
    const uint32_t cycleSize = mf->cycleSize;
    const uint32_t pos = mf->readPos + mf->offset;
    uint32_t *ptr0 = son + (cyclePos << 1) + 1;
    uint32_t *ptr1 = son + (cyclePos << 1);
    uint32_t len0 = 0;
    uint32_t len1 = 0;
    while (true)
    {
        const uint32_t delta = pos - curMatch;
        if (depth-- == 0 || delta >= cycleSize)
        {
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return matches;
        }
        uint32_t *const pair = son + ((cyclePos - delta + ((delta > cyclePos) ? cycleSize : 0)) << 1);
        const uint8_t *const pb = cur - delta;
        uint32_t len = CMPTLZ_FIND_MIN(len0, len1);
        if (pb[len] == cur[len])
        {
            len = CmptMemCmpLenSafe(pb, cur, len + 1, niceLen);
            if (longestLen < len)
            {
                longestLen = len;
                matches->len = len;
                matches->dist = delta - 1;
                ++matches;
                if (len == niceLen)
                {
                    *ptr1 = pair[0];
                    *ptr0 = pair[1];
                    return matches;
                }
            }
        }
        if (pb[len] < cur[len])
        {
            CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len);
        }
        else
        {
            CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len);
        }
    }
}

static void CmptBtSkip(CmptMfCtx *mf, const uint32_t lenLimit, const uint32_t pos, const uint8_t *const cur,
                       uint32_t curMatch)
{
    uint32_t depth = mf->depth;
    uint32_t *const son = mf->son;
    const uint32_t cyclePos = mf->cyclePos;
    const uint32_t cycleSize = mf->cycleSize;
    uint32_t *ptr0 = son + (cyclePos << 1) + 1;
    uint32_t *ptr1 = son + (cyclePos << 1);
    uint32_t len0 = 0;
    uint32_t len1 = 0;
    while (true)
    {
        const uint32_t delta = pos - curMatch;
        if (depth-- == 0 || delta >= cycleSize)
        {
            *ptr0 = CMPT_EMPTY_HASH_VALUE;
            *ptr1 = CMPT_EMPTY_HASH_VALUE;
            return;
        }
        uint32_t *pair = son + ((cyclePos - delta + ((delta > cyclePos) ? cycleSize : 0)) << 1);
        const uint8_t *pb = cur - delta;
        uint32_t len = CMPTLZ_FIND_MIN(len0, len1);
        if (pb[len] == cur[len])
        {
            len = CmptMemCmpLenSafe(pb, cur, len + 1, lenLimit);
            if (len == lenLimit)
            {
                *ptr1 = pair[0];
                *ptr0 = pair[1];
                return;
            }
        }
        if (pb[len] < cur[len])
        {
            CMPT_MF_LEFT_SON_UPDATE(ptr1, pair, curMatch, len1, len);
        }
        else
        {
            CMPT_MF_RIGHT_SON_UPDATE(ptr0, pair, curMatch, len0, len);
        }
    }
}

static uint32_t CmptlzBt4Finder(CmptMfCtx *mf, CmptlzMatchPair *matches)
{
    const uint32_t niceLen = mf->niceLen;
    const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
    const uint32_t pos = mf->readPos + mf->offset;
    uint32_t temp, hash2Value, hash3Value, hashValue;
    uint32_t longestLen = 1, matchesCount = 0;
    CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue);
    uint32_t delta2 = pos - mf->hash[hash2Value];
    uint32_t delta3 = pos - mf->hash[CMPTLZ_FIX_3_HASH + hash3Value];
    uint32_t curMatch = mf->hash[CMPTLZ_FIX_4_HASH + hashValue];
    CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos);
    CMPT_HASH_FIND_2_BYTES(mf, delta2, longestLen, matchesCount, cur, matches);
    CMPT_HASH_FIND_3_BYTES(mf, delta2, delta3, longestLen, matchesCount, cur, matches);
    if (matchesCount != 0)
    {
        longestLen = CmptMemCmpLenSafe(cur, cur - delta2, longestLen, niceLen);
        matches[matchesCount - 1].len = longestLen;
        if (longestLen == niceLen)
        {
            CmptBtSkip(mf, niceLen, pos, cur, curMatch);
            CMPT_MF_MOVE_POS(mf);
            return matchesCount;
        }
    }
    if (longestLen < CMPT_MF_MATCH_3_BYTES)
    {
        longestLen = CMPT_MF_MATCH_3_BYTES;
    }
    matchesCount = (uint32_t)(CmptBtFind(mf, curMatch, matches + matchesCount, longestLen) - matches);
    CMPT_MF_MOVE_POS(mf);
    return matchesCount;
}

void CmptlzMatchSkiper(CmptMfCtx *mf, uint32_t amount)
{
    mf->readAhead += amount;
    uint32_t pos, temp, hash2Value, hash3Value, hashValue, curMatch;
    const uint32_t niceLen = mf->niceLen;
    do
    {
        uint32_t lenLimit = mf->srcLen - mf->readPos;
        if (CMPTLZ_LIKELY(niceLen <= lenLimit))
        {
            lenLimit = niceLen;
        }
        else
        {
            mf->readPos++;
            continue;
        }
        const uint8_t *cur = (const uint8_t *)(mf->srcStart + mf->readPos);
        pos = mf->readPos + mf->offset;
        CMPT_HASH_4_CALC(mf, cur, temp, hash2Value, hash3Value, hashValue);
        curMatch = mf->hash[CMPTLZ_FIX_4_HASH + hashValue];
        CMPT_HASH_UPDATE(mf, hash2Value, hash3Value, hashValue, pos);
        CmptBtSkip(mf, lenLimit, pos, cur, curMatch);
        CMPT_MF_MOVE_POS(mf);
    } while (--amount != 0);
}

uint32_t CmptlzMatchFinder(CmptMfCtx *mf, uint32_t *pCount, CmptlzMatchPair *matches)
{
    if (CMPTLZ_UNLIKELY(mf->srcLen - mf->readPos < mf->niceLen))
    {
        *pCount = 0;
        mf->readPos++;
        mf->readAhead++;
        return 0;
    }
    const uint32_t count = CmptlzBt4Finder(mf, matches);
    if (count == 0)
    {
        *pCount = 0;
        mf->readAhead++;
        return 0;
    }
    uint32_t longestLen = matches[count - 1].len;
    if (longestLen == mf->niceLen)
    {
        uint32_t bytesAvail = CMPTLZ_FIND_MIN(mf->srcLen - mf->readPos + 1, CMPT_MF_LONGEST_MATCH);
        const uint8_t *p1 = (const uint8_t *)(mf->srcStart + mf->readPos - 1);
        const uint8_t *p2 = p1 - matches[count - 1].dist - 1;
        longestLen = CmptMemCmpLenSafe(p1, p2, longestLen, bytesAvail);
    }
    *pCount = count;
    mf->readAhead++;
    return longestLen;
}

static uint32_t CmptPriceOneBitDirect(uint32_t bit)
{
    return (bit << CMPT_PRICE_BITS_MOVING_NUM);
}

static uint32_t CmptPriceOneBit(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob, uint32_t curbit)
{
    return encCtx->priceRootTable[(bit0Prob ^ ((uint32_t)(0 - curbit) & (CMPTLZ_PROB_MAX_NUM - 1))) >>
                                  CMPT_PRICE_BITS_MOVING_NUM];
}

static uint32_t CmptPriceBit0(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob)
{
    return encCtx->priceRootTable[bit0Prob >> CMPT_PRICE_BITS_MOVING_NUM];
}

static uint32_t CmptPriceBit1(CmptLzEncCtx *encCtx, CmptlzProb bit0Prob)
{
    return encCtx->priceRootTable[(bit0Prob ^ (CMPTLZ_PROB_MAX_NUM - 1)) >> CMPT_PRICE_BITS_MOVING_NUM];
}

static uint32_t CmptPriceSymbol(CmptLzEncCtx *encCtx, CmptlzProb *symbolProbs, uint32_t symbolBitsNum, uint32_t symbol)
{
    uint32_t price = 0;
    symbol += (1U << symbolBitsNum);
    do
    {
        uint32_t bit = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[symbol], bit);
    } while (symbol != 1);
    return price;
}

static uint32_t CmptPriceSymbolReverse(CmptLzEncCtx *encCtx, CmptlzProb *symbolProbs, uint32_t symbolBitsNum,
                                       uint32_t symbol)
{
    uint32_t price = 0;
    uint32_t i = 1;
    do
    {
        uint32_t bit = symbol & 1;
        symbol >>= 1;
        price += CmptPriceOneBit(encCtx, symbolProbs[i], bit);
        i = (i << 1) + bit;
    } while (--symbolBitsNum);
    return price;
}

void CmptPriceGenRootTable(CmptLzEncCtx *encCtx)
{
    uint32_t *rootTable = encCtx->priceRootTable;
    const unsigned expandCycleNum = 4;
    const unsigned bitsTotalModeNum = 11;
    const unsigned valueForNormal = 15;
    const unsigned wTopBoarder = 1 << 16;
    for (unsigned i = 0; i < ((uint32_t)1 << bitsTotalModeNum >> CMPT_PRICE_BITS_MOVING_NUM); i++)
    {
        unsigned w = (i << CMPT_PRICE_BITS_MOVING_NUM) + (1 << (CMPT_PRICE_BITS_MOVING_NUM - 1));
        unsigned dummyNormalizeCnt = 0;
        for (unsigned j = 0; j < expandCycleNum; j++)
        {
            w = w * w;
            dummyNormalizeCnt <<= 1;
            while (w >= wTopBoarder)
            {
                w >>= 1;
                dummyNormalizeCnt++;
            }
        }
        rootTable[i] = (uint32_t)((bitsTotalModeNum << expandCycleNum) - valueForNormal - dummyNormalizeCnt);
    }
}

void CmptPriceGenDistTable(CmptLzEncCtx *encCtx)
{
    uint32_t distState = 0;
    do
    {
        uint32_t *const tmpPriceDistSlot = encCtx->priceDistSlotTable[distState];
        for (uint32_t i = 0; i < encCtx->distTableSize; i++)
        {
            tmpPriceDistSlot[i] = CmptPriceSymbol(encCtx, encCtx->probDistSlot[distState], CMPTLZ_DIST_SLOT_BITS, i);
        }
        for (uint32_t i = 14; i < encCtx->distTableSize; i++)
        {
            tmpPriceDistSlot[i] += CmptPriceOneBitDirect((i >> 1) - 1 - CMPTLZ_ALIGN_BITS);
        }
        for (uint32_t i = 0; i < 4; i++)
        {
            encCtx->priceDistTable[distState][i] = tmpPriceDistSlot[i];
        }
        distState++;
    } while (distState < CMPTLZ_DIST_STATE_TOTAL);
    for (uint32_t i = 4; i < 128; i++)
    {
        uint32_t distSlot = PosSloter(i);
        uint32_t footerBits = (distSlot >> 1) - 1;
        uint32_t base = (2 | (distSlot & 1)) << footerBits;
        uint32_t price =
            CmptPriceSymbolReverse(encCtx, encCtx->probDistSpecial + base - distSlot - 1, footerBits, i - base);
        for (distState = 0; distState < 4; distState++)
        {
            encCtx->priceDistTable[distState][i] = price + encCtx->priceDistSlotTable[distState][distSlot];
        }
    }
    encCtx->matchPriceCount = 0;
}

void CmptPriceGenAlignTable(CmptLzEncCtx *encCtx)
{
    for (uint32_t i = 0; i < (1 << CMPTLZ_ALIGN_BITS); i++)
    {
        encCtx->priceAlignTable[i] = CmptPriceSymbolReverse(encCtx, encCtx->probAlign, CMPTLZ_ALIGN_BITS, i);
    }
}

uint32_t CmptPriceLiteral(CmptLzEncCtx *encCtx, bool matchMode, uint32_t matchByte, uint32_t symbol)
{
    uint32_t pos = encCtx->litMarcov.pos;
    uint32_t prevByte = encCtx->litMarcov.prevByte;
    uint32_t litCtx = encCtx->litMarcov.lcBits;
    uint32_t lpMask = encCtx->litMarcov.posMask;
    CmptlzProb *subCoder = CMPT_LIT_SUBCODER(encCtx->litMarcov.literal, litCtx, lpMask, pos, prevByte);
    uint32_t price = 0;
    if (!matchMode)
    {
        price = CmptPriceSymbol(encCtx, subCoder, 8, symbol);
    }
    else
    {
        uint32_t offset = 0x100;
        symbol += 1 << 8;
        do
        {
            matchByte <<= 1;
            const uint32_t matchBit = matchByte & offset;
            const uint32_t subCoderIndex = offset + matchBit + (symbol >> 8);
            const uint32_t bit = (symbol >> 7) & 1;
            price += CmptPriceOneBit(encCtx, subCoder[subCoderIndex], bit);
            symbol <<= 1;
            offset &= ~(matchByte ^ symbol);
        } while (symbol < (1 << 16));
    }
    return price;
}

static void CmptPriceSet(CmptLzEncCtx *encCtx, const CmptlzProb *probs, uint32_t startPrice, uint32_t *prices)
{
    uint32_t i;
    for (i = 0; i < 8; i += 2)
    {
        uint32_t price = startPrice;
        uint32_t prob;
        price += CmptPriceOneBit(encCtx, probs[1], (i >> 2));
        price += CmptPriceOneBit(encCtx, probs[2 + (i >> 2)], (i >> 1) & 1);
        prob = probs[4 + (i >> 1)];
        prices[i] = price + CmptPriceBit0(encCtx, prob);
        prices[i + 1] = price + CmptPriceBit1(encCtx, prob);
    }
}

void CmptPriceGenLenTable(CmptLzEncCtx *encCtx, CmptLenEncoder *lenEncoder)
{
    const uint32_t numPosStates = 1 << encCtx->posBits;
    uint32_t b;
    uint32_t prob = lenEncoder->low[0];
    uint32_t a, c;
    uint32_t posState;
    b = CmptPriceBit1(encCtx, prob);
    a = CmptPriceBit0(encCtx, prob);
    c = b + CmptPriceBit0(encCtx, lenEncoder->low[1 << CMPT_LEN_LOW_BITS]);
    for (posState = 0; posState < numPosStates; posState++)
    {
        uint32_t *prices = lenEncoder->prices[posState];
        const CmptlzProb *probs = lenEncoder->low + (posState << (1 + CMPT_LEN_LOW_BITS));
        CmptPriceSet(encCtx, probs, a, prices);
        CmptPriceSet(encCtx, probs + (1 << CMPT_LEN_LOW_BITS), c, prices + (1 << CMPT_LEN_LOW_BITS));
    }
    uint32_t i = lenEncoder->tableSize;
    if (i > (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE)
    {
        const CmptlzProb *probs = lenEncoder->high;
        uint32_t *prices = lenEncoder->prices[0] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE;
        i -= (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE - 1;
        i >>= 1;
        b += CmptPriceBit1(encCtx, lenEncoder->low[(1 << CMPT_LEN_LOW_BITS)]);
        do
        {
            uint32_t sym = --i + (1 << (CMPT_LEN_HIGH_BITS - 1));
            uint32_t price = b;
            do
            {
                uint32_t bit = sym & 1;
                sym >>= 1;
                price += CmptPriceOneBit(encCtx, probs[sym], bit);
            } while (sym >= 2);
            prob = probs[(size_t)i + (1 << (CMPT_LEN_HIGH_BITS - 1))];
            prices[(size_t)i * CMPT_DOUBLE] = price + CmptPriceBit0(encCtx, prob);
            prices[(size_t)i * CMPT_DOUBLE + 1] = price + CmptPriceBit1(encCtx, prob);
        } while (i);
        size_t num =
            (lenEncoder->tableSize - (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE) * sizeof(lenEncoder->prices[0][0]);
        for (posState = 1; posState < numPosStates; posState++)
        {
            memcpy_s(lenEncoder->prices[posState] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE, CMPT_MF_LONGEST_MATCH - 1,
                     lenEncoder->prices[0] + (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE, num);
        }
    }
}

uint32_t CmptPriceLen(CmptLenEncoder *lenEncoder, uint32_t len, uint32_t posState)
{
    return lenEncoder->prices[posState][len - CMPTLZ_MATCH_LEN_MIN];
}

uint32_t CmptPriceShortRep(CmptLzEncCtx *encCtx, CmptlzState state, uint32_t posState)
{
    return CmptPriceBit0(encCtx, encCtx->isRepG0[state]) + CmptPriceBit0(encCtx, encCtx->isRep0Long[state][posState]);
}

uint32_t CmptPriceLongRep(CmptLzEncCtx *encCtx, uint32_t longRepIndex, CmptlzState state, uint32_t posState)
{
    uint32_t price = 0;
    switch (longRepIndex)
    {
    case 0:
        price =
            CmptPriceBit0(encCtx, encCtx->isRepG0[state]) + CmptPriceBit1(encCtx, encCtx->isRep0Long[state][posState]);
        break;
    case 1:
        price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) + CmptPriceBit0(encCtx, encCtx->isRepG1[state]);
        break;
    case 2:
        price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) + CmptPriceBit1(encCtx, encCtx->isRepG1[state]) +
                CmptPriceBit0(encCtx, encCtx->isRepG2[state]);
        break;
    case 3:
        price = CmptPriceBit1(encCtx, encCtx->isRepG0[state]) + CmptPriceBit1(encCtx, encCtx->isRepG1[state]) +
                CmptPriceBit1(encCtx, encCtx->isRepG2[state]);
        break;
    default:
        break;
    }
    return price;
}

uint32_t CmptPriceDistWithLen(CmptLzEncCtx *encCtx, uint32_t dist, uint32_t len, uint32_t posState)
{
    const uint32_t distState = CMPT_GET_DIST_STATE(len);
    uint32_t price;
    if (dist < 128)
    {
        price = encCtx->priceDistTable[distState][dist];
    }
    else
    {
        uint32_t distSlot = PosSloter(dist);
        price = encCtx->priceDistSlotTable[distState][distSlot] +
                encCtx->priceAlignTable[dist & ((1 << CMPTLZ_ALIGN_BITS) - 1)];
    }
    price += CmptPriceLen(&encCtx->matchLenEncoder, len, posState);
    return price;
}

static int CmptRcLitProcess(CmptRcCtx *rcCtx, CmptlzProb *prob, uint32_t sym)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range, bit0Prob, newBound, curBit;
    for (sym |= 0x100; sym < 0x10000; sym <<= 1)
    {
        CmptlzProb *litProbTableIndex = prob + (sym >> 8);
        curBit = (sym >> 7) & 1;
        CMPT_RC_BIT_PROCESS(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    rcCtx->range = range;
    return CMPT_OK;
}

static int CmptRcLitAfterMatch(CmptRcCtx *rcCtx, CmptlzProb *prob, uint32_t sym, uint32_t matchByte)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range, offs = 0x100, bit0Prob, newBound, curBit;
    for (sym |= 0x100; sym < 0x10000;)
    {
        matchByte <<= 1;
        CmptlzProb *litProbTableIndex = prob + (offs + (matchByte & offs) + (sym >> 8));
        curBit = (sym >> 7) & 1;
        sym <<= 1;
        offs &= ~(matchByte ^ sym);
        CMPT_RC_BIT_PROCESS(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    rcCtx->range = range;
    return CMPT_OK;
}

int CmptRcPrepare(CmptLzEncCtx *encCtx, uint8_t *dest, size_t *destLen, CmptLzMemHook *alloc)
{
    CmptRcCtx *rc = alloc->CmptLzAlloc(CMPTLZ_RC_CCTX_HANDLE, sizeof(CmptRcCtx));
    if (rc == NULL) {
        return CMPT_ENC_RC_INIT_FAIL;
    }
    memset_s(rc, sizeof(CmptRcCtx), 0, sizeof(CmptRcCtx));
    encCtx->rcCtx = rc;
    rc->bufBase = alloc->CmptLzAlloc(CMPTLZ_RC_BUF_HANDLE, CMPTLZ_RC_BUFFER_SIZE);
    memset_s(rc->bufBase, CMPTLZ_RC_BUFFER_SIZE, 0, CMPTLZ_RC_BUFFER_SIZE);
    if (rc->bufBase == NULL) {
        return CMPT_ENC_RC_INIT_FAIL;
    }
    rc->outBufLeft = *destLen;
    rc->outBuf = dest;
    rc->buf = rc->bufBase;
    rc->range = 0xFFFFFFFF;
    rc->cacheSize = 0;
    rc->cache = 0;
    rc->low = 0;
    return 0;
}

int CmptRcFlush64Kb(CmptRcCtx *rcCtx)
{
    size_t flushOutLen = rcCtx->buf - rcCtx->bufBase;
    int res = memcpy_s(rcCtx->outBuf, rcCtx->outBufLeft, rcCtx->bufBase, flushOutLen);
    if (res != 0) {
        return CMPT_ENC_ERROR_WRITE;
    }
    rcCtx->outBuf += flushOutLen;
    rcCtx->outBufLeft -= flushOutLen;
    rcCtx->buf = rcCtx->bufBase;
    return CMPT_OK;
}

int CmptRcShiftLow(CmptRcCtx *rcCtx)
{
    int res = CMPT_OK;
    uint32_t lowLow32 = (uint32_t)rcCtx->low;
    uint64_t high = (uint32_t)(rcCtx->low >> 32);
    rcCtx->low = (uint32_t)(lowLow32 << 8);
    CMPT_RC_BREAK_CHECK(rcCtx, rcCtx->buf, res);
    if (lowLow32 < 0xFF000000 || high != 0) {
        uint8_t *buf = rcCtx->buf;
        *(buf) = (uint8_t)(rcCtx->cache + high);
        buf++;
        rcCtx->buf = buf;
        rcCtx->cache = (uint8_t)(lowLow32 >> 24);
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
    } else {
        rcCtx->cacheSize++;
    }
    return res;
}

int CmptRcFlushData(CmptRcCtx *rcCtx)
{
    int i;
    int res;
    for (i = 0; i < 5; i++) {
        res = CmptRcShiftLow(rcCtx);
        if (res != CMPT_OK) {
            break;
        }
    }
    return res;
}

int CmptRcLenProcess(CmptLenEncoder *lenEncoder, CmptRcCtx *rcCtx, uint32_t len, uint64_t posState)
{
    int shiftRes = CMPT_OK;
    uint32_t range = rcCtx->range;
    uint32_t newBound, bit0Prob;
    len -= CMPTLZ_MATCH_LEN_MIN;
    CmptlzProb *probs = lenEncoder->low;
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    if (len >= CMPT_LEN_BOUND) {
        CMPT_RC_BIT_1_PROCESS(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs += CMPT_LEN_BOUND;
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        if (len >= CMPT_LEN_BOUND * CMPT_DOUBLE) {
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
    probs += (posState << (1 + 3));
    bit = (len >> 2);
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

int CmptRcPosSlotProcess(CmptLzEncCtx *encCtx,
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
    while (sym < (1 << (CMPTLZ_DIST_SLOT_BITS * 2)));
    encCtx->rcCtx->range = range;
    return CMPT_OK;
}

static int CmptRcReverseProcess(CmptRcCtx *rcCtx, CmptlzProb *probs,
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
            encCtx->rcCtx->low += range & (0 - (pos2 >> 31));
            pos2 += pos2;
            CMPT_RC_NORMALIZE(encCtx->rcCtx, range, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        }
        while (pos2 != 0xF0000000);
        uint32_t m = 1;
        uint32_t bit;
        uint32_t bit0Prob, newBound;
        int k;
        for (k = 0; k < CMPTLZ_ALIGN_BITS - 1; k++) {
            bit = dist & 1;
            dist >>= 1;
            CMPT_RC_BIT_PROCESS(encCtx->rcCtx, encCtx->probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
            CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
            m = (m << 1) + bit;
        }
        bit = dist & 1;
        CMPT_RC_BIT_PROCESS(encCtx->rcCtx, encCtx->probAlign + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        encCtx->rcCtx->range = range;
    }
    return CMPT_OK;
}

static int CmptlzEncLit(CmptLzEncCtx *encCtx, CmptMfCtx *mf, uint32_t nowpos32)
{
    int shiftRes = CMPT_OK;
    CmptRcCtx *rc = encCtx->rcCtx;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = rc->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(rc, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    rc->range = range;
    CmptlzProb *litProb = &encCtx->litMarcov.literal[0][0];
    const uint8_t curByte = mf->srcStart[mf->readPos - mf->readAhead];
    probs = CMPT_LIT_PROB_GET(encCtx, litProb, nowpos32, mf->srcStart[mf->readPos - mf->readAhead - 1]);
    CmptlzState state = encCtx->state;
    CMPT_STATE_UPDATE_WHEN_LIT(encCtx->state);
    if (state < 7)
    {
        shiftRes = CmptRcLitProcess(rc, probs, curByte);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    else
    {
        const uint8_t match_byte = mf->srcStart[mf->readPos - encCtx->reps[0] - 1 - mf->readAhead];
        shiftRes = CmptRcLitAfterMatch(rc, probs, curByte, match_byte);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

static int CmptlzEncShortRep(CmptLzEncCtx *encCtx, uint32_t nowpos32)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = encCtx->rcCtx->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRepG0[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRep0Long[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    encCtx->rcCtx->range = range;
    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_SHORTREP(state);
    return CMPT_OK;
}

static int CmptlzEncNormalMatch(CmptLzEncCtx *encCtx, uint32_t nowpos32, uint32_t backRes, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    range = encCtx->rcCtx->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    encCtx->rcCtx->range = range;
    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_MATCH(state);
    shiftRes = CmptRcLenProcess(&encCtx->matchLenEncoder, encCtx->rcCtx, lenRes, posState);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    backRes -= CMPTLZ_NUM_REPS;
    encCtx->reps[3] = encCtx->reps[2];
    encCtx->reps[2] = encCtx->reps[1];
    encCtx->reps[1] = encCtx->reps[0];
    encCtx->reps[0] = backRes;
    encCtx->matchPriceCount++;
    uint32_t posSlot = PosSloter(backRes);
    shiftRes = CmptRcPosSlotProcess(encCtx, posSlot, lenRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    if (backRes >= 4)
    {
        shiftRes = CmptRcDistProcess(encCtx, posSlot, backRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    }
    return CMPT_OK;
}

static int CmptlzEncLongRep(CmptLzEncCtx *encCtx, uint32_t repIndex, uint32_t nowpos32, uint32_t lenRes)
{
    int shiftRes = CMPT_OK;
    uint32_t posState = nowpos32 & encCtx->pbMask;
    uint32_t range, bit0Prob, newBound;
    uint32_t realDist;
    range = encCtx->rcCtx->range;
    CmptlzProb *probs = &encCtx->isMatch[encCtx->state][posState];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRep[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    probs = &encCtx->isRepG0[encCtx->state];
    CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
    switch (repIndex)
    {
    case 0:
        CMPT_RC_BIT_0_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRep0Long[encCtx->state][posState];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_1(encCtx->rcCtx, probs, newBound, range, bit0Prob);
        break;
    case 1:
        CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRepG1[encCtx->state];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_0(probs, newBound, range, bit0Prob);
        realDist = encCtx->reps[1];
        encCtx->reps[1] = encCtx->reps[0];
        encCtx->reps[0] = realDist;
        break;
    case 2:
        CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRepG1[encCtx->state];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRepG2[encCtx->state];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_0(probs, newBound, range, bit0Prob);
        realDist = encCtx->reps[2];
        encCtx->reps[2] = encCtx->reps[1];
        encCtx->reps[1] = encCtx->reps[0];
        encCtx->reps[0] = realDist;
        break;
    case 3:
        CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRepG1[encCtx->state];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_1_PROCESS(encCtx->rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
        probs = &encCtx->isRepG2[encCtx->state];
        CMPT_RC_GET_NEWBOUND(probs, bit0Prob, range, newBound);
        CMPT_RC_BIT_1(encCtx->rcCtx, probs, newBound, range, bit0Prob);
        realDist = encCtx->reps[3];
        encCtx->reps[3] = encCtx->reps[2];
        encCtx->reps[2] = encCtx->reps[1];
        encCtx->reps[1] = encCtx->reps[0];
        encCtx->reps[0] = realDist;
        break;
    default:
        break;
    }
    CMPT_RC_NORMALIZE(encCtx->rcCtx, range, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    encCtx->rcCtx->range = range;
    shiftRes = CmptRcLenProcess(&encCtx->repLenEncoder, encCtx->rcCtx, lenRes, posState);
    CMPTLZ_RETURN_IF_NOT_OK(shiftRes);
    --encCtx->repLenPriceCount;
    CmptlzState state = encCtx->state;
    encCtx->state = CMPT_STATE_UPDATE_WHEN_LONGREP(state);
    return CMPT_OK;
}

void CmptlzFreeAll(CmptLzEncCtx *encCtx, CmptLzMemHook *alloc)
{
    if (encCtx == NULL)
    {
        return;
    }
    if (encCtx->mfCtx != NULL)
    {
        if (encCtx->mfCtx->hash != NULL)
        {
            alloc->CmptLzFree(CMPTLZ_MF_HASH_HANDLE, encCtx->mfCtx->hash);
            encCtx->mfCtx->hash = NULL;
        }
        if (encCtx->mfCtx->son != NULL)
        {
            alloc->CmptLzFree(CMPTLZ_MF_SON_HANDLE, encCtx->mfCtx->son);
            encCtx->mfCtx->son = NULL;
        }
        alloc->CmptLzFree(CMPTLZ_MF_CCTX_HANDLE, encCtx->mfCtx);
        encCtx->mfCtx = NULL;
    }
    if (encCtx->rcCtx != NULL)
    {
        if (encCtx->rcCtx->bufBase != NULL)
        {
            alloc->CmptLzFree(CMPTLZ_RC_BUF_HANDLE, encCtx->rcCtx->bufBase);
            encCtx->rcCtx->bufBase = NULL;
        }
        alloc->CmptLzFree(CMPTLZ_RC_CCTX_HANDLE, encCtx->rcCtx);
        encCtx->rcCtx = NULL;
    }
    alloc->CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE, encCtx);
    encCtx = NULL;
}

int CmptlzEncodeIO(CmptLzEncCtx *encCtx, uint8_t *dest, size_t *destLen, const uint8_t *src, size_t srcLen,
                   CmptLzMemHook *alloc)
{
    int res;
    res = CmptMfPrepare(encCtx, src, srcLen, alloc);
    if (res != 0)
    {
        CMPTLZ_LOG(res, "CmptMfPrepare Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    res = CmptRcPrepare(encCtx, dest, destLen, alloc);
    if (res != 0)
    {
        CMPTLZ_LOG(res, "CmptRcPrepare Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    CmptlzEncPrepare(encCtx);
    res = CmptEncodeAll(encCtx);
    if (res != 0)
    {
        CmptlzFreeAll(encCtx, alloc);
        CMPTLZ_LOG(res, "CmptEncode Process Fail!");
        return res;
    }
    *destLen -= encCtx->rcCtx->outBufLeft;
    if (encCtx->nowpos64 != srcLen)
    {
        CMPTLZ_LOG(res, "CmptEncode FileSize Fail!");
        CmptlzFreeAll(encCtx, alloc);
        return CMPT_ENC_ERROR_FILESIZE;
    }
    CmptlzFreeAll(encCtx, alloc);
    return res;
}

int CmptlzEncode(uint8_t *dest, size_t *destLen, const uint8_t *src, size_t srcLen, const CmptlzEncParam *props,
                 uint8_t *propsEncoded, size_t *propsSize, int writeEndMark, CmptLzMemHook *alloc)
{
    int res;
    if (alloc == NULL || alloc->CmptLzAlloc == NULL || alloc->CmptLzFree == NULL)
    {
        CMPTLZ_LOG(CMPT_ENC_ERROR_PARAM, "Cmptlz input wrong param!");
        return CMPT_ENC_ERROR_PARAM;
    }
    CmptLzEncCtx *encCtx = (CmptLzEncCtx *)CmptInitCctx(alloc, writeEndMark);
    if (encCtx == NULL)
    {
        CMPTLZ_LOG(CMPT_ENC_CTX_INIT_FAIL, "CmptInitCctx Fail!");
        return CMPT_ENC_CTX_INIT_FAIL;
    }
    CmptlzSetParam(encCtx, props);
    res = CmptHeadWrite(encCtx, propsEncoded, propsSize);
    if (res != 0)
    {
        alloc->CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE, encCtx);
        CMPTLZ_LOG(res, "CmptHeadWrite Fail!");
        return res;
    }
    res = CmptlzEncodeIO(encCtx, dest, destLen, src, srcLen, alloc);
    if (res != 0)
    {
        CMPTLZ_LOG(res, "CmptlzEncode I / O Fail!");
    }
    return res;
}

int CmptlzCompress(void *src, size_t srcSize, void *dst, size_t *dstSize, CmptlzCompParam *param)
{
    if ((src == NULL) && (srcSize != 0))
    {
        return CMPT_ENC_ERROR_PARAM;
    }
    const int endMarker = 0;
    CmptlzEncParam props;
    props.level = param->level;
    props.dictSize = param->dictSize;
    props.litCtx = param->litCtx;
    props.litPos = param->litPos;
    props.posBits = param->posBits;
    props.fastBytes = param->fastBytes;
    props.numThreads = param->numThreads;
    CmptLzMemHook *alloc = param->memHook;
    return CmptlzEncode((uint8_t *)dst, dstSize, (const uint8_t *)src, srcSize, &props, param->protData,
                        &param->protSize, endMarker, alloc);
}

