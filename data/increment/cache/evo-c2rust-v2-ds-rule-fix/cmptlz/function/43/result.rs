pub fn CmptMemCmpLen(mut buf1: Ptr<u8>, mut buf2: Ptr<u8>, mut len: u32, mut limit: u32) -> u32 {
    return CmptMemCmpByOneByte(buf1.cast(), buf2.cast(), len.cast(), limit.cast()).cast();
}
