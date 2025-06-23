pub fn PosSloter(mut dist: u32) -> u32 {
    if (dist <= 4).as_bool() {
        return dist.cast();
    }
    let mut helper: u32 = PosSlotHelper(dist.cast()).cast();
    return (helper + helper + ((dist >> (helper - 1)) & 1)).cast();
}
