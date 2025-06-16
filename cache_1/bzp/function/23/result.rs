pub fn BzpSelectMidVal(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut l: i32, mut r: i32) -> i32 {
    let mut mid: i32 = (l + r) >> 1;
    let mut vl: i32 = idx[sortBlock[l]].cast();
    let mut vmid: i32 = idx[sortBlock[mid]].cast();
    let mut vr: i32 = idx[sortBlock[r]].cast();
    if (vl > vr).as_bool() {
        let mut tmp: i32 = l.cast();
        l = r.cast();
        r = tmp.cast();
        vl = idx[sortBlock[l]].cast();
        vr = idx[sortBlock[r]].cast();
    }
    if (vmid <= vl).as_bool() {
        return vl.cast();
    } else if (vmid <= vr).as_bool() {
        return vmid.cast();
    } else {
        return vr.cast();
    }
}
