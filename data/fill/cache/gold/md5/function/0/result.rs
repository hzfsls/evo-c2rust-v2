pub fn VOS_MD5CalcDigestOfBuff(mut context: Ptr<MD5_CTX>) {
    let mut i: u32;
    let mut tmpValue: u32;
    let mut textFragment: Array<u32, 16> = Default::default();
    let mut tmpState: Array<u32, 4> = Default::default();
    let mut tmpText: Ptr<u8> = context.aucBuffer.cast();
    tmpState[0] = context.aulState[0];
    tmpState[1] = context.aulState[1];
    tmpState[2] = context.aulState[2];
    tmpState[3] = context.aulState[3];
    c_for!(i = 0; i < 16; i += 4; {
        textFragment[i] = (tmpText[0] as u32) + ((tmpText[1] as u32) << 8) + ((tmpText[2] as u32) << 16) + ((tmpText[3] as u32) << 24);
        textFragment[i + 1] = (tmpText[4] as u32) + ((tmpText[5] as u32) << 8) + ((tmpText[6] as u32) << 16) + ((tmpText[7] as u32) << 24);
        textFragment[i + 2] = (tmpText[8] as u32) + ((tmpText[9] as u32) << 8) + ((tmpText[10] as u32) << 16) + ((tmpText[11] as u32) << 24);
        textFragment[i + 3] = (tmpText[12] as u32) + ((tmpText[13] as u32) << 8) + ((tmpText[14] as u32) << 16) + ((tmpText[15] as u32) << 24);
        tmpText = tmpText + 16;
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