pub fn CmptLzDecMemAlloc(mut memHook: Ptr<CmptLzMemHook>, mut memHandle: i32, mut allocSize: usize) -> Ptr<Void> {
    return memHook.CmptLzAlloc(memHandle.cast(), allocSize.cast());
}
