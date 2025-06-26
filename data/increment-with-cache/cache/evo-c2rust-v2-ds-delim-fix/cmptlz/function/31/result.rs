pub fn CmptRcFlushData(mut rcCtx: Ptr<CmptRcCtx>) -> i32 {
    let mut i: i32;
    let mut res: i32;
    c_for!(i = 0; i < 5; i.suffix_plus_plus(); {
        res = CmptRcShiftLow(rcCtx.cast()).cast();
        if (res != CMPT_OK!()).as_bool() {
            break;
        }
    });
    return res.cast();
}
