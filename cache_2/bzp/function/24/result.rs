pub fn BzpQSortSingle(mut sortBlock: Ptr<i32>, mut idx: Ptr<i32>, mut stack: Ptr<BzpQSortInfo>) {
    let mut tl: i32 = stack.tl;
    let mut tr: i32 = stack.tr;
    let mut value: i32 = BzpSelectMidVal(sortBlock, idx, tl, tr);
    let mut lPos: i32 = tl;
    let mut rPos: i32 = tr;
    let mut ePos: i32 = tl;

    while (ePos <= rPos) {
        if (idx[sortBlock[ePos]] < value) {
            BzpSwap2Elem(sortBlock, ePos, lPos);
            ePos += 1;
            lPos += 1;
        } else if (idx[sortBlock[ePos]] == value) {
            ePos += 1;
        } else {
            while (rPos >= ePos) && (idx[sortBlock[rPos]] > value) {
                rPos -= 1;
            }
            if (rPos < ePos) {
                break;
            }
            if (idx[sortBlock[rPos]] == value) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
            } else if (lPos == ePos) {
                BzpSwap2Elem(sortBlock, ePos, rPos);
                lPos += 1;
            } else {
                BzpSwap3Elem(sortBlock, lPos, ePos, rPos);
                lPos += 1;
            }
            ePos += 1;
            rPos -= 1;
        }
    }

    if (lPos - tl > tr - rPos) {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
    } else {
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = (rPos + 1);
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = tr;
        stack.cnt += 1;
        let tmp0 = stack.cnt;
        stack.stackL[tmp0] = tl;
        let tmp0 = stack.cnt;
        stack.stackR[tmp0] = (lPos - 1);
        stack.cnt += 1;
    }
}
