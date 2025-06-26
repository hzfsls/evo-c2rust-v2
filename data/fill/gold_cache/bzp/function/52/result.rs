pub fn BzpSelectMidVal(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) -> i32 {
    let mut mid: i32 = (l + r) >> 1;
    let mut vl: i32 = idx[sortBlock[l]];
    let mut vmid: i32 = idx[sortBlock[mid]];
    let mut vr: i32 = idx[sortBlock[r]];
    if vl > vr {
        let mut tmp: i32 = l;
        l = r;
        r = tmp;
        vl = idx[sortBlock[l]];
        vr = idx[sortBlock[r]];
    }
    if vmid <= vl {
        return vl;
    } else if vmid <= vr {
        return vmid;
    } else {
        return vr;
    }
}