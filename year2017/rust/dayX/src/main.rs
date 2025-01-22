use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("../data/dayX_data.txt")?;

    println!("{input}");
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_part_1() {
        // Arrange
        // Act
        // Assert
        assert!(true)
    }
}
