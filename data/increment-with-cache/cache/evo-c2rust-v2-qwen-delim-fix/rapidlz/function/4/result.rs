pub fn RapidlzHash4CalcValue(mut curSrc: Ptr<u8>) -> u32 {
    return (RAPIDLZ_READ32BIT!(curSrc).cast::<u32>() * RAPIDLZ_GOLDEN_SECTION_PRIME!().cast::<u32>()) >> RAPIDLZ_STREAM_HASH_BITS!();
}