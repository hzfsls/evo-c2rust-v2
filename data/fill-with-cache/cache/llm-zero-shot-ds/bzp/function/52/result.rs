pub fn bzp_select_mid_val(sort_block: &[i32], idx: &[i32], l: i32, r: i32) -> i32 {
    let mid = (l + r) >> 1;
    let vl = idx[sort_block[l as usize] as usize];
    let vmid = idx[sort_block[mid as usize] as usize];
    let vr = idx[sort_block[r as usize] as usize];
    
    let (mut l, mut vl, mut r, mut vr) = if vl > vr {
        (r, vr, l, vl)
    } else {
        (l, vl, r, vr)
    };
    
    if vmid <= vl {
        vl
    } else if vmid <= vr {
        vmid
    } else {
        vr
    }
}
