pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        if factors.iter().any(|&factor| factor != 0 && i % factor == 0) {
            sum += i;
        }
    }
    sum
}

//impl using for_each
/*pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    for i in 1..limit {
        let mut is_multiple = false;
        factors.iter().for_each(|&factor| {
            if factor != 0 && i % factor == 0 {
                is_multiple = true;
            }
        });
        if is_multiple {
            sum += i;
        }
    }
    sum
}*/