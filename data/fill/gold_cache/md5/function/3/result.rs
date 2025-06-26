pub fn VOS_MD5Update(mut context: Ptr<MD5_CTX>, mut input: Ptr<u8>, mut inputLen: u32) {
    let mut totalInputBits: u64;
    let mut inputIndex: u32 = 0;
    let mut inputBit: u64;
    let mut tmpPos: u32;
    let mut contextBuffer: Ptr<u8> = NULL!();
    if context == NULL!() || (input == NULL!() && inputLen != 0) {
        return;
    }
    inputBit = inputLen.cast::<u64>() << 3;
    totalInputBits = (context.aulCount[1].cast::<u64>() << 32) + context.aulCount[0].cast::<u64>();
    if (MD5_INPUT_LEN_MAX!() - totalInputBits) < inputBit {
        return;
    }
    totalInputBits += inputBit;
    context.aulCount[0] = totalInputBits.cast();
    context.aulCount[1] = (totalInputBits >> 32).cast();
    tmpPos = context.uiPos;
    contextBuffer = context.aucBuffer.cast();
    while inputIndex < inputLen {
        if tmpPos < MD5_BUFFER_SIZE!() {
            contextBuffer[tmpPos] = input[inputIndex];
            inputIndex += 1;
            tmpPos += 1;
            continue;
        }
        VOS_MD5CalcDigestOfBuff(context);
        tmpPos = 0;
    }
    if tmpPos == MD5_BUFFER_SIZE!() {
        VOS_MD5CalcDigestOfBuff(context);
        tmpPos = 0;
    }
    context.uiPos = tmpPos;
    return;
}