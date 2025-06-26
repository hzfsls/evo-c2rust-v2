pub fn RapidlzHash4PutPos(mut pos: u32, mut hashValue: u32, mut hashTable: Ptr<u32>) {
    hashTable[hashValue] = pos.cast();
}
