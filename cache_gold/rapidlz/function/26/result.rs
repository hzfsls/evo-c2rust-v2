pub fn RapidlzCompressDefault(
    mut src: VoidPtr,
    mut dst: VoidPtr,
    mut srcSize: usize,
    mut dstSize: usize,
) -> usize {
    return RapidlzCompress(src, dst, srcSize, dstSize, 1);
}