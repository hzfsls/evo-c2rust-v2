use std::mem;

const CMPTLZ_NUM_REPS: usize = 4;

#[derive(Debug, Clone, Copy)]
struct CmptlzState(u32);

#[derive(Debug)]
struct CmptLzEncCtx {
    opts: Vec<Opt>,
}

#[derive(Debug)]
struct Opt {
    posPrev: u32,
    backPrev: u32,
    state: CmptlzState,
    backs: [u32; CMPTLZ_NUM_REPS],
}

macro_rules! CMPT_STATE_UPDATE_WHEN_SHORTREP {
    ($state:expr) => {
        $state.0 = ($state.0).wrapping_add(1);
    };
}

macro_rules! CMPT_STATE_UPDATE_WHEN_LIT {
    ($state:expr) => {
        $state.0 = ($state.0).wrapping_add(2);
    };
}

macro_rules! CMPT_STATE_UPDATE_WHEN_LONGREP {
    ($state:expr) => {
        $state.0 = ($state.0).wrapping_add(3);
    };
}

macro_rules! CMPT_STATE_UPDATE_WHEN_MATCH {
    ($state:expr) => {
        $state.0 = ($state.0).wrapping_add(4);
    };
}

fn cmptlz_dp_pre(enc_ctx: &mut CmptLzEncCtx, main_reps: &mut [u32; CMPTLZ_NUM_REPS], cur: u32) {
    let pos_pointer = enc_ctx.opts[cur as usize].posPrev;
    let mut state = enc_ctx.opts[pos_pointer as usize].state;

    if pos_pointer == cur - 1 {
        if enc_ctx.opts[cur as usize].backPrev == 0 {
            CMPT_STATE_UPDATE_WHEN_SHORTREP!(state);
        } else {
            CMPT_STATE_UPDATE_WHEN_LIT!(state);
        }
    } else {
        let back_pointer = enc_ctx.opts[cur as usize].backPrev;

        if back_pointer < CMPTLZ_NUM_REPS as u32 {
            CMPT_STATE_UPDATE_WHEN_LONGREP!(state);
        } else {
            CMPT_STATE_UPDATE_WHEN_MATCH!(state);
        }

        if back_pointer < CMPTLZ_NUM_REPS as u32 {
            main_reps[0] = enc_ctx.opts[pos_pointer as usize].backs[back_pointer as usize];

            for i in 1..=back_pointer as usize {
                main_reps[i] = enc_ctx.opts[pos_pointer as usize].backs[i - 1];
            }
            for i in (back_pointer as usize + 1)..CMPTLZ_NUM_REPS {
                main_reps[i] = enc_ctx.opts[pos_pointer as usize].backs[i];
            }
        } else {
            main_reps[0] = back_pointer - CMPTLZ_NUM_REPS as u32;
            for i in 1..CMPTLZ_NUM_REPS {
                main_reps[i] = enc_ctx.opts[pos_pointer as usize].backs[i - 1];
            }
        }
    }

    enc_ctx.opts[cur as usize].state = state;

    for i in 0..CMPTLZ_NUM_REPS {
        enc_ctx.opts[cur as usize].backs[i] = main_reps[i];
    }
}
