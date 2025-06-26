use std::ptr;

pub struct BzpHuffmanDecode {
    pub base: [u32; 256],
    pub perm: [u32; 256],
    pub limit: [u32; 256],
    pub select_cnt: u32,
    pub de_code_num: u32,
}

impl BzpHuffmanDecode {
    pub fn reset(&mut self) {
        // Zero out the arrays
        unsafe {
            ptr::write_bytes(self.base.as_mut_ptr(), 0, self.base.len());
            ptr::write_bytes(self.perm.as_mut_ptr(), 0, self.perm.len());
            ptr::write_bytes(self.limit.as_mut_ptr(), 0, self.limit.len());
        }
        
        // Reset counters
        self.select_cnt = 0;
        self.de_code_num = 0;
    }
}
