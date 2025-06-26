struct BzpQSortInfo {
    cnt: i32,
    stack_l: Vec<i32>,
    stack_r: Vec<i32>,
    tl: i32,
    tr: i32,
}

impl BzpQSortInfo {
    fn new() -> Self {
        BzpQSortInfo {
            cnt: 0,
            stack_l: Vec::new(),
            stack_r: Vec::new(),
            tl: 0,
            tr: 0,
        }
    }
}

const BZP_THRESHOLD_SHELL_SORT: i32 = 10; // Assuming a value; adjust as needed

fn bzp_quick_sort(sort_block: &mut [i32], idx: &mut [i32], l: i32, r: i32) {
    let mut stack = BzpQSortInfo::new();
    stack.stack_l.push(l);
    stack.stack_r.push(r);
    stack.cnt += 1;
    
    while stack.cnt > 0 {
        stack.cnt -= 1;
        let tl = stack.stack_l.pop().unwrap();
        let tr = stack.stack_r.pop().unwrap();
        
        if tl >= tr {
            continue;
        }
        
        if tr - tl < BZP_THRESHOLD_SHELL_SORT {
            bzp_shell_sort(sort_block, idx, tl, tr);
            continue;
        }
        
        stack.tl = tl;
        stack.tr = tr;
        bzp_qsort_single(sort_block, idx, &mut stack);
    }
}

// Assuming the following functions are defined elsewhere:
// fn bzp_shell_sort(sort_block: &mut [i32], idx: &mut [i32], l: i32, r: i32);
// fn bzp_qsort_single(sort_block: &mut [i32], idx: &mut [i32], stack: &mut BzpQSortInfo);
