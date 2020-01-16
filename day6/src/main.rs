use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn traverse(orbits: &HashMap<String, Option<String>>) -> u32 {
    let mut visited: Vec<String> = vec![];
    let mut to_visit: Vec<String> = orbits.keys().map(|x| x.to_string()).collect();
    for (name, descendant) in orbits {
        match descendant {
            Some(x) => {
                if let Some(pos) = to_visit.iter().position(|z| z == x) {
                    to_visit.swap_remove(pos);
                }
            }
            _ => ()
        }
    }
    while to_visit.len() > 0 {
        visited.push(to_visit[0].into());
        let parent: String = orbits.get(&to_visit[0]).unwrap().unwrap();
        to_visit.push(parent);
    }

    visited.len() as u32
}

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
            (*col).insert(nodes[1].into(), Some(nodes[0].into()));
            (*col).entry(nodes[1].into()).or_insert(None);
            col
        });

    println!("orbits: {:?}", result);

    println!("#orbits: {}", traverse(&result));

    Ok(())
}
