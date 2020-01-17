use std::fs::File;
use std::io::prelude::*;

fn compute(opcode: i32, in1: i32, in2: i32) -> i32 {
    match opcode {
        1 => in1 + in2,
        2 => in1 * in2,
        _ => {
            println!("Unknown op {}", opcode);
            0
        },
    }
}

fn mode_to_value(input: &mut Vec<i32>, mode: i32, param: i32) -> i32 {
    match mode {
        0 => input[param as usize],
        1 => param,
        _ => {
            println!("Unknown mode {}", mode);
            0
        },
    }
}

fn eval(input: &mut Vec<i32>) -> () {
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
            3 => {
                let out = input[curr + 1] as usize;
                curr += 1;
            }
            4 => {
                let out = input[curr + 1] as usize;
                curr += 1;
            }
            99 => {
                break;
            }
            _ => {
                let opcode = input[curr] % 100;
                let mode1 = (input[curr] % 1000 - opcode) / 100;
                let mode2 = (input[curr] % 10000 - opcode - mode1) / 1000;
                let mode3 = (input[curr] % 100000 - opcode - mode1 - mode2) / 10000;
                println!("opcode = {}, mode1 = {}, mode2 = {}, mode3 = {}", opcode, mode1, mode2, mode3);
                let in1 = mode_to_value(input, mode1, input[curr + 1]);
                let in2 = mode_to_value(input, mode2, input[curr + 2]);
                let out = mode_to_value(input, mode3, input[curr + 3]) as usize;
                println!("retrieved {} in mode {} from {}", out, mode3, input[curr + 3]);
                println!("in1 = {}, in2 = {}, out={}", in1, in2, out);
                input[out] = compute(opcode, in1, in2);
                curr += 1;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
 let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut input: Vec<i32>  = contents.split(",")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    eval(&mut input);

    Ok(())
}

#[test]
fn test_add() {
    let mut input: Vec<i32> = vec![1, 1, 2, 0, 99];
    eval(&mut input);
    assert_eq!(input, vec![3, 1, 2, 0, 99]);
}

#[test]
fn test_mul() {
    let mut input: Vec<i32> = vec![2, 2, 2, 0, 99];
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
