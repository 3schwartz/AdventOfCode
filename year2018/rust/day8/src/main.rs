use std::fs;

fn main() {
    let file = fs::read_to_string("../data/day8_data.txt")
        .expect("couldn't open file");
    let tree = Tree::new(file);

    let meta_sum = tree.get_meta_sum();

    println!("Part 1: {}", meta_sum);

    let root_value = tree.get_root_value();

    println!("Part 2: {}", root_value);
}

struct Tree {
    children: Vec<Tree>,
    meta: Vec<u32>
}

impl Tree {
    fn new(file: String) -> Self {
        let numbers = file.split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        let idx = 0;
        let (tree, _) = Tree::create_tree(idx, &numbers);

        return tree
    }

    fn create_tree(mut idx: usize, numbers: &Vec<u32>) -> (Tree, usize) {
        let mut children = Vec::<Tree>::new();
        let number_of_children: u32 = numbers[idx];
        let number_of_meta = numbers[idx+1];
        idx += 2;
        for _ in 0..number_of_children{
            let (tree, id) = Tree::create_tree(idx, numbers);
            children.push(tree);
            idx = id
        }
        let mut meta = Vec::<u32>::new();
        for _ in 0..number_of_meta {
            meta.push(numbers[idx]);
            idx += 1;
        }

        return (Tree{children, meta}, idx)
    }

    fn get_meta_sum(&self) -> u32 {
        let mut sum = 0;
        for child in &self.children {
            sum += child.get_meta_sum();
        }
        for meta in &self.meta {
            sum += meta;
        }
        return sum;
    }

    fn get_root_value(&self) -> u32 {
        let child_length = self.children.len();
        let mut sum = 0;

        for meta in &self.meta {
            if child_length == 0 {
                sum += meta;
                continue;
            }
            let idx = (meta - 1) as usize;
            if idx >= child_length {
                continue;
            }
            sum += self.children[idx].get_root_value();
        }

        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        // Arrange
        let file = fs::read_to_string("../data/day8_test_data.txt")
            .expect("couldn't open file");
        let tree = Tree::new(file);

        // Act
        let root_value = tree.get_root_value();

        // Assert
        assert_eq!(66, root_value);
    }
    
    #[test]
    fn test_part1() {
        // Arrange
        let file = fs::read_to_string("../data/day8_test_data.txt")
            .expect("couldn't open file");
        let tree = Tree::new(file);

        // Act
        let meta_sum = tree.get_meta_sum();

        // Assert
        assert_eq!(138, meta_sum);
    }
}
