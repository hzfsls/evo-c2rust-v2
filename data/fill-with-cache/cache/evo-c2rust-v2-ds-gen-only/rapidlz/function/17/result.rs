pub fn RapidlzGetPosOnTable(mut hashValue: u32, mut hashTable: Ptr<u8>, mut hashType: u8) -> u32 {
    if (hashType == 4).as_bool() {
        return (*((hashTable.cast::<Ptr<u16>>() + hashValue)).cast();
    } else if (hashType == 5).as_bool() {
        return (*((hashTable.cast::<Ptr<u32>>() + hashValue)).cast();
    }
    return 0;
}
