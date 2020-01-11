use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
 let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input: Vec<u32>  = contents.split(",")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    eval(&mut input);

    println!("Total = {:?}", input);
    let diff: i32 = 19690720 - (input[0] as i32);
    println!("diff = {}", diff);

    Ok(())
}
