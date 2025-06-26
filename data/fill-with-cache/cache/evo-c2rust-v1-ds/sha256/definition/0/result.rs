#[repr(C)]
#[derive(Default)]
pub struct VOS_SHA256_CTX {
    pub h: Array<u32, 8>,
    pub N: Array<u32, 2>,
    pub block: Array<u32, { SHA256_BLOCK_SIZE!() / core::mem::size_of::<u32>() }>,
    pub blocklen: u32,
    pub outlen: u32,
    pub computed: u32,
    pub corrupted: u32,
}
