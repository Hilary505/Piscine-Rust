pub fn nextprime(nbr: u64) -> u64 {
    if nbr < 2 {
        return 0;
    }
    let mut n = nbr;
    while n >= 2 {
        if is_prime(n) {
            return n;
        }
        n += 1
    }
    0
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64 + 1;

    for i in 3..sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
