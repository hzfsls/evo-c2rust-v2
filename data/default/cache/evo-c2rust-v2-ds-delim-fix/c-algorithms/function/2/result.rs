pub fn list_append(mut list: Ptr<Ptr<ListEntry>>, mut data: ListValue) -> Ptr<ListEntry> {
    let mut rover: Ptr<ListEntry> = Default::default();
    let mut newentry: Ptr<ListEntry> = Default::default();

    if (list == NULL!()).as_bool() {
        return NULL!();
    }

    newentry = c_malloc!(c_sizeof!(ListEntry));

    if (newentry == NULL!()).as_bool() {
        return NULL!();
    }

    newentry.data = data.cast();
    newentry.next = NULL!();

    if (*list == NULL!()).as_bool() {
        *list = newentry.cast();
        newentry.prev = NULL!();
    } else {
        rover = *list.cast();
        while (rover.next != NULL!()).as_bool() {
            rover = rover.next.cast();
        }

        newentry.prev = rover.cast();
        rover.next = newentry.cast();
    }

    return newentry.cast();
}
