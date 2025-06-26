pub fn slist_sort_internal(mut list: Ptr<Ptr<SListEntry>>, mut compare_func: SListCompareFunc) -> Ptr<SListEntry> {
    let mut pivot: Ptr<SListEntry>;
    let mut rover: Ptr<SListEntry>;
    let mut less_list: Ptr<SListEntry>;
    let mut more_list: Ptr<SListEntry>;
    let mut less_list_end: Ptr<SListEntry>;
    let mut more_list_end: Ptr<SListEntry>;

    if (*list == NULL!()).as_bool() || ((*list).next == NULL!()).as_bool() {
        return (*list).cast();
    }

    pivot = (*list).cast();

    less_list = NULL!();
    more_list = NULL!();
    rover = (*list).next.cast();

    while (rover != NULL!()).as_bool() {
        let mut next: Ptr<SListEntry> = rover.next.cast();

        if (compare_func(rover.data.cast(), pivot.data.cast()) < 0).as_bool() {
            rover.next = less_list.cast();
            less_list = rover.cast();
        } else {
            rover.next = more_list.cast();
            more_list = rover.cast();
        }

        rover = next.cast();
    }

    less_list_end = slist_sort_internal(c_ref!(less_list).cast(), compare_func.cast()).cast();
    more_list_end = slist_sort_internal(c_ref!(more_list).cast(), compare_func.cast()).cast();

    *list = less_list.cast();

    if (less_list == NULL!()).as_bool() {
        *list = pivot.cast();
    } else {
        less_list_end.next = pivot.cast();
    }

    pivot.next = more_list.cast();

    if (more_list == NULL!()).as_bool() {
        return pivot.cast();
    } else {
        return more_list_end.cast();
    }
}
