pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    T: PartialOrd,
    U: AsRef<[T]>,
{
    let list = array.as_ref();
    if list.is_empty() {
        return None;
    }

    let mut l = 0;
    let mut r = list.len() - 1;

    while l != r {
        let mid = ((l + r) as f64 / 2.0).ceil() as usize;
        if list[mid] > key {
            r = mid - 1;
        } else {
            l = mid;
        }
    }
    if list[l] == key {
        return Some(l);
    }
    None
}
