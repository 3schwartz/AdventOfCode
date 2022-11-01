use std::{env, fs};
use std::path::Path;
use relative_path::RelativePath;

fn main() -> std::io::Result<()> {
    let root  = env::current_dir()?;
    println!("The current directory is {}", root.display());

    let relativePath = RelativePath::new("../data/day1_data.txt");
    let fullPath = relativePath.to_path(&root);

    println!("The full path is {}", fullPath.display());

    let file = match fs::read_to_string(fullPath) {
        Err(why) => panic!("couldn't open file: {}", why),
        Ok(file) => file
    };

    let lines = file.split("\r\n");

    let mut sum :i32 = 0;
    for s in lines {
        let numberString = &s[1..];
        let mut foo = numberString.parse();
        let mut number : i32 = foo.unwrap();
        let sign = s.chars().next().unwrap();
        match sign {
            '+'=> {
                number = number
            },
            '-'=> {
                number = -1*number
            }
            _ => println!("{} not known", sign)
        }
        sum+= number;
    }
    println!("Part 1: {}", sum);
    Ok(())
}
