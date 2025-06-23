pub fn RapidlzGetPosOnTable(mut hashValue: u32, mut hashTable: Ptr<u8>, mut hashType: u8) -> u32 {
    if (hashType == 4) {
        return (*((hashTable.cast::<Ptr<u16>>() + hashValue)).cast::<u32>());
    } else if (hashType == 5) {
        return (*((hashTable.cast::<Ptr<u32>>() + hashValue)).cast::<u32>());
    }
    return 0;
}