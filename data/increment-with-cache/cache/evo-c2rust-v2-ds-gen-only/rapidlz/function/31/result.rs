pub fn RapidlzWildCopy32(mut srcPtr: Ptr<u8>, mut dstPtr: Ptr<u8>, mut dstEnd: Ptr<u8>) {
    let mut tmpDstPtr: Ptr<u8> = dstPtr.cast();
    let mut tmpSrcPtr: Ptr<u8> = srcPtr.cast();
    c_do!({
        RapidlzCopy32Byte(tmpDstPtr.cast(), tmpSrcPtr.cast());
        tmpDstPtr += 32;
        tmpSrcPtr += 32;
    } while tmpDstPtr < dstEnd);
}
