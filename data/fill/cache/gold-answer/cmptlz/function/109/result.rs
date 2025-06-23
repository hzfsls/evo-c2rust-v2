pub fn CmptRcFlushData(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut i: i32;
    let mut res: i32 = Default::default();
    c_for!(i = 0; i < 5; i += 1; {
        res = CmptRcShiftLow(rcCtx);
        if res != CMPT_OK!() {
            break;
        }
    });
    return res;
}