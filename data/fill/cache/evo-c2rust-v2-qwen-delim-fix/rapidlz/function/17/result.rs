pub fn RapidlzGetPosOnTable(mut hashValue: u32, mut hashTable: Ptr<u8>, mut hashType: u8) -> u32 {
    if (hashType == 4).as_bool() {
        return (*((hashTable.cast::<Ptr<u16>>() + hashValue).cast::<Ptr<u16>>())).cast::<u32>();
    } else if (hashType == 5).as_bool() {
        return (*((hashTable.cast::<Ptr<u32>>() + hashValue).cast::<Ptr<u32>>())).cast::<u32>();
    }
    return 0;
}