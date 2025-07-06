pub fn RapidlzCompressDefault(mut src: Ptr<Void>, mut dst: Ptr<Void>, mut srcSize: usize, mut dstSize: usize) -> usize {
    return RapidlzCompress(src.cast(), dst.cast(), srcSize.cast(), dstSize.cast(), 1).cast();
}
