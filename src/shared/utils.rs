pub fn is_sorted<T: PartialOrd>(v: &Vec<T>) -> bool {
    if v.is_empty() || v.len() == 1 {
        return true;
    }

    for i in 0..(v.len() - 1) {
        if v[i] > v[i + 1] {
            return false;
        }
    }

    true
}
