pub struct InDeComdata {
    pub input: *mut BzpStream,
    pub output: *mut BzpStream,
    pub las_char: i32,
    pub num: i32,
    pub buf: u32,
    pub n_buf: i32,
    pub block_size: i32,
    pub block_crc: u32,
    pub list: [i32; BZP_ASCII_SIZE],
}
