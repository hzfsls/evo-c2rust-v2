#[derive(Default)]
#[repr(C)]
pub struct CmptLzMemHook {
    pub CmptLzAlloc: FuncPtr<fn(i32, usize) -> VoidPtr>,
    pub CmptLzFree: FuncPtr<fn(i32, VoidPtr)>,
}