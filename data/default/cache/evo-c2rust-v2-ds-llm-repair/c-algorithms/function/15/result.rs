pub fn list_find_data(mut list: Ptr<ListEntry>, mut callback: ListEqualFunc, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = Default::default();

    rover = list.cast();
    while (rover != NULL!()).as_bool() {
        if (callback(rover.data.cast(), data.cast()) != 0).as_bool() {
            return rover.cast();
        }
        rover = rover.next.cast();
    }

    return NULL!();
}
