#[derive(Default)]
#[repr(C)]
pub struct RapidlzCCtx {
    pub hashTable: Ptr<u8>,
    pub hashType: u8,
    pub hashBits: u8,
    pub step: u8,
    pub bufferLimit: u8,
}