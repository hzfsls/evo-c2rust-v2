use std::mem;

pub struct MD5_CTX {
    pub aul_state: [u32; 4],
    // Assuming other fields are present in the actual MD5_CTX struct
    // but not shown in the provided C code
}

pub fn vos_md5_init(context: &mut MD5_CTX) {
    if context.is_null() {
        return;
    }
    
    // Zero out the entire context
    unsafe {
        std::ptr::write_bytes(context, 0, 1);
    }
    
    // Initialize the state with MD5 magic numbers
    context.aul_state[0] = 0x67452301;
    context.aul_state[1] = 0xefcdab89;
    context.aul_state[2] = 0x98badcfe;
    context.aul_state[3] = 0x10325476;
}
