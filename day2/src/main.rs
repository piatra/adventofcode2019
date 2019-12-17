use std::fs::File;
use std::io::prelude::*;

fn eval(input: &mut Vec<u32>) -> () {
    let mut curr = 0;
    loop {
        match input[curr] {
            1 => {
                let out = input[curr + 3] as usize;
                let in1 = input[curr + 1] as usize;
                let in2 = input[curr + 2] as usize;
                input[out] = input[in1] + input[in2];
                println!("{} + {} to {}", input[in1], input[in2], input[out]);
                curr += 3;
            }
            2 => {
                let out = input[curr + 3] as usize;
                let in1 = input[curr + 1] as usize;
                let in2 = input[curr + 2] as usize;
                input[out] = input[in1] * input[in2];
                println!("{} * {} to {}", input[in1], input[in2], input[out]);
                curr += 3;
            }
            99 => {
                break;
            }
            _ => curr += 1
        }
    }
}

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

#[test]
fn test_add() {
    let mut input: Vec<u32> = vec![1, 1, 2, 0, 99];
    eval(&mut input);
    assert_eq!(input, vec![3, 1, 2, 0, 99]);
}

#[test]
fn test_mul() {
    let mut input: Vec<u32> = vec![2, 2, 2, 0, 99];
    eval(&mut input);
    assert_eq!(input, vec![4, 2, 2, 0, 99]);
}

#[test]
fn test_case1() {
    let mut input = vec![1,1,1,4,99,5,6,0,99];
    eval(&mut input);
    assert_eq!(input, vec![30,1,1,4,2,5,6,0,99]);
}

#[test]
fn test_case2() {
    let mut input = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    eval(&mut input);
    assert_eq!(input, vec![3500,9,10,70,2,3,11,0,99,30,40,50]);
}
