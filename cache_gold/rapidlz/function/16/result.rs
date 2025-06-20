pub fn RapidlzPutPosOnTable(
    mut pos: u32,
    mut hashValue: u32,
    mut hashTable: Ptr<u8>,
    mut hashType: u8,
) {
    if hashType == 4 {
        *(hashTable.cast::<Ptr<u16>>() + hashValue) = pos.cast();
    } else if hashType == 5 {
        *(hashTable.cast::<Ptr<u32>>() + hashValue) = pos.cast();
    }
}