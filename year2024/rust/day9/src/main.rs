use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/day9_data.txt")?;

    let blocks = make_blocks(&input);
    let ordered = order_blocks(&blocks);
    let calcualted = calculate(&ordered);

    println!("Part 1: {}", calcualted);

    let ordered = order_blocks_part_2(&blocks);
    let calcualted = calculate_part_2(&ordered);

    println!("Part 2: {}", calcualted);

    Ok(())
}

fn calculate(blocks: &[Block]) -> i64 {
    let mut sum = 0;
    let mut index = 0;
    for block in blocks {
        let next = block.length + index;
        for i in index..next {
            sum += block.id * i;
        }
        index = next;
    }
    sum
}

fn calculate_part_2(blocks: &[Block]) -> i64 {
    let mut sum = 0;
    let mut index = 0;
    for block in blocks {
        let next = block.length + index;
        for i in index..next {
            sum += block.id * i;
        }
        index = next + block.free_length;
    }
    sum
}

fn order_blocks_part_2(input: &[Block]) -> Vec<Block> {
    let mut blocks: Vec<Block> = input.to_vec();

    let mut e_i = blocks.len() as i64;
    loop {
        e_i -= 1;
        if e_i == 0 {
            break;
        }
        let mut e = usize::MAX;
        for t in (0..blocks.len()).rev() {
            let temp = blocks[t];
            if temp.id == e_i {
                e = t;
                break;
            }
        }
        assert!(e != usize::MAX);

        let end = blocks[e];

        for s in 0..blocks.len() {
            if e == s {
                break;
            }
            let mut start = blocks[s];
            if start.free_length < end.length && end.length != 0 {
                continue;
            }
            let mut block = Block {
                id: end.id,
                length: end.length,
                free_length: start.free_length - end.length,
                is_dirty: true,
            };
            if s == e - 1 {
                block.free_length += end.free_length + end.length;
            } else {
                blocks[e - 1].free_length += end.free_length + end.length;
            }
            blocks.remove(e);
            start.free_length = 0;
            blocks[s] = start;
            blocks.insert(s + 1, block);
            break;
        }
    }

    blocks
}

fn order_blocks(input: &[Block]) -> Vec<Block> {
    let mut blocks: Vec<Block> = input.to_vec();

    let mut s = 0;
    loop {
        if s == blocks.len() - 1 {
            break;
        }
        if blocks[s].free_length == 0 {
            s += 1;
            continue;
        }

        let mut start = blocks[s];
        let e = blocks.len() - 1;
        let mut end = blocks[e];

        let mut_to_insert = if end.length <= start.free_length {
            let length = end.length;
            let free_length = start.free_length - end.length;
            start.free_length = 0;
            end.length = 0;
            Block {
                id: end.id,
                length,
                free_length,
                is_dirty: true,
            }
        } else {
            let length = start.free_length;
            end.length -= start.free_length;
            start.free_length = 0;
            Block {
                id: end.id,
                length,
                free_length: 0,
                is_dirty: true,
            }
        };
        if end.length == 0 {
            blocks.remove(e);
        } else {
            blocks[e] = end;
        }
        if s == blocks.len() {
            blocks.push(start);
        } else {
            blocks[s] = start;
        }

        blocks.insert(s + 1, mut_to_insert);
        s += 1;
    }

    blocks
}

#[derive(Clone, Copy, Default)]
struct Block {
    id: i64,
    length: i64,
    free_length: i64,
    is_dirty: bool,
}

fn make_blocks(input: &str) -> Vec<Block> {
    let mut blocks = vec![];

    let mut current = Block::default();
    for (i, c) in input.chars().enumerate() {
        if !c.is_numeric() {
            continue;
        }
        let n = (c as u8 - b'0') as i64;
        if i % 2 != 0 {
            current.free_length = n;
            blocks.push(current);
            current = Block::default()
        } else {
            current.id = (i / 2) as i64;
            current.length = n;
            current.is_dirty = true;
        }
    }
    if current.is_dirty {
        blocks.push(current);
    }
    blocks
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_test_data.txt")?;

        // Act
        let blocks = make_blocks(&input);
        let ordered = order_blocks(&blocks);
        let calcualted = calculate(&ordered);

        // Assert
        assert_eq!(calcualted, 1928);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        // Arrange
        let input = fs::read_to_string("../../data/day9_test_data.txt")?;

        // Act
        let blocks = make_blocks(&input);
        let ordered = order_blocks_part_2(&blocks);
        let calcualted = calculate_part_2(&ordered);

        // Assert
        assert_eq!(calcualted, 2858);
        Ok(())
    }
}
