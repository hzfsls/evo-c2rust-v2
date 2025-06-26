pub fn BzpHuffmanWeightAdd(mut w1: i32, mut w2: i32) -> i32 {
    return (((w1 as u32 & 0xffffff00) + (w2 as u32 & 0xffffff00)) | (BZP_MAX_FUN!(w1 as u32 & 0x000000ff, w2 as u32 & 0x000000ff) + 1)).cast();
}