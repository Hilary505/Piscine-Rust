pub fn reverse_it(v: i32) -> String {
    let s = v.abs().to_string();
    let mut res = s.chars().rev().collect::<String>() + &s;
    if v < 0 {
        res.insert(0, '-');
    }
    res
}
