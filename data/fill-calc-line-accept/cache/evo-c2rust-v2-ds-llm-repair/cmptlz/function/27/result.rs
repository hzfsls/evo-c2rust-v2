pub fn CmptLzGetBaseDistByPosSlot(mut posSlot: u32) -> u32 {
    return (2 | (posSlot & 1)).cast();
}
