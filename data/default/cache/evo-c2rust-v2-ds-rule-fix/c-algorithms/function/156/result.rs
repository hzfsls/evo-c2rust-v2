pub fn slist_append(mut list: Ptr<Ptr<SListEntry>>, mut data: SListValue) -> Ptr<SListEntry> {
    let mut rover: Ptr<SListEntry> = Default::default();
    let mut newentry: Ptr<SListEntry> = Default::default();

    newentry = c_malloc!(c_sizeof!(SListEntry));

    if (newentry == NULL!()) {
        return NULL!();
    }

    newentry.data = data;
    newentry.next = NULL!();

    if (*list == NULL!()) {
        *list = newentry;
    } else {
        rover = *list;
        while (rover.next != NULL!()) {
            rover = rover.next;
        }
        rover.next = newentry;
    }

    return newentry;
}
