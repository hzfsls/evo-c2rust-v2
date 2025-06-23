pub struct MD5_CTX {
    pub aul_state: [u32; 4],
    pub aul_count: [u32; 2],
    pub auc_buffer: [u8; 64],
    pub ui_pos: u32,
}
