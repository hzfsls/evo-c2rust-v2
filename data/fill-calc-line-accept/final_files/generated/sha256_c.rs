use crate::translation_utils::*;

#[repr(C)]
#[derive(Default)]
pub struct VOS_SHA256_CTX {
    pub h: Array<u32, 8>,
    pub N: Array<u32, 2>,
    pub block: Array<u32, { SHA256_BLOCK_SIZE!() / std::mem::size_of::<u32>() }>,
    pub blocklen: u32,
    pub outlen: u32,
    pub computed: u32,
    pub corrupted: u32,
}

pub const K256: Array<u32, 64> = arr![
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

macro_rules! SHA256_BLOCK_SIZE {
    () => {
        64
    };
}
pub(crate) use SHA256_BLOCK_SIZE;

macro_rules! SHA256_DIGEST_SIZE {
    () => {
        32
    };
}
pub(crate) use SHA256_DIGEST_SIZE;

macro_rules! SHA256_OK {
    () => {
        0
    };
}
pub(crate) use SHA256_OK;

macro_rules! SHA256_ERROR {
    () => {
        (!0isize) as u32
    };
}
pub(crate) use SHA256_ERROR;

macro_rules! BITS_PRE_BYTE {
    () => {
        8
    };
}
pub(crate) use BITS_PRE_BYTE;

macro_rules! SHIFTS_PER_BYTE {
    () => {
        3
    };
}
pub(crate) use SHIFTS_PER_BYTE;

macro_rules! BITSIZE {
    ($t:ty) => {
        c_sizeof!($t) * BITS_PRE_BYTE!()
    };
}
pub(crate) use BITSIZE;

macro_rules! PUT_UINT32_BE {
    ($v:expr, $p:expr, $i:expr) => {
        $p[$i + 0] = ($v >> 24) as u8;
        $p[$i + 1] = ($v >> 16) as u8;
        $p[$i + 2] = ($v >> 8) as u8;
        $p[$i + 3] = ($v >> 0) as u8;
    };
}
pub(crate) use PUT_UINT32_BE;

macro_rules! GET_UINT32_BE {
    ($p:expr, $i:expr) => {
        (($p[$i + 0] as u32) << 24)
            | (($p[$i + 1] as u32) << 16)
            | (($p[$i + 2] as u32) << 8)
            | (($p[$i + 3] as u32) << 0)
    };
}
pub(crate) use GET_UINT32_BE;

macro_rules! VOS_ROTR32 {
    ($x:expr, $uiBlcLen:expr) => {
        ($x << (32 - $uiBlcLen)) | ($x >> $uiBlcLen)
    };
}
pub(crate) use VOS_ROTR32;

macro_rules! VOS_ROUND {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr, $k:expr, $W:expr) => {
        $h += (VOS_ROTR32!($e, 6) ^ VOS_ROTR32!($e, 11) ^ VOS_ROTR32!($e, 25))
            + ($g ^ ($e & ($f ^ $g)))
            + $k
            + $W[$i];
        $d += $h;
        $h += (VOS_ROTR32!($a, 2) ^ VOS_ROTR32!($a, 13) ^ VOS_ROTR32!($a, 22))
            + (($a & ($b | $c)) | ($b & $c));
    };
}
pub(crate) use VOS_ROUND;

pub fn vosSha256Begin(mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    if (pstCtx == NULL!()) {
        return;
    }
    c_memset_s!(
        pstCtx,
        c_sizeof!(VOS_SHA256_CTX),
        0,
        c_sizeof!(VOS_SHA256_CTX)
    )
    .cast::<Void>();
    pstCtx.h[0] = 0x6a09e667u32;
    pstCtx.h[1] = 0xbb67ae85u32;
    pstCtx.h[2] = 0x3c6ef372u32;
    pstCtx.h[3] = 0xa54ff53au32;
    pstCtx.h[4] = 0x510e527fu32;
    pstCtx.h[5] = 0x9b05688cu32;
    pstCtx.h[6] = 0x1f83d9abu32;
    pstCtx.h[7] = 0x5be0cd19u32;
    pstCtx.outlen = SHA256_DIGEST_SIZE!();
}

pub fn vosSha256CtxPrepare(mut pstCtx: Ptr<VOS_SHA256_CTX>, mut uiLen: u32) -> u32 {
    let mut uiCntFirst: u32;
    let mut uiCntSec: u32;
    uiCntFirst = (pstCtx.N[0] + (uiLen << SHIFTS_PER_BYTE!())) & 0xffffffff;
    if (uiCntFirst < pstCtx.N[0]).as_bool() {
        pstCtx.N[1].prefix_plus_plus();
        if (pstCtx.N[1] == 0).as_bool() {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
    }
    uiCntSec = pstCtx.N[1] + (uiLen >> (BITSIZE!(u32) - SHIFTS_PER_BYTE!()));
    if (uiCntSec < pstCtx.N[1]).as_bool() {
        pstCtx.corrupted = 1;
        return SHA256_ERROR!();
    }
    pstCtx.N[1] = uiCntSec.cast();
    pstCtx.N[0] = uiCntFirst.cast();
    return SHA256_OK!();
}

pub fn vosSha256LastPadding(
    mut pucData: Ptr<u8>,
    mut uiLen: u32,
    mut pstCtx: Ptr<VOS_SHA256_CTX>,
    mut puiPaddingLen: Ptr<u32>,
) -> u32 {
    let mut err: errno_t = Default::default();
    let mut uiBlcLen: u32 = pstCtx.blocklen;
    let mut pucBlock: Ptr<u8> = pstCtx.block.cast::<Ptr<u8>>();
    if (uiLen >= SHA256_BLOCK_SIZE!()) || (uiLen + uiBlcLen >= SHA256_BLOCK_SIZE!()) {
        err = c_memcpy_s!(
            pucBlock + uiBlcLen,
            SHA256_BLOCK_SIZE!() - uiBlcLen,
            pucData,
            SHA256_BLOCK_SIZE!() - uiBlcLen
        );
        if (err != EOK!()) {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
        vosSha256CompressMul(pstCtx, pucBlock, 1);
        *puiPaddingLen = (SHA256_BLOCK_SIZE!() - uiBlcLen);
        pstCtx.blocklen = 0;
        c_memset_s!(pucBlock, SHA256_BLOCK_SIZE!(), 0, SHA256_BLOCK_SIZE!()).cast::<Void>();
    } else {
        err = c_memcpy_s!(
            pucBlock + uiBlcLen,
            SHA256_BLOCK_SIZE!() - uiBlcLen,
            pucData,
            uiLen
        );
        if (err != EOK!()) {
            pstCtx.corrupted = 1;
            return SHA256_ERROR!();
        }
        pstCtx.blocklen += uiLen;
        return SHA256_ERROR!();
    }
    return SHA256_OK!();
}

pub fn vosSha256HashByBlcMulti(
    mut pucData: Ptr<u8>,
    mut uiLen: u32,
    mut pstCtx: Ptr<VOS_SHA256_CTX>,
) {
    let mut err: errno_t = Default::default();
    let mut uiBlcLen: u32;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData;
    uiBlcLen = (uiLenTmp / SHA256_BLOCK_SIZE!());
    if (uiBlcLen > 0) {
        vosSha256CompressMul(pstCtx, pucSrc, uiBlcLen);
        uiBlcLen *= SHA256_BLOCK_SIZE!();
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }
    if (uiLenTmp != 0) {
        pstCtx.blocklen = uiLenTmp;
        err = c_memcpy_s!(
            pstCtx.block.cast::<Ptr<u8>>(),
            SHA256_BLOCK_SIZE!(),
            pucSrc,
            uiLenTmp
        );
        if (err != EOK!()) {
            pstCtx.corrupted = 1;
            return;
        }
    }
    return;
}

pub fn vosSha256Hash(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiBlcLen: u32 = 0;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData.cast();
    if (pucSrc == NULL!()).as_bool()
        || (uiLenTmp == 0).as_bool()
        || (pstCtx == NULL!()).as_bool()
        || (pstCtx.corrupted == 1).as_bool()
        || (pstCtx.computed == 1).as_bool()
        || (vosSha256CtxPrepare(pstCtx.cast(), uiLenTmp.cast()) != SHA256_OK!()).as_bool()
    {
        return;
    }
    if (pstCtx.blocklen != 0).as_bool() {
        if (vosSha256LastPadding(
            pucSrc.cast(),
            uiLenTmp.cast(),
            pstCtx.cast(),
            c_ref!(uiBlcLen).cast(),
        ) == SHA256_OK!())
        .as_bool()
        {
            pucSrc += uiBlcLen;
            uiLenTmp -= uiBlcLen;
        } else {
            return;
        }
    }
    vosSha256HashByBlcMulti(pucSrc.cast(), uiLenTmp.cast(), pstCtx.cast());
    return;
}

pub fn vosSha256End(mut pucOut: Ptr<u8>, mut uiOutSize: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiIndex: u32 = Default::default();
    let mut pucBlock: Ptr<u8> = NULL!();
    let mut uiBlcLen: u32 = Default::default();
    if (pstCtx == NULL!()).as_bool() {
        return;
    }
    pucBlock = pstCtx.block.cast::<Ptr<u8>>();
    uiBlcLen = pstCtx.blocklen.cast();
    if (pstCtx.corrupted == 1).as_bool() || (uiOutSize < pstCtx.outlen).as_bool() {
        c_memset_s!(
            pstCtx,
            c_sizeof!(VOS_SHA256_CTX),
            0,
            c_sizeof!(VOS_SHA256_CTX)
        )
        .cast::<Void>();
        return;
    }
    if (pstCtx.computed == 0).as_bool() {
        pucBlock[uiBlcLen] = 0x80;
        uiBlcLen += 1;
        if (uiBlcLen > (SHA256_BLOCK_SIZE!() - 8)).as_bool() {
            c_memset_s!(
                pucBlock + uiBlcLen,
                SHA256_BLOCK_SIZE!() - uiBlcLen,
                0,
                SHA256_BLOCK_SIZE!() - uiBlcLen
            )
            .cast::<Void>();
            uiBlcLen = 0;
            vosSha256CompressMul(pstCtx.cast(), pucBlock.cast(), 1);
        }
        c_memset_s!(
            pucBlock + uiBlcLen,
            SHA256_BLOCK_SIZE!() - uiBlcLen,
            0,
            SHA256_BLOCK_SIZE!() - 8 - uiBlcLen
        )
        .cast::<Void>();
        pucBlock += SHA256_BLOCK_SIZE!() - 8;
        PUT_UINT32_BE!(pstCtx.N[1], pucBlock, 0);
        pucBlock += c_sizeof!(u32);
        PUT_UINT32_BE!(pstCtx.N[0], pucBlock, 0);
        pucBlock += c_sizeof!(u32);
        pucBlock -= SHA256_BLOCK_SIZE!();
        vosSha256CompressMul(pstCtx.cast(), pucBlock.cast(), 1);
        pstCtx.blocklen = 0;
        c_memset_s!(pucBlock, SHA256_BLOCK_SIZE!(), 0, SHA256_BLOCK_SIZE!()).cast::<Void>();
        pstCtx.computed = 1;
    }
    uiBlcLen = if pstCtx.outlen <= uiOutSize {
        pstCtx.outlen
    } else {
        uiOutSize
    } / c_sizeof!(u32);
    if (pucOut != NULL!()).as_bool() {
        c_for!(uiIndex = 0; uiIndex < uiBlcLen; uiIndex.suffix_plus_plus(); {
            PUT_UINT32_BE!(pstCtx.h[uiIndex], pucOut, c_sizeof!(u32) * uiIndex);
        });
    }
    return;
}

pub fn vosSha256CompressBlock(mut state: Ptr<u32>, mut block: Ptr<u8>) {
    let mut W: Array<u32, 64> = Default::default();
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();
    let mut a: u32 = Default::default();
    let mut b: u32 = Default::default();
    let mut c: u32 = Default::default();
    let mut d: u32 = Default::default();
    let mut e: u32 = Default::default();
    let mut f: u32 = Default::default();
    let mut g: u32 = Default::default();
    let mut h: u32 = Default::default();
    c_for!(i = 0; i < 16; i.suffix_plus_plus(); {
        W[i] = GET_UINT32_BE!(block, 4 * i);
    });
    c_for!(i = 16; i < 64; i.suffix_plus_plus(); {
        W[i] = W[i - 16] + W[i - 7] + (VOS_ROTR32!(W[i - 15], 7) ^ VOS_ROTR32!(W[i - 15], 18) ^ (W[i - 15] >> 3)) +
               (VOS_ROTR32!(W[i - 2], 17) ^ VOS_ROTR32!(W[i - 2], 19) ^ (W[i - 2] >> 10));
    });
    j = 0;
    a = state[j];
    j += 1;
    b = state[j];
    j += 1;
    c = state[j];
    j += 1;
    d = state[j];
    j += 1;
    e = state[j];
    j += 1;
    f = state[j];
    j += 1;
    g = state[j];
    j += 1;
    h = state[j];
    c_for!(i = 0; i < 64; i += 8; {
        j = 0;
        VOS_ROUND!(a, b, c, d, e, f, g, h, i + j, K256[i + 0], W);
        j += 1;
        VOS_ROUND!(h, a, b, c, d, e, f, g, i + j, K256[i + 1], W);
        j += 1;
        VOS_ROUND!(g, h, a, b, c, d, e, f, i + j, K256[i + 2], W);
        j += 1;
        VOS_ROUND!(f, g, h, a, b, c, d, e, i + j, K256[i + 3], W);
        j += 1;
        VOS_ROUND!(e, f, g, h, a, b, c, d, i + j, K256[i + 4], W);
        j += 1;
        VOS_ROUND!(d, e, f, g, h, a, b, c, i + j, K256[i + 5], W);
        j += 1;
        VOS_ROUND!(c, d, e, f, g, h, a, b, i + j, K256[i + 6], W);
        j += 1;
        VOS_ROUND!(b, c, d, e, f, g, h, a, i + j, K256[i + 7], W);
    });
    j = 0;
    state[j] += a;
    j += 1;
    state[j] += b;
    j += 1;
    state[j] += c;
    j += 1;
    state[j] += d;
    j += 1;
    state[j] += e;
    j += 1;
    state[j] += f;
    j += 1;
    state[j] += g;
    j += 1;
    state[j] += h;
}

pub fn vosSha256CompressMul(
    mut pstCtx: Ptr<VOS_SHA256_CTX>,
    mut pucInput: Ptr<u8>,
    mut uiNum: u32,
) {
    let mut uiNumTmp: u32 = uiNum;
    let mut pucBlock: Ptr<u8> = pucInput.cast();
    while (uiNumTmp.suffix_minus_minus() != 0).as_bool() {
        vosSha256CompressBlock(pstCtx.h.cast(), pucBlock.cast());
        pucBlock += SHA256_BLOCK_SIZE!();
    }
}

pub fn VOS_Sha256Calc(
    mut pucInput: Ptr<u8>,
    mut uiInputLen: u32,
    mut pucOutput: Ptr<u8>,
    mut uiOutputLen: u32,
) {
    let mut stCtx: VOS_SHA256_CTX = Default::default();
    vosSha256Begin(c_ref!(stCtx).cast());
    vosSha256Hash(pucInput.cast(), uiInputLen.cast(), c_ref!(stCtx).cast());
    vosSha256End(pucOutput.cast(), uiOutputLen.cast(), c_ref!(stCtx).cast());
}
