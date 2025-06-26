pub fn bzp_select_mid_val(sort_block: &[i32], idx: &[i32], l: usize, r: usize) -> i32 {
    let mid = (l + r) >> 1;
    let mut vl = idx[sort_block[l] as usize];
    let vmid = idx[sort_block[mid] as usize];
    let mut vr = idx[sort_block[r] as usize];
    
    if vl > vr {
        std::mem::swap(&mut vl, &mut vr);
    }
    
    if vmid <= vl {
        vl
    } else if vmid <= vr {
        vmid
    } else {
        vr
    }
}
