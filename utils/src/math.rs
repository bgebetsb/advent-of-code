pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }

    if b > a {
        std::mem::swap(&mut a, &mut b);
    }

    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }

    a
}

pub fn lcm_two(a: usize, b: usize) -> usize {
    a * (b / gcd(a, b))
}

pub fn lcm(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .fold(*numbers.first().unwrap(), |current_lcm, current_item| {
            lcm_two(current_lcm, *current_item)
        })
}
