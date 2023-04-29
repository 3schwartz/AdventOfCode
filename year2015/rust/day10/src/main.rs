fn main() {
    let input = 1321131112;

    let part_1 = play_game(input, 40);

    println!("Part 1: {}", part_1);

    let part_2 = play_game(input, 50);

    println!("Part 2: {}", part_2);
}

fn play_game(sequence: i32, count: u8) -> usize {
    let mut input = sequence;
    let mut chars = vec![];

    while input != 0 {
        let this = input % 10;
        chars.push(this as u8);
        input /= 10;

    }
    chars.reverse();

    for _ in 0..count {
        chars = look_say(chars);
    }

    chars.len()
}

fn look_say(input: Vec<u8>) -> Vec<u8> {
    let mut count = 0;
    let mut last = 0;

    let mut output = vec![];

    for c in input {
        if c == last {
            count += 1;
            continue;
        }
        if count == 0 {
            count+=1;
            last = c;
            continue;
        }
        output.push(count);
        output.push(last);
        count = 1;
        last = c;
    };
    output.push(count);
    output.push(last);

    output
}