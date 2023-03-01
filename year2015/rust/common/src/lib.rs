use std::{fs::File, io, path::Path};
use std::io::prelude::*;

/// Return lines of file
pub fn read_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>()
}