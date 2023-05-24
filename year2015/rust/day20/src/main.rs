

fn main() {
    let input = 29000000;

    let lowest_house = bs(input);
    println!("Part 1: {}", lowest_house);

    let lowest_house = force(input, 10);
    println!("Part 1: {}", lowest_house);

    let lowest_house = force2(input, 11);
    println!("Part 2: {}", lowest_house);
}

fn force2(find: u64, multiple: u64) -> u64 {
    let base = find / 500;
    for i in base..=find {
        let present = find_divisors2(i, multiple);
        if present > find {
            return i
        }
    }
    0
}

fn find_divisors2(house: u64, multiple: u64) -> u64 {
    let mut presents = 0;
    let sqrt_n = (house as f64).sqrt() as u64;
    for i in 1..=sqrt_n {
        if house % i == 0 {
            let quotient = house / i;
            
            if quotient <= 50 {
                presents += i * multiple;
            }
            
            if quotient != i && house / quotient <= 50 {
                presents += quotient * multiple;
            }
        }
    }
    presents
}

fn force(find: u64, multiple: u64) -> u64 {
    for i in 1..find {
        let present = find_divisors(i, multiple);
        if present > find {
            return i
        }
    }
    0
}

#[allow(dead_code)]
fn bs(find: u64) -> u64 {
    let mut low = 0;
    let mut high = find;
    loop {
        if low == high {
            break;
        }
        let mid = low + (high - low) / 2;
        let presents = find_divisors(mid, 10);
        if presents < find {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn find_divisors(house: u64, multiple: u64) -> u64 {
    let mut presents = 0;
    let sqrt_n = (house as f64).sqrt() as u64;
    for i in 1..=sqrt_n {
        if house % i == 0 {
            presents += i * multiple;

            let quotient = house / i;
            if quotient != i {
                presents += quotient * multiple;
            }
        }
    }
    presents
}