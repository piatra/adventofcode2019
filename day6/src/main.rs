use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let orbits: &mut HashMap<_, _> = &mut HashMap::new();
    let result = contents
        .split("\n")
        .filter(|x| x.len() > 0)
        .fold(orbits, |col: &mut HashMap<String, Option<String>>, line: &str| {
            let nodes: Vec<&str> = line.split(")").collect();
            (*col).insert(nodes[0].into(), Some(nodes[1].into()));
            (*col).entry(nodes[1].into()).or_insert(None);
            col
        });

    println!("orbits: {:?}", result);

    Ok(())
}
