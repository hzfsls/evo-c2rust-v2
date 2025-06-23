pub type CmptLzMemHook = CmptLzMemHookStruct;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzMemHookStruct {
    pub CmptLzAlloc: FuncPtr<fn(i32, usize) -> VoidPtr>,
    pub CmptLzFree: FuncPtr<fn(i32, VoidPtr)>,
}