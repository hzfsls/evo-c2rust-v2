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

    c_memset_s!(context, c_sizeofval!(*context), 0, c_sizeofval!(*context)).cast::<Void>();
}
