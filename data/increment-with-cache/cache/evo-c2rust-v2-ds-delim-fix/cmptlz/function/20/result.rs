pub fn CmptlzDpPre(mut encCtx: Ptr<CmptLzEncCtx>, mut mainReps: Ptr<u32>, mut cur: u32) {
    let mut posPointer: u32 = encCtx.opts[cur].posPrev.cast();
    let mut state: CmptlzState = encCtx.opts[posPointer].state.cast();
    if (posPointer == cur - 1).as_bool() {
        if (encCtx.opts[cur].backPrev == 0).as_bool() {
            CMPT_STATE_UPDATE_WHEN_SHORTREP!(state);
        } else {
            CMPT_STATE_UPDATE_WHEN_LIT!(state);
        }
    } else {
        let mut backPointer: u32;
        backPointer = encCtx.opts[cur].backPrev.cast();
        if (backPointer < CMPTLZ_NUM_REPS!()).as_bool() {
            CMPT_STATE_UPDATE_WHEN_LONGREP!(state);
        } else {
            CMPT_STATE_UPDATE_WHEN_MATCH!(state);
        }
        let mut i: u32;
        if (backPointer < CMPTLZ_NUM_REPS!()).as_bool() {
            mainReps[0] = encCtx.opts[posPointer].backs[backPointer].cast();
            c_for!(i = 1; i <= backPointer; i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i - 1].cast();
            });
            c_for!(; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i].cast();
            });
        } else {
            mainReps[0] = (backPointer - CMPTLZ_NUM_REPS!()).cast();
            c_for!(i = 1; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
                mainReps[i] = encCtx.opts[posPointer].backs[i - 1].cast();
            });
        }
    }
    encCtx.opts[cur].state = state.cast();
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.opts[cur].backs[i] = mainReps[i].cast();
    });
}
