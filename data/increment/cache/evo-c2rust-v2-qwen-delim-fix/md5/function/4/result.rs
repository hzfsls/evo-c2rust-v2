pub fn VOS_MD5FinalEx(mut digest: Ptr<u8>, mut bufLen: u32, mut context: Ptr<MD5_CTX>) {
    let mut needAnotherBuff: bool = false;

    if (digest == NULL!()).as_bool() || (context == NULL!()).as_bool() || (bufLen < MD5_DIGEST_LEN!()).as_bool() {
        return;
    }

    needAnotherBuff = VOS_MD5PadBuff(context.cast()).cast();
    VOS_MD5CalcDigestOfBuff(context.cast());

    if needAnotherBuff.as_bool() {
        context.uiPos = 0;
        while (context.uiPos < MD5_TEXT_IN_BUFFER_MAX!()).as_bool() {
            context.aucBuffer[context.uiPos] = 0;
            context.uiPos += 1;
        }
        MD5_RECORD_MESSAGE_LEN!(context.cast());
        VOS_MD5CalcDigestOfBuff(context.cast());
    }

    MD5_COMPOSE_DIGEST!(digest.cast(), context.aulState.cast());

    c_memset_s!(context.cast::<Ptr<Void>>(), c_sizeof!(MD5_CTX).cast(), 0, c_sizeof!(MD5_CTX).cast()).cast::<Void>();
}