pub fn BzpSwap3Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut ePos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos].cast();
    sortBlock[lPos] = sortBlock[rPos].cast();
    sortBlock[rPos] = sortBlock[ePos].cast();
    sortBlock[ePos] = value.cast();
}
