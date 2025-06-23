#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzDecParam {
    pub protData: Ptr<u8>,
    pub protSize: u32,
    pub memHook: Ptr<CmptLzMemHook>,
}
