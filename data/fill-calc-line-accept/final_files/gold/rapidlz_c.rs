use crate::translation_utils::*;

pub type RapidlzLogFunc = FuncPtr<fn(Ptr<u8>, usize)>;

pub type RapidlzStreamCtx = TagRapidlzStreamCtx;

#[repr(C)]
#[derive(Default)]
pub struct TagRapidlzStreamCtx {
    pub hashTable: Array<u32, { RAPIDLZ_STREAM_HASH_SIZE!() }>,
    pub dict: Ptr<u8>,
    pub dictSize: u32,
    pub currentOffset: u32,
    pub acceleration: i32,
    pub strmCtxSpecific: Ptr<RapidlzStreamCtx>,
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct RapidlzUnalignU16 {
    pub v: u16,
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct RapidlzUnalignU32 {
    pub v: u32,
}

#[repr(C, packed)]
#[derive(Default, Clone, Copy)]
pub struct RapidlzUnalignU64 {
    pub v: u64,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct RapidlzCCtx {
    pub hashTable: Ptr<u8>,
    pub hashType: u8,
    pub hashBits: u8,
    pub step: u8,
    pub bufferLimit: u8,
}

pub static g_overlapOffAddVal: Global<Array<u8, 8>> = global!(arr![0, 1, 2, 2, 4, 3, 2, 1]);

pub static g_rapidlzLogFunc: Global<RapidlzLogFunc> = global!(NULL!());

pub static g_rapidlzVersion: Global<Ptr<u8>> = global!(cstr!("rapidlz 3.24.10.B201"));

macro_rules! RAPIDLZ_STREAM_HASH_SIZE {
    () => {
        1 << 12
    };
}
pub(crate) use RAPIDLZ_STREAM_HASH_SIZE;

macro_rules! RAPIDLZ_MAX_BYTE_VALUE {
    () => {
        255
    };
}
pub(crate) use RAPIDLZ_MAX_BYTE_VALUE;

macro_rules! RAPIDLZ_MAX_4BIT_VALUE {
    () => {
        15
    };
}
pub(crate) use RAPIDLZ_MAX_4BIT_VALUE;

macro_rules! RAPIDLZ_MIN_MATCH {
    () => {
        4
    };
}
pub(crate) use RAPIDLZ_MIN_MATCH;

macro_rules! RAPIDLZ_HASH_TYPE_4 {
    () => {
        4
    };
}
pub(crate) use RAPIDLZ_HASH_TYPE_4;

macro_rules! RAPIDLZ_HASH_TYPE_5 {
    () => {
        5
    };
}
pub(crate) use RAPIDLZ_HASH_TYPE_5;

macro_rules! RAPIDLZ_STEP_FORWARD_BASE {
    () => {
        6
    };
}
pub(crate) use RAPIDLZ_STEP_FORWARD_BASE;

macro_rules! RAPIDLZ_MAX_OFFSET {
    () => {
        65535
    };
}
pub(crate) use RAPIDLZ_MAX_OFFSET;

macro_rules! RAPIDLZ_EIGHT_BYTE {
    () => {
        8
    };
}
pub(crate) use RAPIDLZ_EIGHT_BYTE;

macro_rules! RAPIDLZ_SIXTEEN_BYTE {
    () => {
        16
    };
}
pub(crate) use RAPIDLZ_SIXTEEN_BYTE;

macro_rules! RAPIDLZ_COPY_PROTECT_SIZE {
    () => {
        16
    };
}
pub(crate) use RAPIDLZ_COPY_PROTECT_SIZE;

macro_rules! RAPIDLZ_INPUT_INVALID {
    () => {
        (-100isize) as usize
    };
}
pub(crate) use RAPIDLZ_INPUT_INVALID;

macro_rules! RAPIDLZ_MALLOC_FAILED {
    () => {
        (-99isize) as usize
    };
}
pub(crate) use RAPIDLZ_MALLOC_FAILED;

macro_rules! RAPIDLZ_DST_SIZE_SMALL {
    () => {
        (-98isize) as usize
    };
}
pub(crate) use RAPIDLZ_DST_SIZE_SMALL;

macro_rules! RAPIDLZ_SECUREC_ERROR {
    () => {
        (-97isize) as usize
    };
}
pub(crate) use RAPIDLZ_SECUREC_ERROR;

macro_rules! RAPIDLZ_FORMAT_INVALID {
    () => {
        (-96isize) as usize
    };
}
pub(crate) use RAPIDLZ_FORMAT_INVALID;

macro_rules! LOG_BUF_SIZE {
    () => {
        1024
    };
}
pub(crate) use LOG_BUF_SIZE;

macro_rules! RAPIDLZFILENAME {
    () => {
        if strrchr(__FILE__!(), b'/').as_bool() {
            strrchr(__FILE__!(), b'/') + 1
        } else {
            __FILE__!()
        }
    };
}
pub(crate) use RAPIDLZFILENAME;

macro_rules! RAPIDLZ_MAX_INPUT_SIZE {
    () => {
        0x7E000000
    };
}
pub(crate) use RAPIDLZ_MAX_INPUT_SIZE;

macro_rules! RAPIDLZ_MAX_4BIT_MATCH {
    () => {
        19
    };
}
pub(crate) use RAPIDLZ_MAX_4BIT_MATCH;

macro_rules! RAPIDLZ_ACCELERATION_MAX {
    () => {
        10
    };
}
pub(crate) use RAPIDLZ_ACCELERATION_MAX;

macro_rules! RAPIDLZ_SRC_SIZE_THRESHOLD {
    () => {
        65536
    };
}
pub(crate) use RAPIDLZ_SRC_SIZE_THRESHOLD;

macro_rules! RAPIDLZ_LAST_LITERALS {
    () => {
        6
    };
}
pub(crate) use RAPIDLZ_LAST_LITERALS;

macro_rules! RAPIDLZ_MIN_COMPRESS_SIZE {
    () => {
        16
    };
}
pub(crate) use RAPIDLZ_MIN_COMPRESS_SIZE;

macro_rules! RAPIDLZ_MIN_HASH_BIT {
    () => {
        6
    };
}
pub(crate) use RAPIDLZ_MIN_HASH_BIT;

macro_rules! RAPIDLZ_MAX_HASH_BIT {
    () => {
        12
    };
}
pub(crate) use RAPIDLZ_MAX_HASH_BIT;

macro_rules! RAPIDLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        RapidlzLogWrite(($error_code as usize).cast(), RAPIDLZFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr),*) => {
        RapidlzLogWrite(($error_code as usize).cast(), RAPIDLZFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    }
}
pub(crate) use RAPIDLZ_LOG;

macro_rules! RAPIDLZ_LIKELY {
    ($x:expr) => {
        $x
    };
}
pub(crate) use RAPIDLZ_LIKELY;

macro_rules! RAPIDLZ_UNLIKELY {
    ($x:expr) => {
        $x
    };
}
pub(crate) use RAPIDLZ_UNLIKELY;

macro_rules! RAPIDLZ_READ16BIT {
    ($ptr:expr) => {
        $ptr.cast::<Ptr<RapidlzUnalignU16>>().v
    };
}
pub(crate) use RAPIDLZ_READ16BIT;

macro_rules! RAPIDLZ_READ32BIT {
    ($ptr:expr) => {
        $ptr.cast::<Ptr<RapidlzUnalignU32>>().v
    };
}
pub(crate) use RAPIDLZ_READ32BIT;

macro_rules! RAPIDLZ_READ64BIT {
    ($ptr:expr) => {
        $ptr.cast::<Ptr<RapidlzUnalignU64>>().v
    };
}
pub(crate) use RAPIDLZ_READ64BIT;

macro_rules! RAPIDLZ_WRITE64BIT {
    ($ptr:expr, $val:expr) => {
        ($ptr.cast::<Ptr<RapidlzUnalignU64>>()).v = $val
    };
}
pub(crate) use RAPIDLZ_WRITE64BIT;

macro_rules! RAPIDLZ_ASSERT {
    ($x:expr) => {
        assert!($x)
    };
}
pub(crate) use RAPIDLZ_ASSERT;

macro_rules! RAPIDLZ_EXPAND_FORWARD {
    ($srcBegin:expr, $matchBegin:expr, $srcCurr:expr, $srcAnchor:expr) => {
        while $srcBegin < $matchBegin
            && $srcCurr > $srcAnchor
            && RAPIDLZ_UNLIKELY!(*($matchBegin - 1) == *($srcCurr - 1))
        {
            $matchBegin.minus_minus();
            $srcCurr.minus_minus();
        }
    };
}
pub(crate) use RAPIDLZ_EXPAND_FORWARD;

macro_rules! RAPIDLZ_READ_OPTIONAL_LENGTH {
    ($len:expr, $srcCurr:expr, $srcEnd:expr, $temp:expr) => {
        if RAPIDLZ_LIKELY!($srcCurr < $srcEnd) {
            $temp = (*$srcCurr.plus_plus()).cast();
            $len += $temp as u32;
        }
        while ($temp == RAPIDLZ_MAX_BYTE_VALUE!() && $srcCurr < $srcEnd) {
            $temp = (*$srcCurr.plus_plus()).cast();
            $len += $temp as u32;
        }
    };
}
pub(crate) use RAPIDLZ_READ_OPTIONAL_LENGTH;

macro_rules! SAFE_COPY_MATCH {
    ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) => {
        let mut matchLength = $matchLength;
        while matchLength > 0 {
            matchLength -= 1;
            *$dstCurr.plus_plus() = *$matchSrc.plus_plus();
        }
    };
}
pub(crate) use SAFE_COPY_MATCH;

macro_rules! RAPIDLZ_COMPRESSBOUND {
    ($size:expr) => {
        if ($size as u32) > RAPIDLZ_MAX_INPUT_SIZE!() {
            0
        } else {
            $size + ($size / 255) + 16
        }
    };
}
pub(crate) use RAPIDLZ_COMPRESSBOUND;

pub fn RapidlzIsLE() -> i32 {
    let mut n: i32 = 1;
    return (*c_ref!(n).cast::<Ptr<u8>>()).cast::<i32>();
}

pub fn RapidlzReadLE16Bit(mut addr: Ptr<Void>) -> u16 {
    if (RapidlzIsLE() != 0).as_bool() {
        return (*addr.cast::<Ptr<u16>>()).cast();
    }
    let mut tmp1: u8 = ((addr.cast::<Ptr<u8>>())[0]).cast();
    let mut tmp2: u8 = ((addr.cast::<Ptr<u8>>())[1]).cast();
    return (tmp1 + (tmp2 << 8)).cast::<u16>();
}

pub fn RapidlzCountTailZero64(mut x: u64) -> u8 {
    if (x == 0).as_bool() {
        return 0;
    }
    let mut val: u64 = x.cast();
    let mut num: u8 = 0;
    while (val & 1 == 0).as_bool() {
        val >>= 1;
        num += 1;
    }
    return num.cast();
}

pub fn RapidlzCountLeadZero64(mut x: u64) -> u8 {
    if (x == 0).as_bool() {
        return 0;
    }
    let mut num: u8 = 0;
    let mut val: u64 = x;
    while (val & 0x8000000000000000u64 == 0).as_bool() {
        val <<= 1;
        num += 1;
    }
    return num.cast();
}

pub fn RapidlzHighBit64(mut x: u64) -> u8 {
    RAPIDLZ_ASSERT!(x != 0);
    let mut pos: u8 = 64;
    let mut value: u64 = x;
    if (value == 0).as_bool() {
        return 0;
    }
    if (value & 0xFFFFFFFF00000000 == 0).as_bool() {
        value <<= 32;
        pos -= 32;
    }
    if (value & 0xFFFF000000000000 == 0).as_bool() {
        value <<= 16;
        pos -= 16;
    }
    if (value & 0xFF00000000000000 == 0).as_bool() {
        value <<= 8;
        pos -= 8;
    }
    if (value & 0xF000000000000000 == 0).as_bool() {
        value <<= 4;
        pos -= 4;
    }
    if (value & 0xC000000000000000 == 0).as_bool() {
        value <<= 2;
        pos -= 2;
    }
    if (value & 0x8000000000000000 == 0).as_bool() {
        value <<= 1;
        pos -= 1;
    }
    return (pos - 1).cast();
}

pub fn RapidlzWriteLE16(mut addr: Ptr<Void>, mut val: u16) {
    if (RapidlzIsLE() != 0).as_bool() {
        *addr.cast::<Ptr<u16>>() = val.cast();
    } else {
        let mut tmp: Ptr<u8> = addr.cast::<Ptr<u8>>();
        tmp[0] = val.cast::<u8>();
        tmp[1] = (val >> 8).cast::<u8>();
    }
}

pub fn RapidlzCopy16Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
    RAPIDLZ_WRITE64BIT!(
        dst.cast::<Ptr<u8>>() + 8,
        RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 8)
    );
}

pub fn RapidlzCopy8Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
}

pub fn RapidlzWildCopy16(mut srcPtr: Ptr<u8>, mut dstPtr: Ptr<u8>, mut dstEnd: Ptr<u8>) {
    let mut tmpDstPtr: Ptr<u8> = dstPtr.cast();
    let mut tmpSrcPtr: Ptr<u8> = srcPtr.cast();
    c_do!({
        RapidlzCopy16Byte(tmpDstPtr.cast(), tmpSrcPtr.cast());
        tmpDstPtr += 16;
        tmpSrcPtr += 16;
    } while tmpDstPtr < dstEnd);
}

pub fn RapidlzCopyLiteralsFast(mut src: Ptr<u8>, mut dst: Ptr<u8>, mut length: u32) {
    if RAPIDLZ_LIKELY!(length <= RAPIDLZ_SIXTEEN_BYTE!()).as_bool() {
        RapidlzCopy16Byte(dst.cast(), src.cast());
        return;
    }
    RapidlzWildCopy16(src.cast(), dst.cast(), (dst + length).cast());
}

pub fn RapidlzCompressExpandBackward(
    mut matchLimit: Ptr<u8>,
    mut matchPtr: Ptr<u8>,
    mut srcCurr: Ptr<u8>,
) -> Ptr<u8> {
    let mut xorVal: u64;
    let mut loopEnd: Ptr<u8> = (matchLimit - 7).cast();
    let mut srcCurrMatchEnd: Ptr<u8> = srcCurr.cast();
    let mut matchBegin: Ptr<u8> = matchPtr.cast();
    while (srcCurrMatchEnd < loopEnd).as_bool() {
        xorVal = RAPIDLZ_READ64BIT!(matchBegin) ^ RAPIDLZ_READ64BIT!(srcCurrMatchEnd);
        if RAPIDLZ_UNLIKELY!(xorVal == 0).as_bool() {
            srcCurrMatchEnd += c_sizeof!(u64);
            matchBegin += c_sizeof!(u64);
            continue;
        }
        srcCurrMatchEnd += if RapidlzIsLE().as_bool() {
            RapidlzCountTailZero64(xorVal.cast()).cast::<usize>() >> 3
        } else {
            RapidlzCountLeadZero64(xorVal.cast()).cast::<usize>() >> 3
        };
        return srcCurrMatchEnd.cast();
    }
    if (((srcCurrMatchEnd + 3) < matchLimit).as_bool()
        && (RAPIDLZ_READ32BIT!(srcCurrMatchEnd) == RAPIDLZ_READ32BIT!(matchBegin)).as_bool())
    {
        srcCurrMatchEnd += c_sizeof!(u32);
        matchBegin += c_sizeof!(u32);
    }
    if (((srcCurrMatchEnd + 1) < matchLimit).as_bool()
        && (RAPIDLZ_READ16BIT!(srcCurrMatchEnd) == RAPIDLZ_READ16BIT!(matchBegin)).as_bool())
    {
        srcCurrMatchEnd += c_sizeof!(u16);
        matchBegin += c_sizeof!(u16);
    }
    if ((srcCurrMatchEnd < matchLimit).as_bool() && (srcCurrMatchEnd[0] == matchBegin[0]).as_bool())
    {
        srcCurrMatchEnd += 1;
    }
    return srcCurrMatchEnd.cast();
}

pub fn RapidlzCopyMatchFast(
    mut dst: Ptr<u8>,
    mut r#match: Ptr<u8>,
    mut offset: u16,
    mut length: u32,
) {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut matchPtr: Ptr<u8> = r#match.cast();
    if (offset >= RAPIDLZ_SIXTEEN_BYTE!()).as_bool() {
        RapidlzCopyLiteralsFast(matchPtr.cast(), dstCurr.cast(), length.cast());
        return;
    }
    c_for!(let mut i: i32 = 0; i < RAPIDLZ_EIGHT_BYTE!().cast(); i.prefix_plus_plus(); {
        dstCurr[i] = matchPtr[i].cast();
    });
    if (length <= RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        return;
    }
    let mut dstEnd: Ptr<u8> = (dstCurr + length).cast();
    if (offset < RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        matchPtr += (*g_overlapOffAddVal.lock())[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
    }
    c_do!({
        RapidlzCopy8Byte(dstCurr.cast(), matchPtr.cast());
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
        matchPtr += RAPIDLZ_EIGHT_BYTE!();
    } while dstCurr < dstEnd);
}

pub fn RapidlzLogWrite(
    mut error_code: usize,
    mut file_name: Ptr<u8>,
    mut line: u16,
    mut fmt: Ptr<u8>,
    mut alist: VaList,
) {
    // alist already initialized at parameter list
    let mut output: Array<u8, { LOG_BUF_SIZE!() }> = Default::default();
    let mut retVal: i32;
    let mut len: i32;
    let mut func: RapidlzLogFunc = *g_rapidlzLogFunc.lock();
    let mut filename: Ptr<u8>;
    if (func == NULL!()).as_bool() {
        return;
    }
    filename = c_strdup!(file_name);
    if (filename == NULL!()).as_bool() {
        return;
    }
    retVal = c_snprintf_s!(
        output,
        LOG_BUF_SIZE!(),
        LOG_BUF_SIZE!() - 1,
        cstr!("\n[Rapidlz-Log] File={}, Line={}, Error={}\n"),
        c_basename!(filename),
        line,
        error_code
    );
    if (retVal < 0).as_bool() {
        c_free!(filename);
        return;
    }
    len = retVal;
    c_free!(filename);
    // va_start not needed
    retVal = c_vsnprintf_s!(
        output.cast::<Ptr<u8>>() + len,
        LOG_BUF_SIZE!() - len,
        LOG_BUF_SIZE!() - len - 1,
        fmt,
        alist
    );
    // va_end not needed
    if (retVal < 0).as_bool() {
        return;
    }
    func(output.cast(), c_strlen!(output) + 1);
}

pub fn RapidlzLogRegister(mut func: RapidlzLogFunc) {
    *g_rapidlzLogFunc.lock() = func;
}

pub fn RapidlzVersionGet() -> Ptr<u8> {
    return *g_rapidlzVersion.lock();
}

pub fn RapidlzCompressBound(mut srcSize: usize) -> usize {
    return RAPIDLZ_COMPRESSBOUND!(srcSize).cast();
}

pub fn RapidlzPutPosOnTable(
    mut pos: u32,
    mut hashValue: u32,
    mut hashTable: Ptr<u8>,
    mut hashType: u8,
) {
    if (hashType == 4).as_bool() {
        *((hashTable.cast::<Ptr<u16>>()) + hashValue) = pos.cast::<u16>();
    } else if (hashType == 5).as_bool() {
        *((hashTable.cast::<Ptr<u32>>()) + hashValue) = pos.cast::<u32>();
    }
}

pub fn RapidlzGetPosOnTable(mut hashValue: u32, mut hashTable: Ptr<u8>, mut hashType: u8) -> u32 {
    if (hashType == 4).as_bool() {
        return (*(hashTable.cast::<Ptr<u16>>() + hashValue)).cast();
    } else if (hashType == 5).as_bool() {
        return (*(hashTable.cast::<Ptr<u32>>() + hashValue)).cast();
    }
    return 0;
}

pub fn RapidlzCalcHashValue(mut srcCurr: Ptr<u8>, mut hashType: u8, mut hashBits: u8) -> u32 {
    if (hashType == 5).as_bool() {
        return (((RAPIDLZ_READ64BIT!(srcCurr) << 24) * 11400714819323198485u64)
            >> (64 - hashBits))
            .cast::<u32>();
    } else {
        return (RAPIDLZ_READ32BIT!(srcCurr) * 2654435769u32) >> (32 - hashBits);
    }
}

pub fn RapidlzCompressStoreOptionalLength(mut dst: Ptr<u8>, mut litLength: u32) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut length: u32 = litLength.cast();
    if (length < RAPIDLZ_MAX_BYTE_VALUE!()).as_bool() {
        *dstCurr = length.cast::<u8>();
        dstCurr += 1;
        return dstCurr.cast();
    }
    c_do!({
        *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!();
        dstCurr += 1;
        length -= RAPIDLZ_MAX_BYTE_VALUE!();
    } while length >= RAPIDLZ_MAX_BYTE_VALUE!());
    *dstCurr = length.cast::<u8>();
    dstCurr += 1;
    return dstCurr.cast();
}

pub fn RapidlzStoreLastLiterals(
    mut dst: Ptr<u8>,
    mut dstEnd: Ptr<u8>,
    mut srcCurr: Ptr<u8>,
    mut litLength: u32,
    mut bufferLimit: u8,
) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    if (bufferLimit != 0).as_bool() {
        let mut litTokSize: u32 = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE!());
        if (dstCurr + litTokSize > dstEnd).as_bool() {
            RAPIDLZ_LOG!(
                RAPIDLZ_DST_SIZE_SMALL!(),
                cstr!("dstEnd - dstCur:{} litTokSize:{}\n"),
                dstEnd - dstCurr,
                litTokSize
            );
            return NULL!();
        }
    }
    let mut token: u8 = (if litLength < RAPIDLZ_MAX_4BIT_VALUE!() {
        litLength
    } else {
        RAPIDLZ_MAX_4BIT_VALUE!()
    } << 4)
        .cast();
    *dstCurr = token.cast();
    dstCurr += 1;
    if (litLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        dstCurr = RapidlzCompressStoreOptionalLength(
            dstCurr.cast(),
            (litLength - RAPIDLZ_MAX_4BIT_VALUE!()).cast(),
        )
        .cast();
    }
    if (c_memcpy_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLength) != EOK!()).as_bool() {
        RAPIDLZ_LOG!(
            RAPIDLZ_SECUREC_ERROR!(),
            cstr!("dstEnd - dstCurr:{} litLength{}\n"),
            dstEnd - dstCurr,
            litLength
        );
        return NULL!();
    }
    return (dstCurr + litLength).cast();
}

pub fn RapidlzStoreOffMatch(
    mut dst: Ptr<u8>,
    mut token: Ptr<u8>,
    mut matchLength: u32,
    mut offset: u16,
) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    RapidlzWriteLE16(dstCurr.cast(), offset.cast());
    dstCurr += 2;
    if (matchLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        let mut optionalLen: u32 = matchLength - RAPIDLZ_MAX_4BIT_VALUE!();
        *token += RAPIDLZ_MAX_4BIT_VALUE!();
        c_for!(; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE!(); optionalLen -= RAPIDLZ_MAX_BYTE_VALUE!(); {
            *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!();
            dstCurr += 1;
        });
        *dstCurr = optionalLen.cast::<u8>();
        dstCurr += 1;
    } else {
        *token += matchLength.cast::<u8>();
    }
    return dstCurr.cast();
}

pub fn RapidlzStoreSequence(
    mut dst: Ptr<u8>,
    mut srcAnchor: Ptr<u8>,
    mut literalLength: u32,
    mut matchLength: u32,
    mut offset: u16,
) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut token: Ptr<u8> = dstCurr.suffix_plus_plus();
    if (literalLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        *token = (RAPIDLZ_MAX_4BIT_VALUE!() << 4).cast();
        let mut optionalLen: u32 = literalLength - RAPIDLZ_MAX_4BIT_VALUE!();
        c_for!(; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE!(); optionalLen -= RAPIDLZ_MAX_BYTE_VALUE!(); {
            *dstCurr.suffix_plus_plus() = RAPIDLZ_MAX_BYTE_VALUE!().cast();
        });
        *dstCurr.suffix_plus_plus() = optionalLen.cast();
        RapidlzCopy16Byte(dstCurr.cast(), srcAnchor.cast());
        if (literalLength > 16).as_bool() {
            RapidlzWildCopy16(
                (srcAnchor + 16).cast(),
                (dstCurr + 16).cast(),
                (dstCurr + literalLength).cast(),
            );
        }
        dstCurr += literalLength;
    } else if (literalLength > 0).as_bool() {
        *token = (literalLength << 4).cast();
        RapidlzCopy16Byte(dstCurr.cast(), srcAnchor.cast());
        dstCurr += literalLength;
    } else {
        *token = 0;
    }
    return RapidlzStoreOffMatch(
        dstCurr.cast(),
        token.cast(),
        matchLength.cast(),
        offset.cast(),
    );
}

pub fn RapidlzCompressProcess(
    mut dst: Ptr<Void>,
    mut dstSize: usize,
    mut src: Ptr<Void>,
    mut srcSize: usize,
    mut cCtx: Ptr<RapidlzCCtx>,
) -> usize {
    let mut hashValue: u32;
    let mut matchLength: u32;
    let mut literalLength: u32;
    let mut step: u32 = 1;
    let mut offset: u16;
    let mut hashTable: Ptr<u8> = cCtx.hashTable;
    let mut srcBegin: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEnd: Ptr<u8> = src.cast::<Ptr<u8>>() + srcSize;
    let mut srcCurr: Ptr<u8> = srcBegin + 1;
    let mut srcCurrMatchEnd: Ptr<u8>;
    let mut srcAnchor: Ptr<u8> = srcBegin;
    let mut matchBegin: Ptr<u8>;
    let mut matchLimit: Ptr<u8> = srcEnd - RAPIDLZ_LAST_LITERALS!();
    let mut srcLimit: Ptr<u8> = srcEnd - RAPIDLZ_MIN_COMPRESS_SIZE!();
    let mut dstBegin: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEnd: Ptr<u8> = dst.cast::<Ptr<u8>>() + dstSize;
    let mut dstCurr: Ptr<u8> = dstBegin;
    let mut hashType: u8 = cCtx.hashType;
    let mut hashBits: u8 = cCtx.hashBits;
    let mut searchMatchNb: u32 = (cCtx.step << RAPIDLZ_STEP_FORWARD_BASE!()).cast::<u32>();
    let mut searchMatchNbTmp: u32 = searchMatchNb;
    let mut bufferLimit: u8 = cCtx.bufferLimit;
    while RAPIDLZ_LIKELY!(srcCurr <= srcLimit) {
        loop {
            hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
            matchBegin = (srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType));
            RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue, hashTable, hashType);
            if (RAPIDLZ_READ32BIT!(srcCurr) == RAPIDLZ_READ32BIT!(matchBegin))
                && RAPIDLZ_LIKELY!((srcCurr - matchBegin) <= RAPIDLZ_MAX_OFFSET!())
            {
                break;
            }
            srcCurr += step;
            step = (searchMatchNbTmp >> RAPIDLZ_STEP_FORWARD_BASE!());
            searchMatchNbTmp += 1;
            if (srcCurr > srcLimit) {
                dstCurr = RapidlzStoreLastLiterals(
                    dstCurr,
                    dstEnd,
                    srcAnchor,
                    (srcEnd - srcAnchor).cast(),
                    bufferLimit,
                );
                if (dstCurr == NULL!()) {
                    return 0;
                }
                return (dstCurr - dstBegin);
            }
        }
        step = 1;
        searchMatchNbTmp = searchMatchNb;
        srcCurrMatchEnd = RapidlzCompressExpandBackward(
            matchLimit,
            (matchBegin + RAPIDLZ_MIN_MATCH!()),
            (srcCurr + RAPIDLZ_MIN_MATCH!()),
        );
        RAPIDLZ_EXPAND_FORWARD!(srcBegin, matchBegin, srcCurr, srcAnchor);
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH!()).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        literalLength = (srcCurr - srcAnchor).cast();
        if (bufferLimit != 0) {
            let mut writeSize: u32 =
                literalLength + 8 + (literalLength + matchLength / RAPIDLZ_MAX_BYTE_VALUE!());
            if RAPIDLZ_UNLIKELY!((dstCurr + writeSize) > dstEnd) {
                RAPIDLZ_LOG!(
                    RAPIDLZ_DST_SIZE_SMALL!(),
                    cstr!("dstEnd - dstCur:{} writeSize:{}\n"),
                    (dstEnd - dstCurr),
                    writeSize
                );
                return 0;
            }
        }
        dstCurr = RapidlzStoreSequence(dstCurr, srcAnchor, literalLength, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue((srcCurr - 2), hashType, hashBits);
        RapidlzPutPosOnTable(
            (srcCurr - 2 - srcBegin).cast(),
            hashValue,
            hashTable,
            hashType,
        );
        if RAPIDLZ_UNLIKELY!(srcCurr > srcLimit) {
            break;
        }
        hashValue = RapidlzCalcHashValue(srcCurr, hashType, hashBits);
        matchBegin = (srcBegin + RapidlzGetPosOnTable(hashValue, hashTable, hashType));
        RapidlzPutPosOnTable((srcCurr - srcBegin).cast(), hashValue, hashTable, hashType);
        if (RAPIDLZ_READ32BIT!(srcCurr) != RAPIDLZ_READ32BIT!(matchBegin))
            || RAPIDLZ_UNLIKELY!((srcCurr - matchBegin) > RAPIDLZ_MAX_OFFSET!())
        {
            srcCurr += 1;
            continue;
        }
        srcCurrMatchEnd = RapidlzCompressExpandBackward(
            matchLimit,
            (matchBegin + RAPIDLZ_MIN_MATCH!()),
            (srcCurr + RAPIDLZ_MIN_MATCH!()),
        );
        matchLength = (srcCurrMatchEnd - srcCurr - RAPIDLZ_MIN_MATCH!()).cast();
        offset = (srcCurr - matchBegin).cast::<u16>();
        if (bufferLimit != 0) {
            let mut writeSize: u32 = 8 + matchLength / RAPIDLZ_MAX_BYTE_VALUE!();
            if RAPIDLZ_UNLIKELY!((dstCurr + writeSize) > dstEnd) {
                RAPIDLZ_LOG!(
                    RAPIDLZ_DST_SIZE_SMALL!(),
                    cstr!("dstEnd - dstCur:{} writeSize:{}\n"),
                    (dstEnd - dstCurr),
                    writeSize
                );
                return 0;
            }
        }
        *dstCurr = 0;
        dstCurr = RapidlzStoreOffMatch((dstCurr + 1), dstCurr, matchLength, offset);
        srcCurr = srcCurrMatchEnd;
        srcAnchor = srcCurr;
        hashValue = RapidlzCalcHashValue((srcCurr - 2), hashType, hashBits);
        RapidlzPutPosOnTable(
            (srcCurr - 2 - srcBegin).cast(),
            hashValue,
            hashTable,
            hashType,
        );
    }
    if (srcAnchor < srcEnd) {
        dstCurr = RapidlzStoreLastLiterals(
            dstCurr,
            dstEnd,
            srcAnchor,
            (srcEnd - srcAnchor).cast(),
            bufferLimit,
        );
        if (dstCurr == NULL!()) {
            return 0;
        }
    }
    return (dstCurr - dstBegin);
}

pub fn RapidlzCCtxFree(mut cCtx: Ptr<RapidlzCCtx>) {
    if (cCtx != NULL!()).as_bool() {
        if (cCtx.hashTable != NULL!()).as_bool() {
            c_free!(cCtx.hashTable);
            cCtx.hashTable = NULL!();
        }
        c_free!(cCtx);
    }
}

pub fn RapidlzCompress(
    mut src: Ptr<Void>,
    mut dst: Ptr<Void>,
    mut srcSize: usize,
    mut dstSize: usize,
    mut acceleration: i32,
) -> usize {
    if (src == NULL!()) || (dst == NULL!()) || (srcSize == 0) || (dstSize == 0) {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("input invalid\n"));
        return 0;
    }
    if (acceleration < 1) || (acceleration > RAPIDLZ_ACCELERATION_MAX!()) {
        RAPIDLZ_LOG!(
            RAPIDLZ_INPUT_INVALID!(),
            cstr!("acceleration:{}\n"),
            acceleration
        );
        return 0;
    }
    let mut cCtx: Ptr<RapidlzCCtx> = c_malloc!(c_sizeof!(RapidlzCCtx));
    if (cCtx == NULL!()) {
        RAPIDLZ_LOG!(RAPIDLZ_MALLOC_FAILED!(), cstr!("cCtx malloc failed\n"));
        return 0;
    }
    cCtx.hashBits = RAPIDLZ_MIN_HASH_BIT!();
    let mut totalHashSize: usize;
    if (srcSize <= RAPIDLZ_SRC_SIZE_THRESHOLD!()) {
        cCtx.hashType = RAPIDLZ_HASH_TYPE_4!();
        if (srcSize >= 64) {
            cCtx.hashBits =
                if RapidlzHighBit64(srcSize.try_into().unwrap()) > RAPIDLZ_MAX_HASH_BIT!() {
                    RAPIDLZ_MAX_HASH_BIT!() + 1
                } else {
                    RapidlzHighBit64(srcSize.try_into().unwrap())
                };
        }
        totalHashSize = (c_sizeof!(u16) * (1 << cCtx.hashBits).cast::<u32>()).cast();
    } else {
        cCtx.hashType = RAPIDLZ_HASH_TYPE_5!();
        cCtx.hashBits = RAPIDLZ_MAX_HASH_BIT!();
        totalHashSize = (c_sizeof!(u32) * (1 << cCtx.hashBits).cast::<u32>()).cast();
    }
    let mut table: Ptr<u8> = c_malloc!(totalHashSize);
    if (table == NULL!()) {
        RAPIDLZ_LOG!(
            RAPIDLZ_MALLOC_FAILED!(),
            cstr!("hash table malloc failed\n")
        );
        c_free!(cCtx);
        return 0;
    }
    c_memset_s!(table, totalHashSize, 0, totalHashSize).cast::<Void>();
    cCtx.hashTable = table;
    cCtx.step = acceleration.cast::<u8>();
    cCtx.bufferLimit = (dstSize < RapidlzCompressBound(srcSize)).cast::<u8>();
    let mut cSize: usize = RapidlzCompressProcess(dst, dstSize, src, srcSize, cCtx);
    RapidlzCCtxFree(cCtx);
    return cSize;
}

pub fn RapidlzCompressDefault(
    mut src: Ptr<Void>,
    mut dst: Ptr<Void>,
    mut srcSize: usize,
    mut dstSize: usize,
) -> usize {
    return RapidlzCompress(src.cast(), dst.cast(), srcSize.cast(), dstSize.cast(), 1).cast();
}

pub fn RapidlzDecompress(
    mut src: Ptr<Void>,
    mut dst: Ptr<Void>,
    mut srcSize: usize,
    mut dstSize: usize,
) -> usize {
    if (src == NULL!()) || (dst == NULL!()) || (srcSize == 0) || (dstSize == 0) {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("input invalid\n"));
        return 0;
    }
    let mut token: u8 = Default::default();
    let mut temp: u32 = 0;
    let mut offset: u16 = Default::default();
    let mut litLen: u32 = Default::default();
    let mut matchLen: u32 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut srcEnd: Ptr<u8> = (src.cast::<Ptr<u8>>() + srcSize);
    let mut srcCurr: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!());
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize);
    let mut dstCurr: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEndFast: Ptr<u8> = (dstEnd - RAPIDLZ_COPY_PROTECT_SIZE!());
    while (srcCurr < srcEnd) {
        'READ_MATCH: {
            token = *srcCurr;
            srcCurr += 1;
            litLen = (token >> 4).cast();
            if RAPIDLZ_LIKELY!(litLen < RAPIDLZ_MAX_4BIT_VALUE!()) {
                if RAPIDLZ_LIKELY!(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast)
                {
                    RapidlzCopy16Byte(dstCurr, srcCurr);
                    dstCurr += litLen;
                    srcCurr += litLen;
                    break 'READ_MATCH;
                }
            } else {
                RAPIDLZ_READ_OPTIONAL_LENGTH!(litLen, srcCurr, srcEnd, temp);
                if RAPIDLZ_LIKELY!(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast)
                {
                    RapidlzWildCopy16(srcCurr, dstCurr, (dstCurr + litLen));
                    dstCurr += litLen;
                    srcCurr += litLen;
                    break 'READ_MATCH;
                }
            }
            let mut leftSrcSize: usize = (srcEnd - srcCurr);
            if RAPIDLZ_UNLIKELY!(
                litLen > leftSrcSize.try_into().unwrap()
                    || c_memmove_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLen) != EOK!()
            ) {
                RAPIDLZ_LOG!(
                    RAPIDLZ_DST_SIZE_SMALL!(),
                    cstr!("litLen:{} dstEnd - dst:{}\n"),
                    litLen,
                    leftSrcSize
                );
                return 0;
            }
            dstCurr += litLen;
            srcCurr += litLen;
            if (leftSrcSize == litLen.try_into().unwrap()) {
                return (dstCurr - dst.cast::<Ptr<u8>>());
            }
        }
        if RAPIDLZ_UNLIKELY!(srcCurr > srcEnd - 2) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr);
        srcCurr += 2;
        matchSrc = (dstCurr - offset);
        if RAPIDLZ_UNLIKELY!(matchSrc.cast::<Ptr<Void>>() < dst) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }
        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE!()).cast::<u32>() + RAPIDLZ_MIN_MATCH!();
        if (matchLen == RAPIDLZ_MAX_4BIT_MATCH!()) {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(matchLen, srcCurr, srcEnd, temp);
        }
        if RAPIDLZ_LIKELY!(dstCurr + matchLen <= dstEndFast) {
            RapidlzCopyMatchFast(dstCurr, matchSrc, offset, matchLen);
            dstCurr += matchLen;
        } else {
            if (dstCurr + matchLen > dstEnd) {
                RAPIDLZ_LOG!(
                    RAPIDLZ_DST_SIZE_SMALL!(),
                    cstr!("dstEnd - dstCurr:{} matchLen:{}\n"),
                    dstEnd - dstCurr,
                    matchLen
                );
                return 0;
            }
            SAFE_COPY_MATCH!(dstCurr, matchSrc, matchLen);
        }
    }
    return (dstCurr - dst.cast::<Ptr<u8>>());
}
