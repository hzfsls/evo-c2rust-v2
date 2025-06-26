use std::ptr;
use std::mem;

pub struct MD5_CTX {
    aulState: [u32; 4],
    // Assuming there might be other fields in MD5_CTX
    // but they're zeroed out in the initialization
}

pub fn vos_md5_init(context: &mut MD5_CTX) {
    if context as *const _ == ptr::null() {
        return;
    }

    // Zero out the entire structure
    *context = unsafe { mem::zeroed() };

    // Initialize the state with MD5 magic numbers
    context.aulState[0] = 0x67452301;
    context.aulState[1] = 0xefcdab89;
    context.aulState[2] = 0x98badcfe;
    context.aulState[3] = 0x10325476;
}
