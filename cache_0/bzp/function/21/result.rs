pub fn BzpSwap2Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = value.cast();
}
