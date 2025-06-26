pub fn VOS_MD5CalcDigestOfBuff(mut context: Ptr<MD5_CTX>) {
    let mut i: u32 = Default::default();
    let mut tmpValue: u32 = Default::default();
    let mut textFragment: Array<u32, 16> = Default::default();
    let mut tmpState: Array<u32, 4> = Default::default();
    let mut tmpText: Ptr<u8> = context.aucBuffer.cast();
    tmpState[0] = context.aulState[0].cast();
    tmpState[1] = context.aulState[1].cast();
    tmpState[2] = context.aulState[2].cast();
    tmpState[3] = context.aulState[3].cast();
    c_for!(i = 0; i < 16; i += 4; {
        textFragment[i] = (tmpText[0].cast::<u32>() + (tmpText[1].cast::<u32>() << 8) + (tmpText[2].cast::<u32>() << 16) + (tmpText[3].cast::<u32>() << 24)).cast();
        textFragment[i + 1] = (tmpText[4].cast::<u32>() + (tmpText[5].cast::<u32>() << 8) + (tmpText[6].cast::<u32>() << 16) + (tmpText[7].cast::<u32>() << 24).cast();
        textFragment[i + 2] = (tmpText[8].cast::<u32>() + (tmpText[9].cast::<u32>() << 8) + (tmpText[10].cast::<u32>() << 16) + (tmpText[11].cast::<u32>() << 24).cast();
        textFragment[i + 3] = (tmpText[12].cast::<u32>() + (tmpText[13].cast::<u32>() << 8) + (tmpText[14].cast::<u32>() << 16) + (tmpText[15].cast::<u32>() << 24).cast();
        tmpText += 16;
    });
    MD5_F_PROC!(tmpValue, tmpState, textFragment);
    MD5_G_PROC!(tmpValue, tmpState, textFragment);
    MD5_H_PROC!(tmpValue, tmpState, textFragment);
    MD5_I_PROC!(tmpValue, tmpState, textFragment);
    context.aulState[0] += tmpState[0].cast();
    context.aulState[1] += tmpState[1].cast();
    context.aulState[2] += tmpState[2].cast();
    context.aulState[3] += tmpState[3].cast();
}
