enum BZP_ERROR_BASE_NO
{
    BZP_ERROR_MEMORY_OPER_FAILURE = 1,
    BZP_ERROR_PARAM,
    BZP_ERROR_IO,
    BZP_ERROR_DATA,
    BZP_ERROR_DATA_MAGIC,
};

typedef struct
{
    FILE *filePtr;
    int32_t nBuf;
    int32_t pos;
    uint8_t buf[BZP_BUF_SIZE];
} BzpStream;

typedef struct
{
    BzpStream *input;
    BzpStream *output;
    int32_t lasChar;
    int32_t num;
    uint32_t buf;
    int32_t nBuf;
    int32_t blockSize;
    uint32_t blockCRC;
    int32_t list[BZP_ASCII_SIZE];
} InDeComdata;

typedef struct
{
    BzpStream *input;
    BzpStream *output;
    int32_t state;
    int32_t lasChar;
    int32_t num;
    int32_t pad;
} BzpFile;

typedef struct
{
    uint8_t *out;
    int32_t num;
    uint32_t buf;
    int32_t nBuf;
    int32_t blockSize;
} BzpOutComdata;

typedef struct
{
    BzpBwtInfo *bwt;
    BzpHuffmanGroups *huffman;
    BzpMtfInfo *mtf;
    BzpFile *compressFile;
    BzpOutComdata *outData;
} BzpAlgorithmInfo;

typedef struct
{
    int32_t *sortBlock;
    int32_t *idx;
    int32_t *isStartPos;
    uint8_t *block;
    uint32_t blockCRC;
    uint32_t combinedCRC;
    int32_t nBlockMax;
    int32_t blockId;
    int32_t nBlock;
    int32_t oriPtr;
    bool inUse[BZP_ASCII_SIZE];
} BzpBwtInfo;

typedef struct
{
    int32_t stackL[BZP_MAX_STACK_SIZE];
    int32_t stackR[BZP_MAX_STACK_SIZE];
    int32_t cnt;
    int32_t tl, tr;
} BzpQSortInfo;

typedef struct
{
    uint8_t *block;
    int32_t *map;
    int32_t *mtfV;
    bool *inUse;
    int32_t mtfFreq[BZP_MAX_ALPHA_SIZE];
    int32_t nBlock;
    int32_t nMtf;
    int32_t nUse;
    int32_t pad;
} BzpMtfInfo;

typedef struct
{
    int32_t heap[BZP_MAX_ALPHA_SIZE + 1];
    int32_t weight[BZP_MAX_ALPHA_SIZE * 2];
    int32_t parent[BZP_MAX_ALPHA_SIZE * 2];
    int32_t len[BZP_MAX_ALPHA_SIZE];
    int32_t table[BZP_MAX_ALPHA_SIZE];
    int32_t nHeap;
    int32_t nWeight;
    int32_t alphaSize;
} BzpHuffmanInfo;

typedef struct
{
    int32_t *block;
    int32_t *mtfFreq;
    int32_t *select;
    int32_t *selectMTF;
    BzpHuffmanInfo huffmanGroups[BZP_MAX_GROUPS_NUM];
    int32_t cost[BZP_MAX_GROUPS_NUM];
    int32_t nGroups;
    int32_t nBlock;
    int32_t nSelect;
    int32_t alphaSize;
} BzpHuffmanGroups;

typedef struct
{
    int32_t *sorted;
    uint8_t *block;
    uint8_t *deCode;
    int32_t nBlock;
    int32_t oriPtr;
} BzpBwtDecodeInfo;

typedef struct
{
    int32_t *select;
    int32_t len[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];
    int32_t perm[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];
    int32_t limit[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];
    int32_t base[BZP_MAX_GROUPS_NUM][BZP_MAX_ALPHA_SIZE];
    int32_t minLens[BZP_MAX_GROUPS_NUM];
    int32_t nGroups;
    int32_t nSelect;
    int32_t alphaSize;
    int32_t deCodeNum;
    int32_t selectCnt;
    int32_t nBlock;
} BzpHuffmanDecode;

int32_t g_bzpCRC32Table[256] = {
    0x00000000L, 0x04c11db7L, 0x09823b6eL, 0x0d4326d9L, 0x130476dcL, 0x17c56b6bL, 0x1a864db2L, 0x1e475005L, 0x2608edb8L,
    0x22c9f00fL, 0x2f8ad6d6L, 0x2b4bcb61L, 0x350c9b64L, 0x31cd86d3L, 0x3c8ea00aL, 0x384fbdbdL, 0x4c11db70L, 0x48d0c6c7L,
    0x4593e01eL, 0x4152fda9L, 0x5f15adacL, 0x5bd4b01bL, 0x569796c2L, 0x52568b75L, 0x6a1936c8L, 0x6ed82b7fL, 0x639b0da6L,
    0x675a1011L, 0x791d4014L, 0x7ddc5da3L, 0x709f7b7aL, 0x745e66cdL, 0x9823b6e0L, 0x9ce2ab57L, 0x91a18d8eL, 0x95609039L,
    0x8b27c03cL, 0x8fe6dd8bL, 0x82a5fb52L, 0x8664e6e5L, 0xbe2b5b58L, 0xbaea46efL, 0xb7a96036L, 0xb3687d81L, 0xad2f2d84L,
    0xa9ee3033L, 0xa4ad16eaL, 0xa06c0b5dL, 0xd4326d90L, 0xd0f37027L, 0xddb056feL, 0xd9714b49L, 0xc7361b4cL, 0xc3f706fbL,
    0xceb42022L, 0xca753d95L, 0xf23a8028L, 0xf6fb9d9fL, 0xfbb8bb46L, 0xff79a6f1L, 0xe13ef6f4L, 0xe5ffeb43L, 0xe8bccd9aL,
    0xec7dd02dL, 0x34867077L, 0x30476dc0L, 0x3d044b19L, 0x39c556aeL, 0x278206abL, 0x23431b1cL, 0x2e003dc5L, 0x2ac12072L,
    0x128e9dcfL, 0x164f8078L, 0x1b0ca6a1L, 0x1fcdbb16L, 0x018aeb13L, 0x054bf6a4L, 0x0808d07dL, 0x0cc9cdcaL, 0x7897ab07L,
    0x7c56b6b0L, 0x71159069L, 0x75d48ddeL, 0x6b93dddbL, 0x6f52c06cL, 0x6211e6b5L, 0x66d0fb02L, 0x5e9f46bfL, 0x5a5e5b08L,
    0x571d7dd1L, 0x53dc6066L, 0x4d9b3063L, 0x495a2dd4L, 0x44190b0dL, 0x40d816baL, 0xaca5c697L, 0xa864db20L, 0xa527fdf9L,
    0xa1e6e04eL, 0xbfa1b04bL, 0xbb60adfcL, 0xb6238b25L, 0xb2e29692L, 0x8aad2b2fL, 0x8e6c3698L, 0x832f1041L, 0x87ee0df6L,
    0x99a95df3L, 0x9d684044L, 0x902b669dL, 0x94ea7b2aL, 0xe0b41de7L, 0xe4750050L, 0xe9362689L, 0xedf73b3eL, 0xf3b06b3bL,
    0xf771768cL, 0xfa325055L, 0xfef34de2L, 0xc6bcf05fL, 0xc27dede8L, 0xcf3ecb31L, 0xcbffd686L, 0xd5b88683L, 0xd1799b34L,
    0xdc3abdedL, 0xd8fba05aL, 0x690ce0eeL, 0x6dcdfd59L, 0x608edb80L, 0x644fc637L, 0x7a089632L, 0x7ec98b85L, 0x738aad5cL,
    0x774bb0ebL, 0x4f040d56L, 0x4bc510e1L, 0x46863638L, 0x42472b8fL, 0x5c007b8aL, 0x58c1663dL, 0x558240e4L, 0x51435d53L,
    0x251d3b9eL, 0x21dc2629L, 0x2c9f00f0L, 0x285e1d47L, 0x36194d42L, 0x32d850f5L, 0x3f9b762cL, 0x3b5a6b9bL, 0x0315d626L,
    0x07d4cb91L, 0x0a97ed48L, 0x0e56f0ffL, 0x1011a0faL, 0x14d0bd4dL, 0x19939b94L, 0x1d528623L, 0xf12f560eL, 0xf5ee4bb9L,
    0xf8ad6d60L, 0xfc6c70d7L, 0xe22b20d2L, 0xe6ea3d65L, 0xeba91bbcL, 0xef68060bL, 0xd727bbb6L, 0xd3e6a601L, 0xdea580d8L,
    0xda649d6fL, 0xc423cd6aL, 0xc0e2d0ddL, 0xcda1f604L, 0xc960ebb3L, 0xbd3e8d7eL, 0xb9ff90c9L, 0xb4bcb610L, 0xb07daba7L,
    0xae3afba2L, 0xaafbe615L, 0xa7b8c0ccL, 0xa379dd7bL, 0x9b3660c6L, 0x9ff77d71L, 0x92b45ba8L, 0x9675461fL, 0x8832161aL,
    0x8cf30badL, 0x81b02d74L, 0x857130c3L, 0x5d8a9099L, 0x594b8d2eL, 0x5408abf7L, 0x50c9b640L, 0x4e8ee645L, 0x4a4ffbf2L,
    0x470cdd2bL, 0x43cdc09cL, 0x7b827d21L, 0x7f436096L, 0x7200464fL, 0x76c15bf8L, 0x68860bfdL, 0x6c47164aL, 0x61043093L,
    0x65c52d24L, 0x119b4be9L, 0x155a565eL, 0x18197087L, 0x1cd86d30L, 0x029f3d35L, 0x065e2082L, 0x0b1d065bL, 0x0fdc1becL,
    0x3793a651L, 0x3352bbe6L, 0x3e119d3fL, 0x3ad08088L, 0x2497d08dL, 0x2056cd3aL, 0x2d15ebe3L, 0x29d4f654L, 0xc5a92679L,
    0xc1683bceL, 0xcc2b1d17L, 0xc8ea00a0L, 0xd6ad50a5L, 0xd26c4d12L, 0xdf2f6bcbL, 0xdbee767cL, 0xe3a1cbc1L, 0xe760d676L,
    0xea23f0afL, 0xeee2ed18L, 0xf0a5bd1dL, 0xf464a0aaL, 0xf9278673L, 0xfde69bc4L, 0x89b8fd09L, 0x8d79e0beL, 0x803ac667L,
    0x84fbdbd0L, 0x9abc8bd5L, 0x9e7d9662L, 0x933eb0bbL, 0x97ffad0cL, 0xafb010b1L, 0xab710d06L, 0xa6322bdfL, 0xa2f33668L,
    0xbcb4666dL, 0xb8757bdaL, 0xb5365d03L, 0xb1f740b4L};

#define BZP_OK 0

#define BZP_BASE_BLOCK_SIZE 100000

#define BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT 9

#define BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT 1

#define BZP_BLOCK_RESERVED_SPACE_SIZE 19

#define BZP_THRESHOLD_SHELL_SORT 10

#define BZP_MAX_STACK_SIZE 100

#define BZP_ASCII_SIZE 256

#define BZP_SHELL_SORT_INCREMENT_NUMS 2

#define BZP_SHELL_SORT_INCREMENT0 1

#define BZP_SHELL_SORT_INCREMENT1 4

#define BZP_MTF_ENCODE0 0

#define BZP_MTF_ENCODE1 1

#define BZP_MTF_ENCODE_BASE 2

#define BZP_INIT_BLOCK_CRC 0xffffffffL

#define BZP_MAX_ALPHA_SIZE 258

#define BZP_MAX_GROUPS_NUM 6

#define BZP_MAX_ITER_NUM 4

#define BZP_MAX_TREE_HEIGHT_ENCODE 17

#define BZP_NGROUPS_BLOCK_NUM_LIMIT0 200

#define BZP_NGROUPS_BLOCK_NUM_LIMIT1 600

#define BZP_NGROUPS_BLOCK_NUM_LIMIT2 1200

#define BZP_NGROUPS_BLOCK_NUM_LIMIT3 2400

#define BZP_NGROUPS_NUM_0 2

#define BZP_NGROUPS_NUM_1 3

#define BZP_NGROUPS_NUM_2 4

#define BZP_NGROUPS_NUM_3 5

#define BZP_NGROUPS_NUM_4 6

#define BZP_ELEMS_NUM_IN_ONE_GROUP 50

#define BZP_HUFFMAN_HEIGHT_WEIGHT_BITS 8

#define BZP_HUFFMAN_LEN_MAX_COST 15

#define BZP_HUFFMAN_LEN_UPPER_LIMIT 20

#define BZP_HUFFMAN_MAX_SIZE_SELECT                                                                                    \
    (BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP)


#define BZP_HDR_B 0x42

#define BZP_HDR_Z 0x5a

#define BZP_HDR_H 0x68

#define BZP_HDR_0 0x30

#define BZP_BLOCK_HEAD_0 0x31

#define BZP_BLOCK_HEAD_1 0x41

#define BZP_BLOCK_HEAD_2 0x59

#define BZP_BLOCK_HEAD_3 0x26

#define BZP_BLOCK_HEAD_4 0x53

#define BZP_BLOCK_HEAD_5 0x59

#define BZP_FILE_END_0 0x17

#define BZP_FILE_END_1 0x72

#define BZP_FILE_END_2 0x45

#define BZP_FILE_END_3 0x38

#define BZP_FILE_END_4 0x50

#define BZP_FILE_END_5 0x90

#define BZP_BUF_SIZE 5000

#define BZP_EOF (-1)

#define BZP_BIT 1

#define BZP_BITS2 2

#define BZP_BITS3 3

#define BZP_BITS5 5

#define BZP_BITS8 8

#define BZP_BITS15 15

#define BZP_BITS16 16

#define BZP_BITS24 24

#define BZP_BITS32 32

#define BZP_RLC_NUM_1 1

#define BZP_RLC_NUM_2 2

#define BZP_RLC_NUM_3 3

#define BZP_RLC_NUM_4 4

#define BZP_RLC_NUM_LOWER_LIMIT 1

#define BZP_RLC_NUM_UPPER_LIMIT 255

#define BZP_GROUPS_ASCII 16

#define BZP_CHARS_PER_GROUP_ASCII 16

#define BZP_CRC_MOVE_RIGHT_VAL 31

#define BZP_HUFFMAN_LEN_INCREASE 2

#define BZP_HUFFMAN_LEN_REDUCED 3

#define BZP_EXTRA_CHARS_NUM 2

#define BZP_INPUT_COMPRESS 0

#define BZP_OUTPUT_COMPRESS 1

#define BZP_RETUEN_COMPRESS 2

#define BZP_INVALID_BLOCK_SIZE(blockSize)                                                                              \
    ((blockSize) < BZP_BLOCK_SIZE_LEVEL_LOWER_LIMIT || (blockSize) > BZP_BLOCK_SIZE_LEVEL_UPPER_LIMIT)


#define BZP_INVALID_ALPHA_SIZE(alphaSize) ((alphaSize) > BZP_MAX_ALPHA_SIZE || (alphaSize) < 1)


#define BZP_MAX_FUN(a, b) (((a) > (b)) ? (a) : (b))

#define BZP_MIN_FUN(a, b) (((a) < (b)) ? (a) : (b))

#define BZP_BLOCK_FULL(bwt) (bwt->nBlock >= bwt->nBlockMax)

#define BZP_BUFF_READ_EMPTY(bzpf) (bzpf->input->pos >= bzpf->input->nBuf)


#define BZP_UPDATE_CRC(crcVar, cha)                                                                                    \
    {                                                                                                                  \
        (crcVar) = ((crcVar) << 8) ^ g_bzpCRC32Table[((crcVar) >> 24) ^ ((uint8_t)(cha))];                             \
    }

BzpStream *BzpStreamInit()
{
    BzpStream *stream = (BzpStream *)malloc(sizeof(BzpStream));
    if (stream == NULL)
    {
        return NULL;
    }
    stream->filePtr = NULL;
    stream->pos = 0;
    stream->nBuf = 0;
    return stream;
}

void BzpStreamFinish(BzpStream *stream)
{
    if (stream != NULL)
    {
        free(stream);
        stream = NULL;
    }
}

InDeComdata *BzpInDeComdataInit()
{
    InDeComdata *inData = (InDeComdata *)malloc(sizeof(InDeComdata));
    if (inData == NULL)
    {
        return NULL;
    }
    inData->input = NULL;
    inData->output = NULL;
    inData->num = 0;
    inData->lasChar = BZP_ASCII_SIZE;
    inData->nBuf = 0;
    inData->buf = 0;
    inData->num = 0;
    inData->blockCRC = BZP_INIT_BLOCK_CRC;
    return inData;
}

void BzpInDeComdataFinish(InDeComdata *inData)
{
    if (inData != NULL)
    {
        free(inData);
        inData = NULL;
    }
}

uint32_t BzpReadBits(int32_t nBit, InDeComdata *inData)
{
    uint32_t res = 0;
    while (inData->nBuf < nBit)
    {
        if (inData->input->nBuf == inData->input->pos)
        {
            inData->input->nBuf =
                fread(inData->input->buf, sizeof(char), sizeof(inData->input->buf), inData->input->filePtr);
            inData->input->pos = 0;
        }
        int32_t data = ((uint32_t)(inData->input->buf[inData->input->pos]));
        inData->buf = (inData->buf << BZP_BITS8) | data;
        inData->input->pos++;
        inData->nBuf += BZP_BITS8;
    }
    res = inData->buf >> (inData->nBuf - nBit);
    res = res & ((1 << nBit) - 1);
    inData->nBuf -= nBit;
    return res;
}

int32_t BzpWriteChar(uint8_t ch, InDeComdata *inData)
{
    int32_t ret = BZP_OK;
    if (inData->output->nBuf >= BZP_BUF_SIZE)
    {
        int32_t n2 =
            fwrite((void *)(inData->output->buf), sizeof(uint8_t), inData->output->nBuf, inData->output->filePtr);
        if (n2 != inData->output->nBuf)
        {
            ret = BZP_ERROR_IO;
        }
        inData->output->nBuf = 0;
    }
    inData->output->buf[inData->output->nBuf++] = ch;
    return ret;
}

int32_t BzpHuffmanDecodeStep(BzpHuffmanDecode *huffman, InDeComdata *inData)
{
    if (huffman->deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP)
    {
        huffman->deCodeNum = 0;
        huffman->selectCnt++;
    }
    int32_t gid = huffman->select[huffman->selectCnt];
    int32_t chlen = huffman->minLens[gid];
    int32_t val = BzpReadBits(chlen, inData);
    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT && val > huffman->limit[gid][chlen])
    {
        chlen++;
        int32_t nxtbit = BzpReadBits(1, inData);
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT)
    {
        return -1;
    }
    val = val - huffman->base[gid][chlen];
    val = huffman->perm[gid][val];
    huffman->deCodeNum++;
    return val;
}

int32_t BzpCheckFileHead(InDeComdata *inData)
{
    uint8_t ch;
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_1)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_2)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_3)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_4)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_BLOCK_HEAD_5)
    {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}

uint32_t BzpReadUInt24(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}

uint32_t BzpReadUInt32(InDeComdata *inData)
{
    uint8_t ch;
    uint32_t val = 0;
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    ch = BzpReadBits(BZP_BITS8, inData);
    val = (val << BZP_BITS8) | ((uint32_t)ch);
    return val;
}

int32_t BzpDeHuffmanSelect(InDeComdata *inData, BzpHuffmanDecode *huffman)
{
    uint8_t ch;
    int32_t selectmtf[BZP_HUFFMAN_MAX_SIZE_SELECT];
    for (int32_t i = 0; i < huffman->nSelect; i++)
    {
        int32_t j = -1;
        do
        {
            ch = BzpReadBits(BZP_BIT, inData);
            j++;
        } while (ch != 0);
        if (j >= huffman->nGroups)
        {
            return BZP_ERROR_DATA;
        }
        selectmtf[i] = j;
    }
    int32_t listSelect[BZP_MAX_GROUPS_NUM];
    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++)
    {
        listSelect[i] = i;
    }
    for (int32_t i = 0; i < huffman->nSelect; i++)
    {
        int32_t pos = selectmtf[i];
        int32_t tmpv = listSelect[pos];
        for (int32_t j = pos; j > 0; j--)
        {
            listSelect[j] = listSelect[j - 1];
        }
        listSelect[0] = tmpv;
        huffman->select[i] = tmpv;
    }
    return BZP_OK;
}

int32_t BzpDeHuffmanLen(InDeComdata *inData, BzpHuffmanDecode *huffman)
{
    uint8_t ch;
    for (int32_t i = 0; i < huffman->nGroups; i++)
    {
        int32_t val = BzpReadBits(BZP_BITS5, inData);
        for (int32_t j = 0; j < huffman->alphaSize; j++)
        {
            ch = BzpReadBits(BZP_BIT, inData);
            while (ch != 0)
            {
                ch = BzpReadBits(BZP_BIT, inData);
                val += (ch == 0 ? 1 : -1);
                ch = BzpReadBits(BZP_BIT, inData);
            }
            if (val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT)
            {
                return BZP_ERROR_DATA;
            }
            huffman->len[i][j] = val;
        }
    }
    return BZP_OK;
}

int32_t BzpMTFDeCode(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt)
{
    debwt->nBlock = 0;
    uint8_t ch;
    int32_t ninUse = huffman->alphaSize - BZP_EXTRA_CHARS_NUM;
    int32_t eob = ninUse + 1;
    int32_t val = BzpHuffmanDecodeStep(huffman, inData);
    while (val != eob && val != -1)
    {
        if (val == 0 || val == 1)
        {
            int32_t res = 0, basenum = 1;
            while (val == 0 || val == 1)
            {
                res = res + (val + 1) * basenum;
                basenum <<= 1;
                val = BzpHuffmanDecodeStep(huffman, inData);
            }
            for (int32_t j = 0; j < res; j++)
            {
                debwt->block[debwt->nBlock++] = inData->list[0];
            }
        }
        else
        {
            int32_t pos = val - 1;
            ch = inData->list[pos];
            debwt->block[debwt->nBlock++] = ch;
            for (int32_t j = pos; j > 0; j--)
            {
                inData->list[j] = inData->list[j - 1];
            }
            inData->list[0] = ch;
            val = BzpHuffmanDecodeStep(huffman, inData);
        }
    }
    if (val == -1)
    {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}

int32_t BzpDeCodeToStream(InDeComdata *inData, BzpBwtDecodeInfo *debwt)
{
    uint8_t ch;
    int32_t ret = BZP_OK;
    for (int32_t i = 0; i < debwt->nBlock; i++)
    {
        ch = debwt->deCode[i];
        if (inData->num == BZP_RLC_NUM_4)
        {
            for (int32_t j = 0; j < ((int32_t)ch); j++)
            {
                BZP_UPDATE_CRC(inData->blockCRC, (uint8_t)inData->lasChar);
                ret |= BzpWriteChar(inData->lasChar, inData);
            }
            inData->lasChar = BZP_ASCII_SIZE;
            inData->num = 0;
        }
        else if (ch == inData->lasChar)
        {
            BZP_UPDATE_CRC(inData->blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData->num++;
        }
        else
        {
            BZP_UPDATE_CRC(inData->blockCRC, ch);
            ret = BzpWriteChar(ch, inData);
            inData->lasChar = ch;
            inData->num = 1;
        }
        if (ret != BZP_OK)
            break;
    }
    return ret;
}

int32_t BzpGetDictionaryList(InDeComdata *inData)
{
    int32_t ninUse = 0;
    bool use16[16] = {0};
    bool inUse[BZP_ASCII_SIZE] = {0};
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        use16[i] = BzpReadBits(BZP_BIT, inData);
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        if (use16[i])
        {
            for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++)
            {
                inUse[i * BZP_GROUPS_ASCII + j] = BzpReadBits(BZP_BIT, inData);
            }
        }
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        if (inUse[i])
        {
            inData->list[ninUse++] = i;
        }
    }
    return ninUse;
}

int32_t BzpDeCompressOneBlock(InDeComdata *inData, BzpHuffmanDecode *huffman, BzpBwtDecodeInfo *debwt)
{
    int32_t ret = BZP_OK;
    BzpCheckFileHead(inData);
    uint32_t blockCRC = BzpReadUInt32(inData);
    (void)BzpReadBits(BZP_BIT, inData);
    int32_t oriPtr = BzpReadUInt24(inData);
    if (oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE * inData->blockSize)
    {
        return BZP_ERROR_DATA;
    }
    int32_t ninUse = BzpGetDictionaryList(inData);
    huffman->alphaSize = ninUse + BZP_EXTRA_CHARS_NUM;
    huffman->nGroups = BzpReadBits(BZP_BITS3, inData);
    if (huffman->nGroups < BZP_NGROUPS_NUM_0 || huffman->nGroups > BZP_NGROUPS_NUM_4)
    {
        return BZP_ERROR_DATA;
    }
    huffman->nSelect = BzpReadBits(BZP_BITS15, inData);
    int32_t nSelectUpperLimit = (inData->blockSize * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP + 1);
    if (huffman->nSelect < 1 || huffman->nSelect > nSelectUpperLimit)
    {
        return BZP_ERROR_DATA;
    }
    ret |= BzpDeHuffmanSelect(inData, huffman);
    ret |= BzpDeHuffmanLen(inData, huffman);
    if (ret != BZP_OK)
    {
        return ret;
    }
    BzpGenerateDecodeTable(huffman);
    debwt->oriPtr = oriPtr;
    ret = BzpMTFDeCode(inData, huffman, debwt);
    if (ret != BZP_OK || debwt->nBlock >= BZP_BASE_BLOCK_SIZE * inData->blockSize)
    {
        return BZP_ERROR_DATA;
    }
    BzpBwtDecode(debwt);
    ret = BzpDeCodeToStream(inData, debwt);
    if (ret != BZP_OK)
    {
        return ret;
    }
    inData->blockCRC = ~(inData->blockCRC);
    if (blockCRC != inData->blockCRC)
    {
        ret = BZP_ERROR_DATA;
    }
    return ret;
}

int32_t BZPReadFileEnd(InDeComdata *inData, uint32_t caltotalCRC)
{
    uint8_t ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_1)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_2)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_3)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_4)
    {
        return BZP_ERROR_DATA;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_FILE_END_5)
    {
        return BZP_ERROR_DATA;
    }
    uint32_t storedcombinedcrc = BzpReadUInt32(inData);
    if (caltotalCRC != storedcombinedcrc)
    {
        return BZP_ERROR_DATA;
    }
    return BZP_OK;
}

int32_t BzpReadFileHead(InDeComdata *inData)
{
    uint8_t ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_B)
    {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_Z)
    {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    if (ch != BZP_HDR_H)
    {
        return BZP_ERROR_DATA_MAGIC;
    }
    ch = BzpReadBits(BZP_BITS8, inData);
    int32_t blockSize = ch - BZP_HDR_0;
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return BZP_ERROR_DATA_MAGIC;
    }
    inData->blockSize = blockSize;
    return BZP_OK;
}

int32_t BZPDeCompressData(InDeComdata *inData)
{
    int32_t ret = BZP_OK;
    uint32_t caltotalCRC = 0;
    uint8_t ch;
    ret = BzpReadFileHead(inData);
    if (ret != BZP_OK)
    {
        return ret;
    }
    BzpHuffmanDecode *huffman = BzpHuffmanDecodeInit(inData->blockSize);
    BzpBwtDecodeInfo *debwt = BzpBwtDecodeInit(inData->blockSize);
    while ((ch = BzpReadBits(BZP_BITS8, inData)) != BZP_FILE_END_0)
    {
        if (ch != BZP_BLOCK_HEAD_0)
        {
            ret = BZP_ERROR_DATA;
            break;
        }
        BzpHuffmanDecodeReset(huffman);
        inData->blockCRC = BZP_INIT_BLOCK_CRC;
        ret = BzpDeCompressOneBlock(inData, huffman, debwt);
        if (ret != BZP_OK)
        {
            break;
        }
        caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL);
        caltotalCRC ^= inData->blockCRC;
    }
    if (ret == BZP_OK)
    {
        ret = BZPReadFileEnd(inData, caltotalCRC);
    }
    BzpHuffmanDecodeFinish(huffman);
    BzpBwtDecodeFinish(debwt);
    return ret;
}

void BzpDeComStreamFinish(InDeComdata *inData, BzpStream *inStream, BzpStream *outStream)
{
    if (inStream->filePtr != NULL)
    {
        fclose(inStream->filePtr);
        inStream->filePtr = NULL;
    }
    if (outStream->filePtr != NULL)
    {
        fclose(outStream->filePtr);
        outStream->filePtr = NULL;
    }
    BzpStreamFinish(inStream);
    BzpStreamFinish(outStream);
    BzpInDeComdataFinish(inData);
}

int32_t BzpDeCompressStream(char *inName, char *outName)
{
    int32_t ret = BZP_OK;
    if (inName == NULL || outName == NULL)
    {
        return BZP_ERROR_PARAM;
    }
    BzpStream *inStream = BzpStreamInit();
    BzpStream *outStream = BzpStreamInit();
    if (inStream == NULL || outStream == NULL)
    {
        BzpStreamFinish(inStream);
        BzpStreamFinish(outStream);
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    inStream->filePtr = fopen(inName, "rb");
    outStream->filePtr = fopen(outName, "wb");
    if ((inStream->filePtr == NULL || outStream->filePtr == NULL))
    {
        free(inStream);
        inStream = NULL;
        free(outStream);
        outStream = NULL;
        remove(outName);
        return BZP_ERROR_IO;
    }
    InDeComdata *inData = BzpInDeComdataInit();
    if (inData == NULL)
    {
        BzpDeComStreamFinish(inData, inStream, outStream);
        remove(outName);
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    inData->input = inStream;
    inData->output = outStream;
    ret = BZPDeCompressData(inData);
    if (inData->output->nBuf > 0)
    {
        int32_t n2 =
            fwrite((void *)(inData->output->buf), sizeof(uint8_t), inData->output->nBuf, inData->output->filePtr);
        if (n2 != inData->output->nBuf)
        {
            ret = BZP_ERROR_IO;
        }
        inData->output->nBuf = 0;
    }
    BzpDeComStreamFinish(inData, inStream, outStream);
    if (ret != BZP_OK)
    {
        remove(outName);
    }
    return ret;
}

BzpAlgorithmInfo *BzpAlgorithmInfoInit(int32_t blockSize)
{
    BzpAlgorithmInfo *bzpInfo = (BzpAlgorithmInfo *)malloc(sizeof(BzpAlgorithmInfo));
    if (bzpInfo == NULL)
    {
        return NULL;
    }
    bzpInfo->bwt = BzpBlockSortInit(blockSize);
    bzpInfo->mtf = BzpMtfInit(blockSize);
    bzpInfo->huffman = BzpHuffmanGroupsInit(blockSize);
    bzpInfo->outData = BzpOutComDataInit(blockSize);
    bzpInfo->compressFile = BzpFileInit();
    if (bzpInfo->bwt == NULL || bzpInfo->outData == NULL || bzpInfo->compressFile == NULL || bzpInfo->mtf == NULL ||
        bzpInfo->huffman == NULL)
    {
        BzpAlgorithmInfoFinish(bzpInfo);
        return NULL;
    }
    return bzpInfo;
}

int32_t BzpOpenFile(BzpAlgorithmInfo *bzpInfo, char *inName, char *outName)
{
    if (bzpInfo == NULL)
    {
        return BZP_ERROR_PARAM;
    }
    bzpInfo->compressFile->input->filePtr = fopen(inName, "rb");
    bzpInfo->compressFile->output->filePtr = fopen(outName, "wb");
    if ((bzpInfo->compressFile->input->filePtr == NULL || bzpInfo->compressFile->output->filePtr == NULL))
    {
        BzpAlgorithmInfoFinish(bzpInfo);
        remove(outName);
        return BZP_ERROR_IO;
    }
    return BZP_OK;
}

void BzpAlgorithmInfoFinish(BzpAlgorithmInfo *bzpInfo)
{
    if (bzpInfo != NULL)
    {
        BzpBwtFinish(bzpInfo->bwt);
        BzpMtfFinish(bzpInfo->mtf);
        BzpHuffmanGroupsFinish(bzpInfo->huffman);
        BzpFileFinish(bzpInfo->compressFile);
        BzpOutComDataFinish(bzpInfo->outData);
        free(bzpInfo);
    }
}

BzpFile *BzpFileInit()
{
    BzpFile *compressFile = (BzpFile *)malloc(sizeof(BzpFile));
    BzpStream *inStream = BzpStreamInit();
    BzpStream *outStream = BzpStreamInit();
    if (compressFile == NULL || inStream == NULL || outStream == NULL)
    {
        BzpStreamFinish(inStream);
        BzpStreamFinish(outStream);
        BzpFileFinish(compressFile);
        return NULL;
    }
    compressFile->input = inStream;
    compressFile->output = outStream;
    compressFile->input->pos = 0;
    compressFile->output->pos = 0;
    compressFile->num = 0;
    compressFile->lasChar = BZP_ASCII_SIZE;
    compressFile->state = BZP_INPUT_COMPRESS;
    return compressFile;
}

void BzpFileFinish(BzpFile *bzpF)
{
    if (bzpF != NULL)
    {
        BzpStreamFinish(bzpF->input);
        BzpStreamFinish(bzpF->output);
        free(bzpF);
        bzpF = NULL;
    }
}

BzpOutComdata *BzpOutComDataInit(int32_t blockSize)
{
    BzpOutComdata *outData = (BzpOutComdata *)malloc(sizeof(BzpOutComdata));
    if (outData == NULL)
    {
        return NULL;
    }
    outData->out = NULL;
    outData->out = (uint8_t *)malloc(blockSize * BZP_BASE_BLOCK_SIZE * sizeof(uint32_t));
    if (outData->out == NULL)
    {
        free(outData);
        return NULL;
    }
    outData->nBuf = 0;
    outData->buf = 0;
    outData->num = 0;
    outData->blockSize = blockSize;
    return outData;
}

void BzpOutComDataFinish(BzpOutComdata *data)
{
    if (data != NULL)
    {
        if (data->out != NULL)
        {
            free(data->out);
            data->out = NULL;
        }
        free(data);
        data = NULL;
    }
}

void BzpWriteToArray(int32_t val, int32_t n, BzpOutComdata *data)
{
    while (data->nBuf >= BZP_BITS8)
    {
        data->out[data->num++] = (uint8_t)(data->buf >> BZP_BITS24);
        data->nBuf -= BZP_BITS8;
        data->buf <<= BZP_BITS8;
    }
    data->buf |= (val << (BZP_BITS32 - n - data->nBuf));
    data->nBuf += n;
}

void BzpWriteInt32(int32_t val, BzpOutComdata *data)
{
    BzpWriteToArray((val >> BZP_BITS24) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray((val >> BZP_BITS16) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray((val >> BZP_BITS8) & 0xffL, BZP_BITS8, data);
    BzpWriteToArray(val & 0xffL, BZP_BITS8, data);
}

bool BzpFileEOF(FILE *f)
{
    int32_t c = fgetc(f);
    if (c == BZP_EOF)
        return true;
    (void)ungetc(c, f);
    return false;
}

void BzpWriteFileHead(BzpOutComdata *outData, int32_t blockId)
{
    if (blockId == 0)
    {
        BzpWriteToArray(BZP_HDR_B, BZP_BITS8, outData);
        BzpWriteToArray(BZP_HDR_Z, BZP_BITS8, outData);
        BzpWriteToArray(BZP_HDR_H, BZP_BITS8, outData);
        BzpWriteToArray((BZP_HDR_0 + outData->blockSize), BZP_BITS8, outData);
    }
}

void BzpCalculateCRC(BzpBwtInfo *bwt)
{
    bwt->blockCRC = ~(bwt->blockCRC);
    bwt->combinedCRC = (bwt->combinedCRC << 1) | (bwt->combinedCRC >> BZP_CRC_MOVE_RIGHT_VAL);
    bwt->combinedCRC ^= bwt->blockCRC;
}

void BzpWriteBlockHead(BzpOutComdata *outData, BzpBwtInfo *bwt)
{
    BzpWriteToArray(BZP_BLOCK_HEAD_0, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_1, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_2, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_3, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_4, BZP_BITS8, outData);
    BzpWriteToArray(BZP_BLOCK_HEAD_5, BZP_BITS8, outData);
    BzpWriteInt32(bwt->blockCRC, outData);
    BzpWriteToArray(0, BZP_BIT, outData);
    BzpWriteToArray(bwt->oriPtr, BZP_BITS24, outData);
}

void BzpWriteValidASCII(BzpOutComdata *outData, BzpBwtInfo *bwt)
{
    int32_t validGid[BZP_ASCII_SIZE], cnt = 0;
    bool use16[BZP_ASCII_SIZE];
    (void)memset_s(use16, sizeof(use16), 0, sizeof(use16));
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        int32_t gid = i / BZP_CHARS_PER_GROUP_ASCII;
        use16[gid] |= bwt->inUse[i];
    }
    for (int32_t i = 0; i < BZP_GROUPS_ASCII; i++)
    {
        BzpWriteToArray((int32_t)(use16[i]), BZP_BIT, outData);
        if (use16[i])
        {
            validGid[cnt++] = i;
        }
    }
    for (int32_t i = 0; i < cnt; i++)
    {
        for (int32_t j = 0; j < BZP_CHARS_PER_GROUP_ASCII; j++)
        {
            int32_t valid = validGid[i] * BZP_CHARS_PER_GROUP_ASCII + j;
            BzpWriteToArray((int32_t)(bwt->inUse[valid]), BZP_BIT, outData);
        }
    }
}

void BzpWriteSelect(BzpOutComdata *outData, BzpHuffmanGroups *huffman)
{
    BzpWriteToArray(huffman->nSelect, BZP_BITS15, outData);
    for (int32_t i = 0; i < huffman->nSelect; i++)
    {
        for (int32_t j = 0; j < huffman->selectMTF[i]; j++)
        {
            BzpWriteToArray(1, BZP_BIT, outData);
        }
        BzpWriteToArray(0, BZP_BIT, outData);
    }
}

void BzpWriteLen(BzpOutComdata *outData, BzpHuffmanGroups *huffman)
{
    for (int32_t i = 0; i < huffman->nGroups; i++)
    {
        int32_t val = huffman->huffmanGroups[i].len[0];
        BzpWriteToArray(val, BZP_BITS5, outData);
        for (int32_t j = 0; j < huffman->alphaSize; j++)
        {
            int32_t tar = huffman->huffmanGroups[i].len[j];
            int32_t deta = 0, saveVal = 0;
            if (val < tar)
            {
                saveVal = BZP_HUFFMAN_LEN_INCREASE;
                deta = 1;
            }
            else if (val > tar)
            {
                saveVal = BZP_HUFFMAN_LEN_REDUCED;
                deta = -1;
            }
            while (val != tar)
            {
                BzpWriteToArray(saveVal, BZP_BITS2, outData);
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT, outData);
        }
    }
}

void BzpWriteInputEncode(BzpOutComdata *outData, BzpMtfInfo *mtf, BzpHuffmanGroups *huffman)
{
    for (int32_t i = 0; i < mtf->nMtf; i++)
    {
        int32_t val = mtf->mtfV[i];
        int32_t gid = huffman->select[i / BZP_ELEMS_NUM_IN_ONE_GROUP];
        int32_t code = huffman->huffmanGroups[gid].table[val];
        int32_t len = huffman->huffmanGroups[gid].len[val];
        BzpWriteToArray(code, len, outData);
    }
}

void BzpWriteFileEnd(BzpOutComdata *outData, int32_t combinedCRC)
{
    BzpWriteToArray(BZP_FILE_END_0, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_1, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_2, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_3, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_4, BZP_BITS8, outData);
    BzpWriteToArray(BZP_FILE_END_5, BZP_BITS8, outData);
    BzpWriteInt32(combinedCRC, outData);
}

void BzpFlushbuf(BzpOutComdata *outData)
{
    while (outData->nBuf > 0)
    {
        outData->out[outData->num++] = (uint8_t)(outData->buf >> BZP_BITS24);
        outData->nBuf -= BZP_BITS8;
        outData->buf <<= BZP_BITS8;
    }
}

int32_t BzpCompressOneBlock(BzpAlgorithmInfo *bzpInfo, BzpOutComdata *outData)
{
    BzpBwtInfo *bwt = bzpInfo->bwt;
    BzpMtfInfo *mtf = bzpInfo->mtf;
    BzpHuffmanGroups *huffman = bzpInfo->huffman;
    int ret = BZP_OK;
    if (bwt->nBlock == 0)
    {
        return BZP_OK;
    }
    BzpWriteFileHead(outData, bwt->blockId);
    if (bwt->nBlock > 0)
    {
        BzpCalculateCRC(bwt);
        BzpBlockSortMain(bwt);
        BzpMtfReSet(mtf);
        mtf->block = bwt->block;
        mtf->map = bwt->sortBlock;
        mtf->inUse = bwt->inUse;
        mtf->nBlock = bwt->nBlock;
        BzpMtfMain(mtf);
        ret = BzpHuffmanGroupsReset(huffman, mtf->nUse + BZP_EXTRA_CHARS_NUM);
        if (ret != BZP_OK)
        {
            return ret;
        }
        huffman->block = mtf->mtfV;
        huffman->mtfFreq = mtf->mtfFreq;
        huffman->nBlock = mtf->nMtf;
        BzpHuffmanMain(huffman);
        BzpWriteBlockHead(outData, bwt);
        BzpWriteValidASCII(outData, bwt);
        BzpWriteToArray(huffman->nGroups, BZP_BITS3, outData);
        BzpWriteSelect(outData, huffman);
        BzpWriteLen(outData, huffman);
        BzpWriteInputEncode(outData, mtf, huffman);
    }
    return BZP_OK;
}

int32_t BzpBuffToStream(BzpFile *bzpf, BzpOutComdata *outData)
{
    bzpf->output->pos = 0;
    int32_t pos = 0;
    while (pos < outData->num)
    {
        bzpf->output->nBuf = 0;
        while (pos < outData->num && bzpf->output->nBuf < BZP_BUF_SIZE)
        {
            bzpf->output->buf[bzpf->output->nBuf++] = outData->out[pos];
            pos++;
        }
        int32_t n2 = fwrite((void *)(bzpf->output->buf), sizeof(uint8_t), bzpf->output->nBuf, bzpf->output->filePtr);
        if (n2 != bzpf->output->nBuf)
        {
            return BZP_ERROR_IO;
        }
    }
    return BZP_OK;
}

void BzpAddCharToBlock(uint8_t lasch, int32_t num, BzpBwtInfo *bwt)
{
    if (num < BZP_RLC_NUM_LOWER_LIMIT || num > BZP_RLC_NUM_UPPER_LIMIT)
    {
        return;
    }
    for (int32_t i = 0; i < num; i++)
    {
        BZP_UPDATE_CRC(bwt->blockCRC, lasch);
    }
    int32_t val = BZP_MIN_FUN(num, (int32_t)BZP_RLC_NUM_4);
    switch (val)
    {
    case BZP_RLC_NUM_4:
        bwt->block[bwt->nBlock++] = lasch;
    case BZP_RLC_NUM_3:
        bwt->block[bwt->nBlock++] = lasch;
    case BZP_RLC_NUM_2:
        bwt->block[bwt->nBlock++] = lasch;
    case BZP_RLC_NUM_1:
        bwt->block[bwt->nBlock++] = lasch;
    default:
        break;
    }
    if (num >= BZP_RLC_NUM_4)
    {
        bwt->block[bwt->nBlock++] = ((char)(num - BZP_RLC_NUM_4));
        bwt->inUse[num - BZP_RLC_NUM_4] = true;
    }
    bwt->inUse[lasch] = true;
}

void BzpBuffToBlockRLC(BzpFile *bzpf, BzpBwtInfo *bwt, bool IsLastdata)
{
    while (!BZP_BLOCK_FULL(bwt) && !BZP_BUFF_READ_EMPTY(bzpf))
    {
        int32_t pos = bzpf->input->pos;
        uint8_t ch = (uint8_t)bzpf->input->buf[pos];
        uint8_t lasch = (uint8_t)bzpf->lasChar;
        if (ch != lasch || bzpf->num == BZP_RLC_NUM_UPPER_LIMIT)
        {
            BzpAddCharToBlock(lasch, bzpf->num, bwt);
            bzpf->lasChar = ch;
            bzpf->num = 1;
        }
        else
        {
            bzpf->num++;
        }
        bzpf->input->pos++;
    }
    if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf))
    {
        BzpAddCharToBlock(bzpf->lasChar, bzpf->num, bwt);
        bzpf->lasChar = BZP_ASCII_SIZE;
        bzpf->num = 0;
    }
}

void BzpResetCompress(BzpBwtInfo *bwt, BzpOutComdata *outData)
{
    outData->num = 0;
    bwt->nBlock = 0;
    bwt->blockCRC = BZP_INIT_BLOCK_CRC;
    (void)memset_s(bwt->inUse, sizeof(bwt->inUse), 0, sizeof(bwt->inUse));
    int32_t n = outData->blockSize * BZP_BASE_BLOCK_SIZE * sizeof(int32_t);
    (void)memset_s(bwt->isStartPos, n, 0, n);
    bwt->blockId++;
}

int32_t BzpProcessData(BzpAlgorithmInfo *bzpInfo, bool IsLastdata)
{
    BzpFile *bzpf = bzpInfo->compressFile;
    BzpOutComdata *outData = bzpInfo->outData;
    BzpBwtInfo *bwt = bzpInfo->bwt;
    bzpf->state = BZP_INPUT_COMPRESS;
    int32_t ret = BZP_OK;
    while (bzpf->state != BZP_RETUEN_COMPRESS)
    {
        if (bzpf->state == BZP_OUTPUT_COMPRESS)
        {
            ret = BzpBuffToStream(bzpf, outData);
            BzpResetCompress(bwt, outData);
            bzpf->state = BZP_INPUT_COMPRESS;
            if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf))
            {
                bzpf->state = BZP_RETUEN_COMPRESS;
            }
        }
        if (bzpf->state == BZP_INPUT_COMPRESS)
        {
            BzpBuffToBlockRLC(bzpf, bwt, IsLastdata);
            if (IsLastdata && BZP_BUFF_READ_EMPTY(bzpf))
            {
                ret = BzpCompressOneBlock(bzpInfo, outData);
                BzpWriteFileEnd(outData, bwt->combinedCRC);
                BzpFlushbuf(outData);
                bzpf->state = BZP_OUTPUT_COMPRESS;
            }
            else if (BZP_BLOCK_FULL(bwt))
            {
                ret = BzpCompressOneBlock(bzpInfo, outData);
                bzpf->state = BZP_OUTPUT_COMPRESS;
            }
            else
            {
                bzpf->state = BZP_RETUEN_COMPRESS;
            }
        }
        if (ret != BZP_OK)
        {
            return ret;
        }
    }
    return ret;
}

void BzpCompressEnd(BzpAlgorithmInfo *bzpInfo)
{
    if (bzpInfo->compressFile->input->filePtr != NULL)
    {
        fclose(bzpInfo->compressFile->input->filePtr);
    }
    if (bzpInfo->compressFile->output->filePtr != NULL)
    {
        fclose(bzpInfo->compressFile->output->filePtr);
    }
    BzpAlgorithmInfoFinish(bzpInfo);
}

int32_t BzpCompressStream(char *inName, char *outName, int32_t blockSize)
{
    int32_t ret = BZP_OK;
    bool IsLastdata = false;
    if (inName == NULL || outName == NULL || BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return BZP_ERROR_PARAM;
    }
    BzpAlgorithmInfo *bzpInfo = BzpAlgorithmInfoInit(blockSize);
    if (bzpInfo == NULL)
    {
        return BZP_ERROR_MEMORY_OPER_FAILURE;
    }
    ret = BzpOpenFile(bzpInfo, inName, outName);
    if (ret != BZP_OK)
    {
        return ret;
    }
    BzpStream *inStream = bzpInfo->compressFile->input;
    while (!IsLastdata)
    {
        inStream->nBuf = fread(inStream->buf, sizeof(char), sizeof(inStream->buf), inStream->filePtr);
        inStream->pos = 0;
        IsLastdata = BzpFileEOF(inStream->filePtr);
        ret = BzpProcessData(bzpInfo, IsLastdata);
        if (ret != BZP_OK)
        {
            break;
        }
    }
    BzpCompressEnd(bzpInfo);
    if (ret != BZP_OK)
    {
        remove(outName);
    }
    return ret;
}

BzpBwtInfo *BzpBlockSortInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpBwtInfo *bwt = (BzpBwtInfo *)malloc(sizeof(BzpBwtInfo));
    if (bwt == NULL)
    {
        return NULL;
    }
    (void)memset_s(bwt, sizeof(BzpBwtInfo), 0, sizeof(BzpBwtInfo));
    int32_t spaceSize = blockSize * BZP_BASE_BLOCK_SIZE;
    bwt->nBlockMax = spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE;
    bwt->block = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->sortBlock = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    bwt->idx = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    bwt->isStartPos = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (bwt->block == NULL || bwt->sortBlock == NULL || bwt->idx == NULL || bwt->isStartPos == NULL)
    {
        BzpBwtFinish(bwt);
        return NULL;
    }
    (void)memset_s(bwt->isStartPos, spaceSize * sizeof(int32_t), 0, spaceSize * sizeof(int32_t));
    bwt->blockCRC = BZP_INIT_BLOCK_CRC;
    return bwt;
}

void BzpShellSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    int32_t increments[] = {BZP_SHELL_SORT_INCREMENT1, BZP_SHELL_SORT_INCREMENT0};
    int32_t i, j;
    if (l >= r)
    {
        return;
    }
    for (int32_t id = 0; id < BZP_SHELL_SORT_INCREMENT_NUMS; id++)
    {
        int32_t H = increments[id];
        if (r - l + 1 <= H)
        {
            continue;
        }
        for (i = l + H; i <= r; i++)
        {
            int32_t tmpIdx = sortBlock[i];
            int32_t tmpVal = idx[tmpIdx];
            for (j = i - H; j >= l && idx[sortBlock[j]] > tmpVal; j -= H)
            {
                sortBlock[j + H] = sortBlock[j];
            }
            sortBlock[j + H] = tmpIdx;
        }
    }
}

void BzpSwap2Elem(int32_t *sortBlock, int32_t lPos, int32_t rPos)
{
    int32_t value = sortBlock[lPos];
    sortBlock[lPos] = sortBlock[rPos];
    sortBlock[rPos] = value;
}

void BzpSwap3Elem(int32_t *sortBlock, int32_t lPos, int32_t ePos, int32_t rPos)
{
    int32_t value = sortBlock[lPos];
    sortBlock[lPos] = sortBlock[rPos];
    sortBlock[rPos] = sortBlock[ePos];
    sortBlock[ePos] = value;
}

int32_t BzpSelectMidVal(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    int32_t mid = (l + r) >> 1;
    int32_t vl = idx[sortBlock[l]];
    int32_t vmid = idx[sortBlock[mid]];
    int32_t vr = idx[sortBlock[r]];
    if (vl > vr)
    {
        int32_t tmp = l;
        l = r;
        r = tmp;
        vl = idx[sortBlock[l]];
        vr = idx[sortBlock[r]];
    }
    if (vmid <= vl)
    {
        return vl;
    }
    else if (vmid <= vr)
    {
        return vmid;
    }
    else
    {
        return vr;
    }
}

void BzpQSortSingle(int32_t *sortBlock, int32_t *idx, BzpQSortInfo *stack)
{
    int32_t tl = stack->tl, tr = stack->tr;
    int32_t value = BzpSelectMidVal(sortBlock, idx, tl, tr);
    int32_t lPos = tl, rPos = tr, ePos = tl;
    while (ePos <= rPos)
    {
        if (idx[sortBlock[ePos]] < value)
        {
            BzpSwap2Elem(sortBlock, ePos, lPos);
            ePos++;
            lPos++;
        }
        else if (idx[sortBlock[ePos]] == value)
        {
            ePos++;
        }
        else
        {
            while (rPos >= ePos && idx[sortBlock[rPos]] > value)
            {
                rPos--;
            }
            if (rPos < ePos)
            {
                break;
            }
            if (idx[sortBlock[rPos]] == value)
            {
                BzpSwap2Elem(sortBlock, ePos, rPos);
            }
            else if (lPos == ePos)
            {
                BzpSwap2Elem(sortBlock, ePos, rPos);
                lPos++;
            }
            else
            {
                BzpSwap3Elem(sortBlock, lPos, ePos, rPos);
                lPos++;
            }
            ePos++;
            rPos--;
        }
    }
    if (lPos - tl > tr - rPos)
    {
        stack->stackL[stack->cnt] = tl;
        stack->stackR[stack->cnt] = lPos - 1;
        stack->cnt++;
        stack->stackL[stack->cnt] = rPos + 1;
        stack->stackR[stack->cnt] = tr;
        stack->cnt++;
    }
    else
    {
        stack->stackL[stack->cnt] = rPos + 1;
        stack->stackR[stack->cnt] = tr;
        stack->cnt++;
        stack->stackL[stack->cnt] = tl;
        stack->stackR[stack->cnt] = lPos - 1;
        stack->cnt++;
    }
}

void BzpQuickSort(int32_t *sortBlock, int32_t *idx, int32_t l, int32_t r)
{
    BzpQSortInfo stack;
    stack.cnt = 0;
    stack.stackL[stack.cnt] = l;
    stack.stackR[stack.cnt] = r;
    stack.cnt++;
    while (stack.cnt > 0)
    {
        stack.cnt--;
        int32_t tl = stack.stackL[stack.cnt];
        int32_t tr = stack.stackR[stack.cnt];
        if (tl >= tr)
        {
            continue;
        }
        if (tr - tl < BZP_THRESHOLD_SHELL_SORT)
        {
            BzpShellSort(sortBlock, idx, tl, tr);
            continue;
        }
        stack.tl = tl;
        stack.tr = tr;
        BzpQSortSingle(sortBlock, idx, &stack);
    }
}

void BzpUpdateflag(BzpBwtInfo *bwt, int32_t l, int32_t r)
{
    int32_t tmpst = -1;
    for (int32_t i = l; i <= r; i++)
    {
        int32_t tmpnow = bwt->idx[bwt->sortBlock[i]];
        if (tmpst != tmpnow)
        {
            bwt->isStartPos[i] = 1;
            tmpst = tmpnow;
        }
    }
}

void BzpBinaryLiftingSort(BzpBwtInfo *bwt)
{
    int32_t ftab[BZP_ASCII_SIZE];
    (void)memset_s(ftab, sizeof(ftab), 0, sizeof(ftab));
    for (int32_t i = 0; i < bwt->nBlock; i++)
    {
        ftab[bwt->block[i]]++;
    }
    for (int32_t i = 1; i < BZP_ASCII_SIZE; i++)
    {
        ftab[i] += ftab[i - 1];
    }
    for (int32_t i = 0; i < bwt->nBlock; i++)
    {
        int32_t ch = bwt->block[i];
        ftab[ch]--;
        bwt->sortBlock[ftab[ch]] = i;
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        bwt->isStartPos[ftab[i]] = 1;
    }
    int32_t M = 1, sortflag = true;
    while (M < bwt->nBlock && sortflag == true)
    {
        int32_t st = 0;
        sortflag = false;
        for (int32_t i = 0; i < bwt->nBlock; i++)
        {
            if (bwt->isStartPos[i])
            {
                st = i;
            }
            int32_t pos = bwt->sortBlock[i] - M;
            if (pos < 0)
            {
                pos += bwt->nBlock;
            }
            bwt->idx[pos] = st;
        }
        int32_t l = 0, r = 1;
        while (l < bwt->nBlock)
        {
            while (r < bwt->nBlock && bwt->isStartPos[r] != 1)
            {
                r++;
            }
            r--;
            if (l < r)
            {
                sortflag = true;
                BzpQuickSort(bwt->sortBlock, bwt->idx, l, r);
                BzpUpdateflag(bwt, l, r);
            }
            l = r + 1;
            r = l + 1;
        }
        M <<= 1;
    }
}

void BzpBlockSortMain(BzpBwtInfo *bwt)
{
    BzpBinaryLiftingSort(bwt);
    for (int32_t i = 0; i < bwt->nBlock; i++)
    {
        if (bwt->sortBlock[i] == 0)
        {
            bwt->oriPtr = i;
            break;
        }
    }
}

void BzpBwtFinish(BzpBwtInfo *bwt)
{
    if (bwt != NULL)
    {
        if (bwt->block != NULL)
        {
            free(bwt->block);
            bwt->block = NULL;
        }
        if (bwt->sortBlock != NULL)
        {
            free(bwt->sortBlock);
            bwt->sortBlock = NULL;
        }
        if (bwt->idx != NULL)
        {
            free(bwt->idx);
            bwt->idx = NULL;
        }
        if (bwt->isStartPos != NULL)
        {
            free(bwt->isStartPos);
            bwt->isStartPos = NULL;
        }
        free(bwt);
        bwt = NULL;
    }
}

BzpMtfInfo *BzpMtfInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpMtfInfo *mtf = (BzpMtfInfo *)malloc(sizeof(BzpMtfInfo));
    if (mtf == NULL)
    {
        return NULL;
    }
    mtf->mtfV = NULL;
    mtf->mtfV = (int32_t *)malloc(blockSize * BZP_BASE_BLOCK_SIZE * sizeof(int32_t));
    if (mtf->mtfV == NULL)
    {
        free(mtf);
        mtf = NULL;
        return NULL;
    }
    mtf->nUse = 0;
    mtf->nMtf = 0;
    mtf->block = NULL;
    mtf->map = NULL;
    mtf->inUse = NULL;
    return mtf;
}

void BzpMtfReSet(BzpMtfInfo *mtf)
{
    mtf->nUse = 0;
    mtf->nMtf = 0;
    mtf->block = NULL;
    mtf->map = NULL;
    mtf->inUse = NULL;
}

void BzpMapInputChar(BzpMtfInfo *mtf, uint8_t *list, int32_t lenList)
{
    if (BZP_ASCII_SIZE > lenList)
    {
        return;
    }
    for (int32_t i = 0; i < BZP_ASCII_SIZE; i++)
    {
        if (mtf->inUse[i])
        {
            list[mtf->nUse] = (uint8_t)i;
            mtf->nUse++;
        }
    }
}

void BzpNumEncode(BzpMtfInfo *mtf, int32_t num)
{
    num <<= 1;
    do
    {
        num >>= 1;
        num--;
        if (num & 1)
        {
            mtf->mtfV[mtf->nMtf++] = BZP_MTF_ENCODE1;
            mtf->mtfFreq[BZP_MTF_ENCODE1]++;
        }
        else
        {
            mtf->mtfV[mtf->nMtf++] = BZP_MTF_ENCODE0;
            mtf->mtfFreq[BZP_MTF_ENCODE0]++;
        }
    } while (num >= BZP_MTF_ENCODE_BASE);
}

void BzpMtfMain(BzpMtfInfo *mtf)
{
    uint8_t list[BZP_MAX_ALPHA_SIZE];
    int32_t EOB;
    int32_t num = 0;
    BzpMapInputChar(mtf, list, BZP_MAX_ALPHA_SIZE);
    EOB = mtf->nUse + 1;
    for (int32_t i = 0; i <= EOB; i++)
    {
        mtf->mtfFreq[i] = 0;
    }
    for (int32_t i = 0; i < mtf->nBlock; i++)
    {
        int32_t pos = mtf->map[i] - 1;
        if (pos < 0)
        {
            pos += mtf->nBlock;
        }
        uint8_t ch = mtf->block[pos];
        if (ch == list[0])
        {
            num++;
        }
        else
        {
            if (num > 0)
            {
                BzpNumEncode(mtf, num);
                num = 0;
            }
            int32 _t pos_ = 1;
            while (ch != list[pos_] && pos_ < mtf->nUse)
            {
                pos_++;
            }
            for (int32_t j = pos_; j > 0; j--)
            {
                list[j] = list[j - 1];
            }
            list[0] = ch;
            mtf->mtfV[mtf->nMtf] = pos_ + 1;
            mtf->mtfFreq[pos_ + 1]++;
            mtf->nMtf++;
        }
    }
    if (num > 0)
    {
        BzpNumEncode(mtf, num);
    }
    mtf->mtfV[mtf->nMtf] = EOB;
    mtf->mtfFreq[EOB]++;
    mtf->nMtf++;
}

void BzpMtfFinish(BzpMtfInfo *mtf)
{
    if (mtf != NULL)
    {
        if (mtf->mtfV != NULL)
        {
            free(mtf->mtfV);
            mtf->mtfV = NULL;
        }
        free(mtf);
        mtf = NULL;
    }
}

void BzpHuffmanInit(int32_t alphaSize, BzpHuffmanInfo *huffman)
{
    (void)memset_s(huffman->len, sizeof(huffman->len), 0, sizeof(huffman->len));
    huffman->nHeap = 0;
    huffman->nWeight = 0;
    huffman->alphaSize = alphaSize;
}

void BzpHuffmanInitArray(BzpHuffmanInfo *huffman)
{
    int32_t i;
    huffman->nHeap = 0;
    huffman->nWeight = huffman->alphaSize;
    for (i = 0; i < huffman->alphaSize; i++)
    {
        huffman->parent[i] = -1;
    }
}

void BzpHeapAdjustUp(int32_t *heap, int32_t *weight, int32_t pos)
{
    int32_t tmpw = weight[heap[pos]];
    int32_t tmpv = heap[pos];
    while (pos > 1)
    {
        if (tmpw < weight[heap[pos >> 1]])
        {
            heap[pos] = heap[pos >> 1];
            pos >>= 1;
        }
        else
        {
            break;
        }
    }
    heap[pos] = tmpv;
}

void BzpHeapAdjustDown(int32_t *heap, int32_t *weight, int32_t nHeap)
{
    int32_t pos = 1;
    int32_t chpos = pos << 1;
    int32_t tmpid = heap[pos];
    int32_t tmpv = weight[tmpid];
    while (chpos <= nHeap)
    {
        if ((chpos | 1) <= nHeap && weight[heap[chpos]] > weight[heap[chpos | 1]])
        {
            chpos |= 1;
        }
        if (tmpv < weight[heap[chpos]])
        {
            break;
        }
        heap[pos] = heap[chpos];
        pos = chpos;
        chpos = pos << 1;
    }
    heap[pos] = tmpid;
}

void BzpHeapInit(BzpHuffmanInfo *huffman)
{
    int32_t i = 0;
    for (i = 0; i < huffman->alphaSize; i++)
    {
        huffman->nHeap++;
        huffman->heap[huffman->nHeap] = i;
        BzpHeapAdjustUp(huffman->heap, huffman->weight, huffman->nHeap);
    }
}

int32_t BzpHuffmanWeightAdd(int32_t w1, int32_t w2)
{
    return ((w1 & 0xffffff00) + (w2 & 0xffffff00)) | (BZP_MAX_FUN((w1 & 0x000000ff), (w2 & 0x000000ff)) + 1);
}

void BzpBuildHuffmanTree(BzpHuffmanInfo *huffman)
{
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    int32_t idx1, idx2;
    while (huffman->nHeap > 1)
    {
        idx1 = huffman->heap[1];
        huffman->heap[1] = huffman->heap[huffman->nHeap--];
        BzpHeapAdjustDown(huffman->heap, huffman->weight, huffman->nHeap);
        idx2 = huffman->heap[1];
        huffman->heap[1] = huffman->heap[huffman->nHeap--];
        BzpHeapAdjustDown(huffman->heap, huffman->weight, huffman->nHeap);
        huffman->weight[huffman->nWeight] = BzpHuffmanWeightAdd(huffman->weight[idx1], huffman->weight[idx2]);
        huffman->parent[idx1] = huffman->nWeight;
        huffman->parent[idx2] = huffman->nWeight;
        huffman->parent[huffman->nWeight] = -1;
        huffman->nHeap++;
        huffman->heap[huffman->nHeap] = huffman->nWeight;
        huffman->nWeight++;
        BzpHeapAdjustUp(huffman->heap, huffman->weight, huffman->nHeap);
    }
}

int32_t BzpGetCodeLen(BzpHuffmanInfo *huffman)
{
    int32_t maxlen = 0;
    BzpBuildHuffmanTree(huffman);
    int32_t i;
    maxlen = 0;
    for (i = 0; i < huffman->alphaSize; i++)
    {
        int32_t x = i;
        int32_t tlen = 0;
        while (huffman->parent[x] >= 0)
        {
            x = huffman->parent[x];
            tlen++;
        }
        huffman->len[i] = tlen;
        maxlen = BZP_MAX_FUN(maxlen, tlen);
    }
    return maxlen;
}

void BzpBuildTreeBalanceHeight(BzpHuffmanInfo *huffman)
{
    int32_t maxlen = 0;
    for (int32_t i = 0; i < huffman->alphaSize; i++)
    {
        if (huffman->weight[i] == 0)
        {
            huffman->weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        }
        else
        {
            huffman->weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        }
    }
    do
    {
        maxlen = BzpGetCodeLen(huffman);
        if (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE)
        {
            for (int32_t i = 0; i < huffman->alphaSize; i++)
            {
                int32_t w = (huffman->weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS);
                w = ((w >> 1) + 1);
                huffman->weight[i] = w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
            }
        }
    } while (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE);
}

void BzpGetHuffmanTable(BzpHuffmanInfo *huffman)
{
    int32_t vec = 0;
    int32_t mi = huffman->len[0], mx = huffman->len[0];
    for (int32_t i = 0; i < huffman->alphaSize; i++)
    {
        mi = BZP_MIN_FUN(mi, huffman->len[i]);
        mx = BZP_MAX_FUN(mx, huffman->len[i]);
    }
    for (int32_t i = mi; i <= mx; i++)
    {
        for (int32_t j = 0; j < huffman->alphaSize; j++)
        {
            if (huffman->len[j] == i)
            {
                huffman->table[j] = vec;
                vec++;
            }
        }
        vec <<= 1;
    }
}

int32_t BzpHuffmanGroupsReset(BzpHuffmanGroups *huffman, int32_t alphaSize)
{
    if (BZP_INVALID_ALPHA_SIZE(alphaSize))
    {
        return BZP_ERROR_PARAM;
    }
    huffman->alphaSize = alphaSize;
    huffman->block = NULL;
    huffman->mtfFreq = NULL;
    huffman->nSelect = 0;
    huffman->nGroups = 0;
    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++)
    {
        BzpHuffmanInit(alphaSize, &huffman->huffmanGroups[i]);
    }
    return BZP_OK;
}

BzpHuffmanGroups *BzpHuffmanGroupsInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpHuffmanGroups *huffmanGroups = (BzpHuffmanGroups *)malloc(sizeof(BzpHuffmanGroups));
    if (huffmanGroups == NULL)
    {
        return NULL;
    }
    huffmanGroups->select = NULL;
    huffmanGroups->selectMTF = NULL;
    int32_t spaceSize = blockSize * BZP_BASE_BLOCK_SIZE / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffmanGroups->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    huffmanGroups->selectMTF = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffmanGroups->select == NULL || huffmanGroups->selectMTF == NULL)
    {
        BzpHuffmanGroupsFinish(huffmanGroups);
        return NULL;
    }
    huffmanGroups->alphaSize = 0;
    huffmanGroups->block = NULL;
    huffmanGroups->mtfFreq = NULL;
    huffmanGroups->nSelect = 0;
    huffmanGroups->nGroups = 0;
    for (int32_t i = 0; i < BZP_MAX_GROUPS_NUM; i++)
    {
        BzpHuffmanInit(0, &huffmanGroups->huffmanGroups[i]);
    }
    return huffmanGroups;
}

void BzpHuffmanGroupsFinish(BzpHuffmanGroups *huffman)
{
    if (huffman != NULL)
    {
        if (huffman->select != NULL)
        {
            free(huffman->select);
            huffman->select = NULL;
        }
        if (huffman->selectMTF != NULL)
        {
            free(huffman->selectMTF);
            huffman->selectMTF = NULL;
        }
        free(huffman);
        huffman = NULL;
    }
}

int32_t BzpGetHuffmanGroups(int32_t nBlock)
{
    int32_t nGroups = 1;
    if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT0)
    {
        nGroups = BZP_NGROUPS_NUM_0;
    }
    else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT1)
    {
        nGroups = BZP_NGROUPS_NUM_1;
    }
    else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT2)
    {
        nGroups = BZP_NGROUPS_NUM_2;
    }
    else if (nBlock < BZP_NGROUPS_BLOCK_NUM_LIMIT3)
    {
        nGroups = BZP_NGROUPS_NUM_3;
    }
    else
    {
        nGroups = BZP_NGROUPS_NUM_4;
    }
    return nGroups;
}

void BzpGenerateSelectMTF(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = huffman->nGroups;
    int32_t list[nGroups];
    for (int32_t i = 0; i < nGroups; i++)
    {
        list[i] = i;
    }
    for (int32_t i = 0; i < huffman->nSelect; i++)
    {
        int32_t pos = 0;
        for (int32_t j = 0; j < nGroups; j++)
        {
            if (huffman->select[i] == list[j])
            {
                pos = j;
                break;
            }
        }
        for (int32_t j = pos; j > 0; j--)
        {
            list[j] = list[j - 1];
        }
        list[0] = huffman->select[i];
        huffman->selectMTF[i] = pos;
    }
}

void BzpInitLenArray(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = huffman->nGroups;
    int32_t npart = nGroups;
    int32_t AllFreqNum = huffman->nBlock;
    int32_t st = 0, ed;
    while (npart > 0)
    {
        int32_t NowFreqNum = 0;
        int32_t FreqNumLimit = AllFreqNum / npart;
        ed = st - 1;
        while (ed < huffman->alphaSize - 1 && NowFreqNum < FreqNumLimit)
        {
            ed++;
            NowFreqNum += huffman->mtfFreq[ed];
        }
        if (ed > st && npart != nGroups && npart != 1 && ((nGroups - npart) & 1))
        {
            NowFreqNum -= huffman->mtfFreq[ed];
            ed--;
        }
        for (int32_t i = 0; i < huffman->alphaSize; i++)
        {
            if (i >= st && i <= ed)
            {
                huffman->huffmanGroups[npart - 1].len[i] = 0;
            }
            else
            {
                huffman->huffmanGroups[npart - 1].len[i] = BZP_HUFFMAN_LEN_MAX_COST;
            }
        }
        npart--;
        st = ed + 1;
        AllFreqNum -= NowFreqNum;
    }
}

void BzpCalculateCost(BzpHuffmanGroups *huffman, int32_t st, int32_t ed)
{
    (void)memset_s(huffman->cost, sizeof(huffman->cost), 0, sizeof(huffman->cost));
    int32_t nGroups = huffman->nGroups;
    for (int32_t k = st; k <= ed; k++)
    {
        for (int32_t t = 0; t < nGroups; t++)
        {
            huffman->cost[t] += huffman->huffmanGroups[t].len[huffman->block[k]];
        }
    }
}

int32_t BzpSelectTree(BzpHuffmanGroups *huffman)
{
    int32_t id = 0;
    int32_t nGroups = huffman->nGroups;
    for (int32_t k = 0; k < nGroups; k++)
    {
        if (huffman->cost[k] < huffman->cost[id])
        {
            id = k;
        }
    }
    huffman->select[huffman->nSelect++] = id;
    return id;
}

void BzpHuffmanMain(BzpHuffmanGroups *huffman)
{
    int32_t nGroups = BzpGetHuffmanGroups(huffman->nBlock);
    huffman->nGroups = nGroups;
    BzpInitLenArray(huffman);
    int32_t st = 0, ed;
    for (int32_t i = 0; i < BZP_MAX_ITER_NUM; i++)
    {
        for (int32_t j = 0; j < nGroups; j++)
        {
            (void)memset_s(huffman->huffmanGroups[j].weight, sizeof(huffman->huffmanGroups[j].weight), 0,
                           sizeof(huffman->huffmanGroups[j].weight));
        }
        st = 0;
        huffman->nSelect = 0;
        while (st < huffman->nBlock)
        {
            ed = BZP_MIN_FUN(huffman->nBlock, st + (int32_t)BZP_ELEMS_NUM_IN_ONE_GROUP) - 1;
            BzpCalculateCost(huffman, st, ed);
            int32_t id = BzpSelectTree(huffman);
            for (int32_t k = st; k <= ed; k++)
            {
                huffman->huffmanGroups[id].weight[huffman->block[k]]++;
            }
            st = ed + 1;
        }
        for (int32_t j = 0; j < nGroups; j++)
        {
            BzpBuildTreeBalanceHeight(&huffman->huffmanGroups[j]);
        }
    }
    BzpGenerateSelectMTF(huffman);
    for (int32_t i = 0; i < nGroups; i++)
    {
        BzpGetHuffmanTable(&huffman->huffmanGroups[i]);
    }
}

BzpBwtDecodeInfo *BzpBwtDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpBwtDecodeInfo *bwt = (BzpBwtDecodeInfo *)malloc(sizeof(BzpBwtDecodeInfo));
    if (bwt == NULL)
    {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize;
    bwt->block = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->deCode = (uint8_t *)malloc(spaceSize * sizeof(uint8_t));
    bwt->sorted = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (bwt->block == NULL || bwt->sorted == NULL || bwt->deCode == NULL)
    {
        BzpBwtDecodeFinish(bwt);
        return NULL;
    }
    bwt->nBlock = 0;
    bwt->oriPtr = 0;
    return bwt;
}

void BzpBwtDecode(BzpBwtDecodeInfo *bwt)
{
    int32_t ftab[257];
    (void)memset_s(ftab, sizeof(ftab), 0, sizeof(ftab));
    for (int32_t i = 0; i < bwt->nBlock; i++)
    {
        ftab[bwt->block[i] + 1]++;
    }
    for (int32_t i = 1; i <= BZP_ASCII_SIZE; i++)
    {
        ftab[i] += ftab[i - 1];
    }
    for (int32_t i = 0; i < bwt->nBlock; i++)
    {
        uint8_t ch = bwt->block[i];
        bwt->sorted[ftab[ch]] = i;
        ftab[ch]++;
    }
    int32_t cnt = 0;
    int32_t pos = bwt->oriPtr;
    while (cnt < bwt->nBlock)
    {
        pos = bwt->sorted[pos];
        uint8_t ch = bwt->block[pos];
        bwt->deCode[cnt] = ch;
        cnt++;
    }
}

void BzpBwtDecodeFinish(BzpBwtDecodeInfo *bwt)
{
    if (bwt != NULL)
    {
        if (bwt->block != NULL)
        {
            free(bwt->block);
            bwt->block = NULL;
        }
        if (bwt->deCode != NULL)
        {
            free(bwt->deCode);
            bwt->deCode = NULL;
        }
        if (bwt->sorted != NULL)
        {
            free(bwt->sorted);
            bwt->sorted = NULL;
        }
        free(bwt);
        bwt = NULL;
    }
}

BzpHuffmanDecode *BzpHuffmanDecodeInit(int32_t blockSize)
{
    if (BZP_INVALID_BLOCK_SIZE(blockSize))
    {
        return NULL;
    }
    BzpHuffmanDecode *huffman = (BzpHuffmanDecode *)malloc(sizeof(BzpHuffmanDecode));
    if (huffman == NULL)
    {
        return NULL;
    }
    int32_t spaceSize = BZP_BASE_BLOCK_SIZE * blockSize / BZP_ELEMS_NUM_IN_ONE_GROUP;
    huffman->select = (int32_t *)malloc(spaceSize * sizeof(int32_t));
    if (huffman->select == NULL)
    {
        BzpHuffmanDecodeFinish(huffman);
    }
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));
    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
    return huffman;
}

void BzpHuffmanDecodeReset(BzpHuffmanDecode *huffman)
{
    (void)memset_s(huffman->base, sizeof(huffman->base), 0, sizeof(huffman->base));
    (void)memset_s(huffman->perm, sizeof(huffman->perm), 0, sizeof(huffman->perm));
    (void)memset_s(huffman->limit, sizeof(huffman->limit), 0, sizeof(huffman->limit));
    huffman->selectCnt = 0;
    huffman->deCodeNum = 0;
}

void BzpGetOneTable(BzpHuffmanDecode *huffman, int32_t t)
{
    int32_t vec = 0, cnt = 0;
    int32_t mi = huffman->len[t][0], mx = huffman->len[t][0];
    for (int32_t i = 0; i < huffman->alphaSize; i++)
    {
        mi = BZP_MIN_FUN(mi, huffman->len[t][i]);
        mx = BZP_MAX_FUN(mx, huffman->len[t][i]);
    }
    huffman->minLens[t] = mi;
    for (int32_t i = mi; i <= mx; i++)
    {
        for (int32_t j = 0; j < huffman->alphaSize; j++)
        {
            if (huffman->len[t][j] == i)
            {
                huffman->perm[t][cnt++] = j;
            }
        }
    }
    for (int32_t i = 0; i < huffman->alphaSize; i++)
    {
        huffman->base[t][huffman->len[t][i] + 1]++;
    }
    for (int32_t i = 1; i <= mx + 1; i++)
    {
        huffman->base[t][i] += huffman->base[t][i - 1];
    }
    for (int32_t i = mi; i <= mx; i++)
    {
        vec += (huffman->base[t][i + 1] - huffman->base[t][i]);
        huffman->limit[t][i] = vec - 1;
        vec <<= 1;
    }
    for (int32_t i = mi + 1; i <= mx; i++)
    {
        huffman->base[t][i] = ((huffman->limit[t][i - 1] + 1) << 1) - huffman->base[t][i];
    }
}

void BzpGenerateDecodeTable(BzpHuffmanDecode *huffman)
{
    for (int32_t t = 0; t < huffman->nGroups; t++)
    {
        BzpGetOneTable(huffman, t);
    }
}

void BzpHuffmanDecodeFinish(BzpHuffmanDecode *huffman)
{
    if (huffman != NULL)
    {
        if (huffman->select != NULL)
        {
            free(huffman->select);
            huffman->select = NULL;
        }
        free(huffman);
        huffman = NULL;
    }
}

