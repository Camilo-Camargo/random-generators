use std::collections::HashMap;

fn linear_congruential(multiplier: &u128, seed: &u128, increment: &u128, modulus: &u128) -> u128 {
    return (multiplier * seed + increment) % modulus;
}

fn linear_congruential_generator(
    multiplier: &u128,
    seed: &u128,
    increment: &u128,
    modulus: &u128,
) -> (u128, u128, HashMap<u128, u128>) {
    let mut randoms: HashMap<u128, u128> = HashMap::new();

    let mut i: u128 = 0;
    let mut before: u128;

    let mut random: u128 = linear_congruential(multiplier, seed, increment, modulus);
    randoms.insert(random.clone(), i);

    let period: u128;
    let duplicate: u128;

    before = random.clone();

    loop {
        i += 1;
        random = linear_congruential(multiplier, &before, increment, modulus);
        before = random.clone();
        match randoms.get(&random) {
            Some(target) => {
                period = i - target;
                duplicate = random.clone();
                break;
            }
            None => {
                randoms.insert(random.clone(), i);
            }
        }
    }

    return (period, duplicate, randoms);
}

fn main() {
    let multiplier: u128 = 203;
    let increment: u128 = 0;
    let modulus: u128 = u128::pow(10,5);
    let seed: u128 = 17;

    let (period, duplicate, _randoms) =
        linear_congruential_generator(&multiplier, &seed, &increment, &modulus);

    println!("Duplicate: {duplicate}");
    println!("Period: {period}");
}
