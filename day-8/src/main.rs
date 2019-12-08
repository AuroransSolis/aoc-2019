use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let input = input.trim();
    let mut input_chars = input.chars().rev();
    let mut min_zeros = std::usize::MAX;
    let mut one_count = 0;
    let mut two_count = 0;
    let mut image = [['░'; 25]; 6];
    'layers: loop {
        let mut layer_zero_count = 0;
        let mut layer_one_count = 0;
        let mut layer_two_count = 0;
        for y in (0..6).rev() {
            for x in (0..25).rev() {
                match input_chars.next() {
                    Some('0') => {
                        layer_zero_count += 1;
                        image[y][x] = ' ';
                    },
                    Some('1') => {
                        layer_one_count += 1;
                        image[y][x] = '█'
                    },
                    Some('2') => layer_two_count += 1,
                    None => break 'layers,
                    _ => continue,
                }
            }
        }
        if layer_zero_count < min_zeros {
            min_zeros = layer_zero_count;
            one_count = layer_one_count;
            two_count = layer_two_count;
        }
    }
    println!("p1: {}", one_count * two_count);
    println!("p2:");
    for row in image.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!();
    }
}
