pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut pramid =  Vec::new();

    for n in 1..=i {
        let start = " ".repeat((n - 1) as usize);
        let rep = v.repeat(n as usize);
        let line = format!("{}{}", start,rep);
        pramid.push(line);
    }

    for n in (1..i).rev() {
        let start = " ".repeat((n - 1) as usize);
        let rep = v.repeat(n as usize);
        let line = format!("{}{}", start,rep);
        pramid.push(line);
    }
    pramid
}