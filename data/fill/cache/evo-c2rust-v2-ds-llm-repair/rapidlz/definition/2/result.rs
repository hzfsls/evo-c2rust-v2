#[repr(C)]
#[derive(Default)]
pub struct TagRapidlzStreamCtx {
    pub hashTable: Array<u32, { RAPIDLZ_STREAM_HASH_SIZE!() }>,
    pub dict: Ptr<u8>,
    pub dictSize: u32,
    pub currentOffset: u32,
    pub acceleration: i32,
    pub strmCtxSpecific: Ptr<RapidlzStreamCtx>,
}
