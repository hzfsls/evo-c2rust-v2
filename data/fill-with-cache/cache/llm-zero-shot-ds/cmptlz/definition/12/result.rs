pub struct CmptlzDecParam {
    pub prot_data: *const u8,
    pub prot_size: u32,
    pub mem_hook: *mut CmptLzMemHook,
}
