use crate::translation_utils::*;

#[repr(C)]
#[derive(Default)]
pub struct TagMd5Ctx {
    pub aulState: Array<u32, 4>,
    pub aulCount: Array<u32, 2>,
    pub aucBuffer: Array<u8, 64>,
    pub uiPos: u32,
}

pub type MD5_CTX = TagMd5Ctx;

macro_rules! MD5_DIGEST_LEN {
    () => {
        16
    };
}
pub(crate) use MD5_DIGEST_LEN;

macro_rules! MD5_INPUT_LEN_MAX {
    () => {
        0xffffffffffffffff
    };
}
pub(crate) use MD5_INPUT_LEN_MAX;

macro_rules! MD5_BUFFER_SIZE {
    () => {
        64
    };
}
pub(crate) use MD5_BUFFER_SIZE;

macro_rules! MD5_TEXT_IN_BUFFER_MAX {
    () => {
        56
    };
}
pub(crate) use MD5_TEXT_IN_BUFFER_MAX;

macro_rules! MD5_LINEAR_FUNC_F {
    ($B:expr, $C:expr, $D:expr) => {
        ($B & $C) | ((!$B) & $D)
    };
}
pub(crate) use MD5_LINEAR_FUNC_F;

macro_rules! MD5_LINEAR_FUNC_G {
    ($B:expr, $C:expr, $D:expr) => {
        ($B & $D) | ($C & !$D)
    };
}
pub(crate) use MD5_LINEAR_FUNC_G;

macro_rules! MD5_LINEAR_FUNC_H {
    ($B:expr, $C:expr, $D:expr) => {
        $B ^ $C ^ $D
    };
}
pub(crate) use MD5_LINEAR_FUNC_H;

macro_rules! MD5_LINEAR_FUNC_I {
    ($B:expr, $C:expr, $D:expr) => {
        $C ^ ($B | !$D)
    };
}
pub(crate) use MD5_LINEAR_FUNC_I;

macro_rules! MD5_RECORD_MESSAGE_LEN { ($context:expr) =>
    {
        let mut __i: u32;
        c_for!(__i = 0; __i < (c_sizeofval!($context.aulCount) / c_sizeofval!($context.aulCount[0])).cast(); __i.plus_plus(); {
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = ($context.aulCount[__i] & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 8) & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 16) & 0xff) as u8;
            $context.uiPos += 1;
            let idx = $context.uiPos; // bypass the borrow checker
            $context.aucBuffer[idx] = (($context.aulCount[__i] >> 24) & 0xff) as u8;
            $context.uiPos += 1;
        });
    }
}
pub(crate) use MD5_RECORD_MESSAGE_LEN;

macro_rules! MD5_COMPOSE_DIGEST { ($digest:expr, $md5State:expr) =>
    {
        let mut __i: u32 = 0;
        let mut __j: u32 = 0;
        c_for!(; __i < $md5State.len().cast(); __i.plus_plus(); {
            $digest[__j as usize] = $md5State[__i as usize] as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 8) as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 16) as u8;
            __j.plus_plus();
            $digest[__j as usize] = ($md5State[__i as usize] >> 24) as u8;
            __j.plus_plus();
        });
    }
}
pub(crate) use MD5_COMPOSE_DIGEST;

macro_rules! MD5_CYCLE_MOVE {
    ($numMoved:expr, $moveBit:expr) => {
        let mut __tmpValue: u32;
        __tmpValue = $numMoved >> (32 - $moveBit);
        $numMoved = $numMoved << $moveBit;
        $numMoved += __tmpValue;
    };
}
pub(crate) use MD5_CYCLE_MOVE;

macro_rules! MD5_CHANGE_STATE_IN_TURN {
    ($state:expr, $value:expr) => {
        $state[0] = $state[3];
        $state[3] = $state[2];
        $state[2] = $state[1];
        $state[1] = $state[1] + $value;
    };
}
pub(crate) use MD5_CHANGE_STATE_IN_TURN;

macro_rules! MD5_FUNC_F {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        $value = MD5_LINEAR_FUNC_F!($md5State[1], $md5State[2], $md5State[3])
            + $md5State[0]
            + $text
            + $addEnd;
        MD5_CYCLE_MOVE!($value, $moveBit);
        MD5_CHANGE_STATE_IN_TURN!($md5State, $value);
    };
}
pub(crate) use MD5_FUNC_F;

macro_rules! MD5_FUNC_G {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        $value = MD5_LINEAR_FUNC_G!($md5State[1], $md5State[2], $md5State[3])
            + $md5State[0]
            + $text
            + $addEnd;
        MD5_CYCLE_MOVE!($value, $moveBit);
        MD5_CHANGE_STATE_IN_TURN!($md5State, $value);
    };
}
pub(crate) use MD5_FUNC_G;

macro_rules! MD5_FUNC_H {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        $value = MD5_LINEAR_FUNC_H!($md5State[1], $md5State[2], $md5State[3])
            + $md5State[0]
            + $text
            + $addEnd;
        MD5_CYCLE_MOVE!($value, $moveBit);
        MD5_CHANGE_STATE_IN_TURN!($md5State, $value);
    };
}
pub(crate) use MD5_FUNC_H;

macro_rules! MD5_FUNC_I {
    ($value:expr, $md5State:expr, $text:expr, $addEnd:expr, $moveBit:expr) => {
        $value = MD5_LINEAR_FUNC_I!($md5State[1], $md5State[2], $md5State[3])
            + $md5State[0]
            + $text
            + $addEnd;
        MD5_CYCLE_MOVE!($value, $moveBit);
        MD5_CHANGE_STATE_IN_TURN!($md5State, $value);
    };
}
pub(crate) use MD5_FUNC_I;

macro_rules! MD5_F_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[0], 0xd76aa478, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[1], 0xe8c7b756, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[2], 0x242070db, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[3], 0xc1bdceee, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[4], 0xf57c0faf, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[5], 0x4787c62a, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[6], 0xa8304613, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[7], 0xfd469501, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[8], 0x698098d8, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[9], 0x8b44f7af, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[10], 0xffff5bb1, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[11], 0x895cd7be, 22);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[12], 0x6b901122, 7);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[13], 0xfd987193, 12);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[14], 0xa679438e, 17);
        MD5_FUNC_F!($tmpValue, $tmpState, $textFragment[15], 0x49b40821, 22);
    };
}
pub(crate) use MD5_F_PROC;

macro_rules! MD5_G_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[1], 0xf61e2562, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[6], 0xc040b340, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[11], 0x265e5a51, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[0], 0xe9b6c7aa, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[5], 0xd62f105d, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[10], 0x02441453, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[15], 0xd8a1e681, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[4], 0xe7d3fbc8, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[9], 0x21e1cde6, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[14], 0xc33707d6, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[3], 0xf4d50d87, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[8], 0x455a14ed, 20);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[13], 0xa9e3e905, 5);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[2], 0xfcefa3f8, 9);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[7], 0x676f02d9, 14);
        MD5_FUNC_G!($tmpValue, $tmpState, $textFragment[12], 0x8d2a4c8a, 20);
    };
}
pub(crate) use MD5_G_PROC;

macro_rules! MD5_H_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[5], 0xfffa3942, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[8], 0x8771f681, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[11], 0x6d9d6122, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[14], 0xfde5380c, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[1], 0xa4beea44, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[4], 0x4bdecfa9, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[7], 0xf6bb4b60, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[10], 0xbebfbc70, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[13], 0x289b7ec6, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[0], 0xeaa127fa, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[3], 0xd4ef3085, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[6], 0x04881d05, 23);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[9], 0xd9d4d039, 4);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[12], 0xe6db99e5, 11);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[15], 0x1fa27cf8, 16);
        MD5_FUNC_H!($tmpValue, $tmpState, $textFragment[2], 0xc4ac5665, 23);
    };
}
pub(crate) use MD5_H_PROC;

macro_rules! MD5_I_PROC {
    ($tmpValue:expr, $tmpState:expr, $textFragment:expr) => {
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[0], 0xf4292244, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[7], 0x432aff97, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[14], 0xab9423a7, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[5], 0xfc93a039, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[12], 0x655b59c3, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[3], 0x8f0ccc92, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[10], 0xffeff47d, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[1], 0x85845dd1, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[8], 0x6fa87e4f, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[15], 0xfe2ce6e0, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[6], 0xa3014314, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[13], 0x4e0811a1, 21);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[4], 0xf7537e82, 6);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[11], 0xbd3af235, 10);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[2], 0x2ad7d2bb, 15);
        MD5_FUNC_I!($tmpValue, $tmpState, $textFragment[9], 0xeb86d391, 21);
    };
}
pub(crate) use MD5_I_PROC;

pub fn VOS_MD5CalcDigestOfBuff(mut context: Ptr<MD5_CTX>) {
    let mut i: u32 = Default::default();
    let mut tmpValue: u32 = Default::default();
    let mut textFragment: Array<u32, 16> = Default::default();
    let mut tmpState: Array<u32, 4> = Default::default();
    let mut tmpText: Ptr<u8> = context.aucBuffer.cast();
    tmpState[0] = context.aulState[0];
    tmpState[1] = context.aulState[1];
    tmpState[2] = context.aulState[2];
    tmpState[3] = context.aulState[3];
    c_for!(i = 0; i < 16; i += 4; {
        textFragment[i] = (tmpText[0].cast::<u32>() + (tmpText[1].cast::<u32>() << 8) + (tmpText[2].cast::<u32>() << 16) + (tmpText[3].cast::<u32>() << 24));
        textFragment[i + 1] = (tmpText[4].cast::<u32>() + (tmpText[5].cast::<u32>() << 8) + (tmpText[6].cast::<u32>() << 16) + (tmpText[7].cast::<u32>() << 24));
        textFragment[i + 2] = (tmpText[8].cast::<u32>() + (tmpText[9].cast::<u32>() << 8) + (tmpText[10].cast::<u32>() << 16) + (tmpText[11].cast::<u32>() << 24));
        textFragment[i + 3] = (tmpText[12].cast::<u32>() + (tmpText[13].cast::<u32>() << 8) + (tmpText[14].cast::<u32>() << 16) + (tmpText[15].cast::<u32>() << 24));
        tmpText += 16;
    });
    MD5_F_PROC!(tmpValue, tmpState, textFragment);
    MD5_G_PROC!(tmpValue, tmpState, textFragment);
    MD5_H_PROC!(tmpValue, tmpState, textFragment);
    MD5_I_PROC!(tmpValue, tmpState, textFragment);
    context.aulState[0] += tmpState[0];
    context.aulState[1] += tmpState[1];
    context.aulState[2] += tmpState[2];
    context.aulState[3] += tmpState[3];
}

pub fn VOS_MD5PadBuff(mut context: Ptr<MD5_CTX>) -> bool {
    let mut needAnotherBuff: bool = (context.uiPos >= MD5_TEXT_IN_BUFFER_MAX!());
    let tmp0 = context.uiPos;
    context.aucBuffer[tmp0] = 0x80;
    context.uiPos += 1;
    if needAnotherBuff {
        while (context.uiPos < MD5_BUFFER_SIZE!()) {
            let tmp0 = context.uiPos;
            context.aucBuffer[tmp0] = 0;
            context.uiPos += 1;
        }
    } else {
        while (context.uiPos < MD5_TEXT_IN_BUFFER_MAX!()) {
            let tmp0 = context.uiPos;
            context.aucBuffer[tmp0] = 0;
            context.uiPos += 1;
        }
        MD5_RECORD_MESSAGE_LEN!(context);
    }
    return needAnotherBuff;
}

pub fn VOS_MD5Init(mut context: Ptr<MD5_CTX>) {
    if (context == NULL!()).as_bool() {
        return;
    }
    c_memset_s!(context, c_sizeof!(MD5_CTX), 0, c_sizeof!(MD5_CTX)).cast::<Void>();
    context.aulState[0] = 0x67452301;
    context.aulState[1] = 0xefcdab89;
    context.aulState[2] = 0x98badcfe;
    context.aulState[3] = 0x10325476;
}

pub fn VOS_MD5Update(mut context: Ptr<MD5_CTX>, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut totalInputBits: u64;
    let mut inputIndex: u32 = 0;
    let mut inputBit: u64;
    let mut tmpPos: u32;
    let mut contextBuffer: Ptr<u8> = NULL!();
    if (context == NULL!()).as_bool() || ((input == NULL!()).as_bool() && (inputLen != 0).as_bool())
    {
        return;
    }
    inputBit = inputLen.cast::<u64>() << 3;
    totalInputBits =
        ((context.aulCount[1].cast::<u64>()) << 32) + context.aulCount[0].cast::<u64>();
    if ((MD5_INPUT_LEN_MAX!() - totalInputBits) < inputBit) {
        return;
    }
    totalInputBits += inputBit;
    context.aulCount[0] = totalInputBits.cast::<u32>();
    context.aulCount[1] = (totalInputBits >> 32).cast::<u32>();
    tmpPos = context.uiPos;
    contextBuffer = context.aucBuffer.cast();
    while (inputIndex < inputLen).as_bool() {
        if (tmpPos < MD5_BUFFER_SIZE!()).as_bool() {
            contextBuffer[tmpPos] = input[inputIndex].cast();
            inputIndex += 1;
            tmpPos += 1;
            continue;
        }
        VOS_MD5CalcDigestOfBuff(context.cast());
        tmpPos = 0;
    }
    if (tmpPos == MD5_BUFFER_SIZE!()).as_bool() {
        VOS_MD5CalcDigestOfBuff(context.cast());
        tmpPos = 0;
    }
    context.uiPos = tmpPos;
    return;
}

pub fn VOS_MD5FinalEx(mut digest: Ptr<u8>, mut bufLen: u32, mut context: Ptr<MD5_CTX>) {
    let mut needAnotherBuff: bool = false;
    if (digest == NULL!()) || (context == NULL!()) || (bufLen < MD5_DIGEST_LEN!()) {
        return;
    }
    needAnotherBuff = VOS_MD5PadBuff(context);
    VOS_MD5CalcDigestOfBuff(context);
    if needAnotherBuff {
        context.uiPos = 0;
        while (context.uiPos < MD5_TEXT_IN_BUFFER_MAX!()) {
            let tmp0 = context.uiPos;
            context.aucBuffer[tmp0] = 0;
            context.uiPos += 1;
        }
        MD5_RECORD_MESSAGE_LEN!(context);
        VOS_MD5CalcDigestOfBuff(context);
    }
    MD5_COMPOSE_DIGEST!(digest, context.aulState);
    c_memset_s!(context, c_sizeof!(MD5_CTX), 0, c_sizeof!(MD5_CTX)).cast::<Void>();
}

pub fn VOS_MD5Final(mut digest: Ptr<u8>, mut context: Ptr<MD5_CTX>) {
    VOS_MD5FinalEx(digest.cast(), MD5_DIGEST_LEN!(), context.cast());
}

pub fn VOS_MD5CalcEx(
    mut output: Ptr<u8>,
    mut outputLen: u32,
    mut input: Ptr<u8>,
    mut inputLen: u32,
) {
    let mut context: MD5_CTX = Default::default();
    if (outputLen < MD5_DIGEST_LEN!()).as_bool() {
        return;
    }
    VOS_MD5Init(c_ref!(context).cast());
    VOS_MD5Update(
        c_ref!(context).cast(),
        input.cast::<Ptr<u8>>(),
        inputLen.cast(),
    );
    VOS_MD5FinalEx(output.cast(), outputLen.cast(), c_ref!(context).cast());
}

pub fn VOS_MD5Calc(mut output: Ptr<u8>, mut input: Ptr<u8>, mut inputLen: u32) {
    VOS_MD5CalcEx(
        output.cast(),
        MD5_DIGEST_LEN!(),
        input.cast(),
        inputLen.cast(),
    );
}
