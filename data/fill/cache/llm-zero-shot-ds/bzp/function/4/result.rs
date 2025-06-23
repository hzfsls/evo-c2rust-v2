use std::io::Read;

const BZP_BITS8: u32 = 8;

struct Input {
    buf: [u8; std::mem::size_of::<u32>()],
    pos: usize,
    n_buf: usize,
    file_ptr: std::fs::File,
}

struct InDeComdata<'a> {
    buf: u32,
    n_buf: u32,
    input: &'a mut Input,
}

fn bzp_read_bits(n_bit: i32, in_data: &mut InDeComdata) -> u32 {
    let mut res = 0u32;
    let n_bit = n_bit as u32;
    
    while in_data.n_buf < n_bit {
        if in_data.input.n_buf == in_data.input.pos {
            in_data.input.n_buf = in_data.input.file_ptr
                .read(&mut in_data.input.buf)
                .expect("Failed to read from file");
            in_data.input.pos = 0;
        }
        let data = in_data.input.buf[in_data.input.pos] as u32;
        in_data.buf = (in_data.buf << BZP_BITS8) | data;
        in_data.input.pos += 1;
        in_data.n_buf += BZP_BITS8;
    }
    
    res = in_data.buf >> (in_data.n_buf - n_bit);
    res = res & ((1 << n_bit) - 1);
    in_data.n_buf -= n_bit;
    
    res
}
