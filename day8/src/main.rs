use std::fs::File;
use std::io::prelude::*;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> std::io::Result<()> {
 let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();
    let mut images = vec![];

    assert_eq!(contents.len() % WIDTH, 0);

    let items = contents.len() / WIDTH;
    let mut chars = contents.chars();
    let mut zeros = vec![];
    let mut zeroscnt = 0;
    let mut onecnt = 0;
    let mut twocnt = 0;
    let mut min_layer = std::i32::MAX;
    while images.len() < items {
        let mut buf = vec![];
        let mut cnt = 0;
        while buf.len() < WIDTH {
            let chr = chars.next().unwrap();
            if let Some(digit) = chr.to_digit(10) {
                match digit {
                    0 => cnt += 1,
                    1 => onecnt += 1,
                    2 => twocnt += 1,
                    _ => (),
                }
            }
            buf.push(chr);
        }
        zeros.push(cnt);
        zeroscnt += 1;
        if zeroscnt == HEIGHT {
            let layer_zeros = zeros.iter().sum::<i32>();
            if layer_zeros < min_layer {
                min_layer = layer_zeros;
                println!("new min {}, prod = {}", min_layer, onecnt * twocnt);
            }
            // Reset layer couting
            zeroscnt = 0;
            onecnt = 0;
            twocnt = 0;
            zeros.clear();
        }

        images.push(buf);
    }

    assert_eq!(min_layer, 6);

    Ok(())
}
