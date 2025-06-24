pub fn RapidlzCalcHashValue(mut srcCurr: Ptr<u8>, mut hashType: u8, mut hashBits: u8) -> u32 {
    if hashType == 5 {
        return (((RAPIDLZ_READ64BIT!(srcCurr) << 24) * 11400714819323198485) >> (64 - hashBits))
            .cast();
    } else {
        return (RAPIDLZ_READ32BIT!(srcCurr) * 2654435769) >> (32 - hashBits);
    }
}