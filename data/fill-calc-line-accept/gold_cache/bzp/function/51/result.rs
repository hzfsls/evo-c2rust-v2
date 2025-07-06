pub fn BzpSwap3Elem(mut sortBlock: Ptr<i32>, mut lPos: i32, mut ePos: i32, mut rPos: i32) {
    let mut value: i32 = sortBlock[lPos];
    sortBlock[lPos] = sortBlock[rPos];
    sortBlock[rPos] = sortBlock[ePos];
    sortBlock[ePos] = value;
}