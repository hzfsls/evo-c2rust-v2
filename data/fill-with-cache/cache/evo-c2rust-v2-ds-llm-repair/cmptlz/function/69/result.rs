pub fn CmptlzParamNormalize(mut props: Ptr<CmptlzEncParam>) {
    let mut level: i32 = props.level.cast();
    if (level < 0 || level > 9).as_bool() {
        level = 5;
    }
    props.level = level.cast();
    if (props.dictSize < CMPTLZ_MIN_DICTSIZE!() || props.dictSize > CMPTLZ_MAX_DICTSIZE!()).as_bool() {
        CMPTLZ_SET_DICTSIZE_BY_LEVEL!(level, props.dictSize);
    }
    if (props.fastBytes < 5 || props.fastBytes > CMPT_MF_LONGEST_MATCH!()).as_bool() {
        CMPTLZ_SET_FB_BY_LEVEL!(level, props.fastBytes);
    }
    if (props.litCtx < 0 || props.litCtx > CMPTLZ_LC_MAX!()).as_bool() {
        props.litCtx = 3;
    }
    if (props.litPos < 0 || props.litPos > CMPTLZ_LP_MAX!()).as_bool() {
        props.litPos = 0;
    }
    if (props.posBits < 0 || props.posBits > CMPTLZ_PB_MAX!()).as_bool() {
        props.posBits = 2;
    }
    props.numThreads = 1;
}
