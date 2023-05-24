use anyhow::{Result, anyhow, Ok};

fn main() -> Result<()>{
    let input = 29000000;

    let lowest_house = force(input, input, |house| find_divisors(house, 10))
        .ok_or_else(|| anyhow!("not able to find part 1"))?;
    println!("Part 1: {}", lowest_house);

    let lowest_house = force(input, 50, |house| find_divisors2(house, 11))
        .ok_or_else(|| anyhow!("not able to find part 2"))?;
    println!("Part 2: {}", lowest_house);
    
    Ok(())
}

fn force(find: u64, base: u64, find_div: fn(u64) -> u64) -> Option<u64> {
    for i in (find / base)..=find {
        let present = find_div(i);
        if present > find {
            return Some(i)
        }
    }
    None
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


/// Not working since results are not sorted.
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