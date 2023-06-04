fn main() {
    let row = 2978;
    let col = 3083;
    let code = find_code(row, col);

    println!("Part 1: {}", code);
}

/// Observe the rows follows (n-1) + (n-2) + (n-3) + ... + 1
/// which is a triangular sequence.
/// Since (n-1) + (n-2) + (n-3) = n * (n-1) / 2 we have
/// (n-1) + (n-2) + (n-3) + ... + 1 = n * (n-1) / 2 + 1.
/// Hence idx (n,1) is n * (n-1) / 2 + 1.
/// Since the col decreases the row we need to first shift the 
/// initial row value with col, use formula and then add back col to get 
/// final index.
fn find_code(row: u64, col: u64) -> u64 {
    let n = row + col - 1;
    let r = (n-1)*n/2 + 1;
    let c = r + col -1;
    let mut previous = 20151125;
    for _ in 1..c {
        let temp = previous * 252533;
        previous = temp % 33554393;
    }
    return previous;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        // Arrange
        let data = vec![
            (2,1,31916031),
            (1,2,18749137),
            (2,2,21629792),
            (5,4,6899651)
            ];

        // Act
        for d in data {
            let (row, col, expected) = d;

            let actual = find_code(row, col);
    
            // Assert
            assert_eq!(actual, expected);
        }
    }
}