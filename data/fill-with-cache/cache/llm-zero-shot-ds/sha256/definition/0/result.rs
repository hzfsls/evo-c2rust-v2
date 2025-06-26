#[repr(C)]
pub struct VOS_SHA256_CTX {
    pub h: [u32; 8],
    pub N: [u32; 2],
    pub block: [u32; SHA256_BLOCK_SIZE / std::mem::size_of::<u32>()],
    pub blocklen: u32,
    pub outlen: u32,
    pub computed: u32,
    pub corrupted: u32,
}
