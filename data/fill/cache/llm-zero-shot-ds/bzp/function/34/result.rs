use std::mem;

const BZP_ASCII_SIZE: usize = 256;
const BZP_CHARS_PER_GROUP_ASCII: usize = 16;
const BZP_GROUPS_ASCII: usize = BZP_ASCII_SIZE / BZP_CHARS_PER_GROUP_ASCII;
const BZP_BIT: usize = 1;

fn bzp_write_valid_ascii(out_data: &mut BzpOutComdata, bwt: &BzpBwtInfo) {
    let mut valid_gid = [0i32; BZP_ASCII_SIZE];
    let mut cnt = 0;
    let mut use16 = [false; BZP_GROUPS_ASCII];

    for i in 0..BZP_ASCII_SIZE {
        let gid = i / BZP_CHARS_PER_GROUP_ASCII;
        use16[gid] |= bwt.in_use[i];
    }

    for i in 0..BZP_GROUPS_ASCII {
        bzp_write_to_array(use16[i] as i32, BZP_BIT, out_data);
        if use16[i] {
            valid_gid[cnt] = i as i32;
            cnt += 1;
        }
    }

    for i in 0..cnt {
        for j in 0..BZP_CHARS_PER_GROUP_ASCII {
            let valid = valid_gid[i as usize] * BZP_CHARS_PER_GROUP_ASCII as i32 + j as i32;
            bzp_write_to_array(bwt.in_use[valid as usize] as i32, BZP_BIT, out_data);
        }
    }
}
