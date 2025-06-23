pub fn RapidlzCalcHashValue(mut srcCurr: Ptr<u8>, mut hashType: u8, mut hashBits: u8) -> u32 {
    if (hashType == 5).as_bool() {
        return (((RAPIDLZ_READ64BIT!(srcCurr) << 24) * 11400714819323198485u64) >> (64 - hashBits)).cast::<u32>();
    } else {
        return (RAPIDLZ_READ32BIT!(srcCurr) * 2654435769u32) >> (32 - hashBits);
    }
}
