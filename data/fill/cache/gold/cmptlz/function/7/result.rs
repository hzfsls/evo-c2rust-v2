pub fn CmptLzDecMemAlloc(mut memHook: Ptr<CmptLzMemHook>, mut memHandle: i32, mut allocSize: usize) -> VoidPtr {
    return (memHook.CmptLzAlloc)(memHandle, allocSize);
}