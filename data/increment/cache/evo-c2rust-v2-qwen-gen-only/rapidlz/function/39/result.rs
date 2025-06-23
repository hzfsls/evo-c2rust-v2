pub fn RapidlzPutPosOnTable(mut pos: u32, mut hashValue: u32, mut hashTable: Ptr<u8>, mut hashType: u8) {
    if (hashType == 4).as_bool() {
        *(((hashTable.cast::<Ptr<u16>>()) + hashValue).cast::<Ptr<u16>>()) = pos.cast::<u16>();
    } else if (hashType == 5).as_bool() {
        *(((hashTable.cast::<Ptr<u32>>()) + hashValue).cast::<Ptr<u32>>()) = pos.cast::<u32>();
    }
}