pub fn nth(n: u32) -> u32 {
    let mut num: u32 = 1;

    let is_prime = |p: u32| -> bool {
        for num in 2..p {
            if p % num == 0 {
                return false;
            }
        }
        true
    };

    for _ in 0..n + 1 {
        num += 1;

        while !is_prime(num) {
            num += 1;
        }
    }

    num
}
