pub fn CmptlzEncPrepare(mut encCtx: Ptr<CmptLzEncCtx>) {
    let mut i: u32 = Default::default();
    let mut j: u32 = Default::default();

    encCtx.encNeedFinish = false;
    encCtx.cmptlzResponse = 0;
    encCtx.nowpos64 = 0;

    encCtx.state = 0;
    encCtx.pbMask = (1 << encCtx.posBits) - 1;
    encCtx.lpMask = (0x100 << encCtx.litPos) - (0x100 >> encCtx.litCtx);
    encCtx.posMask = (1 << encCtx.posBits) - 1;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.reps[i] = 0;
    });

    encCtx.optsCurIndex = 0;
    encCtx.optEndIndex = 0;
    c_for!(i = 0; i < CMPT_DP_OPTMAX!(); i.suffix_plus_plus(); {
        encCtx.opts[i].price = CMPT_INFINITY_PRICE!();
    });

    c_for!(i = 0; i < CMPTLZ_NUM_STATES!(); i.suffix_plus_plus(); {
        c_for!(j = 0; j < CMPTLZ_NUM_PB_STATES_MAX!(); j.suffix_plus_plus(); {
            encCtx.isMatch[i][j] = CMPTLZ_PROB_INIT!();
            encCtx.isRep0Long[i][j] = CMPTLZ_PROB_INIT!();
        });
        encCtx.isRep[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG0[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG1[i] = CMPTLZ_PROB_INIT!();
        encCtx.isRepG2[i] = CMPTLZ_PROB_INIT!();
    });

    c_for!(i = 0; i < CMPTLZ_DIST_STATE_TOTAL!(); i.suffix_plus_plus(); {
        c_for!(j = 0; j < (1 << CMPTLZ_DIST_SLOT_BITS!()); j.suffix_plus_plus(); {
            encCtx.probDistSlot[i][j] = CMPTLZ_PROB_INIT!();
        });
    });
    c_for!(i = 0; i < CMPT_DIST_LIMIT_2!(); i.suffix_plus_plus(); {
        encCtx.probDistSpecial[i] = CMPTLZ_PROB_INIT!();
    });
    c_for!(i = 0; i < (1 << CMPTLZ_ALIGN_BITS!()); i.suffix_plus_plus(); {
        encCtx.probAlign[i] = CMPTLZ_PROB_INIT!();
    });

    encCtx.litMarcov.lcBits = encCtx.litCtx;
    encCtx.litMarcov.posMask = (1 << encCtx.litPos) - 1;

    c_for!(i = 0; i < (1 << CMPTLZ_LCLP_MAX!()); i.suffix_plus_plus(); {
        c_for!(j = 0; j < CMPTLZ_LIT_MAX_SIZE!(); j.suffix_plus_plus(); {
            encCtx.litMarcov.literal[i][j] = CMPTLZ_PROB_INIT!();
        });
    });

    c_for!(i = 0; i < (1 << CMPT_LEN_HIGH_BITS!()); i.suffix_plus_plus(); {
        encCtx.matchLenEncoder.high[i] = CMPTLZ_PROB_INIT!();
        encCtx.repLenEncoder.high[i] = CMPTLZ_PROB_INIT!();
        encCtx.matchLenEncoder.low[i] = CMPTLZ_PROB_INIT!();
        encCtx.repLenEncoder.low[i] = CMPTLZ_PROB_INIT!();
    });

    CmptlzPriceInit(encCtx.cast());

    encCtx.repLenEncoder.tableSize = encCtx.numFastBytes - 1;
    encCtx.matchLenEncoder.tableSize = encCtx.numFastBytes - 1;
    CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.matchLenEncoder).cast());
    CmptPriceGenLenTable(encCtx.cast(), c_ref!(encCtx.repLenEncoder).cast());
}
