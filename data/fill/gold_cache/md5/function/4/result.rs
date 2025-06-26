pub fn VOS_MD5FinalEx(mut digest: Ptr<u8>, mut bufLen: u32, mut context: Ptr<MD5_CTX>) {
    let mut needAnotherBuff: bool = false;
    if digest == NULL!() || context == NULL!() || bufLen < MD5_DIGEST_LEN!() {
        return;
    }
    needAnotherBuff = VOS_MD5PadBuff(context);
    VOS_MD5CalcDigestOfBuff(context);
    if needAnotherBuff {
        context.uiPos = 0;
        c_for!(context.uiPos = 0; context.uiPos < MD5_TEXT_IN_BUFFER_MAX!(); context.uiPos += 1; {
            index!(context.aucBuffer, context.uiPos, 0);
        });
        MD5_RECORD_MESSAGE_LEN!(context);
        VOS_MD5CalcDigestOfBuff(context);
    }
    MD5_COMPOSE_DIGEST!(digest, context.aulState);
    c_memset_s!(context, c_sizeof!(MD5_CTX), 0, c_sizeof!(MD5_CTX));
}