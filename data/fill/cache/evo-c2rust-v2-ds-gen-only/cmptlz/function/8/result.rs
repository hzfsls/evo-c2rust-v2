pub fn CmptLzDecMemFree(mut memHook: Ptr<CmptLzMemHook>, mut memHandle: i32, mut freeAddress: Ptr<Void>) {
    memHook.CmptLzFree(memHandle.cast(), freeAddress.cast());
}
