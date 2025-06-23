pub fn RapidlzWildCopy16(mut srcPtr: Ptr<u8>, mut dstPtr: Ptr<u8>, mut dstEnd: Ptr<u8>) {
    let mut tmpDstPtr: Ptr<u8> = dstPtr;
    let mut tmpSrcPtr: Ptr<u8> = srcPtr;
    loop {
        RapidlzCopy16Byte(tmpDstPtr, tmpSrcPtr);
        tmpDstPtr = tmpDstPtr + 16;
        tmpSrcPtr = tmpSrcPtr + 16;
        if tmpDstPtr >= dstEnd {
            break;
        }
    }
}