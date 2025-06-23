pub fn CmptlzDpPre(mut encCtx: Ptr<CmptLzEncCtx>, mut mainReps: Ptr<u32>, mut cur: u32) {
    let mut posPointer: u32 = encCtx.opts[cur].posPrev;
    let mut state: CmptlzState = encCtx.opts[posPointer].state;
    if (posPointer == cur - 1) {
        if (encCtx.opts[cur].backPrev == 0) {
            state = if state < 7 { LIT_SHORTREP!() } else { NOTLIT_REP!() };
        } else {
            state = if state < 7 { LIT_LIT!() } else { NOTLIT_REP!() };
        }
    } else {
        let mut backPointer: u32;
        backPointer = encCtx.opts[cur].backPrev;
        if (backPointer < CMPTLZ_NUM_REPS as u32) {
            state = if state < 7 { LIT_LONGREP!() } else { NOTLIT_REP!() };
        } else {
            state = if state < 7 { LIT_MATCH!() } else { NOTLIT_MATCH!() };
        }
        let mut i: u32;
        if (backPointer < CMPTLZ_NUM_REPS as u32) {
            mainReps[0] = encCtx.opts[posPointer].backs[backPointer as usize];
            c_for!(i = 1; i <= backPointer; i.suffix_plus_plus(); {
                mainReps[i as usize] = encCtx.opts[posPointer].backs[(i - 1) as usize];
            });
            c_for!(; i < CMPTLZ_NUM_REPS as u32; i.suffix_plus_plus(); {
                mainReps[i as usize] = encCtx.opts[posPointer].backs[i as usize];
            });
        } else {
            mainReps[0] = (backPointer - CMPTLZ_NUM_REPS as u32);
            c_for!(i = 1; i < CMPTLZ_NUM_REPS as u32; i.suffix_plus_plus(); {
                mainReps[i as usize] = encCtx.opts[posPointer].backs[(i - 1) as usize];
            });
        }
    }
    encCtx.opts[cur].state = state;
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS as u32; i.suffix_plus_plus(); {
        encCtx.opts[cur].backs[i as usize] = mainReps[i as usize];
    });
}
