pub fn pointer_hash(mut location: Ptr<Void>) -> u32 {
    return location.cast::<usize>().cast::<u32>();
}
