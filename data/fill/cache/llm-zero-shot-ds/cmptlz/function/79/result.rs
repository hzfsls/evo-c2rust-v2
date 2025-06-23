fn pos_sloter(dist: u32) -> u32 {
    if dist <= 4 {
        return dist;
    }
    let helper = pos_slot_helper(dist);
    helper + helper + ((dist >> (helper - 1)) & 1)
}
