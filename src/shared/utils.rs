pub fn is_uniq_sorted<T: PartialOrd>(v: &Vec<T>) -> bool {
    if v.is_empty() || v.len() == 1 {
        return true;
    }

    for i in 0..(v.len() - 1) {
        if v[i] >= v[i + 1] {
            return false;
        }
    }

    true
}

pub fn all_zeroes(v: &Vec<f32>) -> bool {
    v.iter().fold(true, |acc, v| acc && *v == 0.0)
}
