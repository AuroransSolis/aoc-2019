use std::io::{stdin, Read};
use std::iter::Iterator;

struct Password {
    current: [u8; 6],
    max: [u8; 6]
}

impl Password {
    fn new(start: &[u8], stop: &[u8]) -> Self {
        let mut current = [0; 6];
        current.copy_from_slice(&start[..]);
        let mut max = [0; 6];
        max.copy_from_slice(&stop[..]);
        Password {
            current,
            max,
        }
    }

    fn next(&mut self) -> bool {
        if self.current == self.max {
            false
        } else {
            let mut ind = 5;
            while self.incr(ind) {
                ind -= 1;
            }
            true
        }
    }

    fn is_valid_p1(&self) -> bool {
        let mut double = false;
        for i in  0..5 {
            if self.current[i] == self.current[i + 1] {
                double = true;
            }
            for j in i + 1..6 {
                if self.current[i] > self.current[j] {
                    return false;
                }
            }
        }
        double
    }

    fn is_valid_p2(&self) -> bool {
        let mut counts = [0; 10];
        counts[self.current[5] as usize] += 1;
        for i in  0..5 {
            counts[self.current[i] as usize] += 1;
            for j in i + 1..6 {
                if self.current[i] > self.current[j] {
                    return false;
                }
            }
        }
        counts.contains(&2)
    }

    fn incr(&mut self, n: usize) -> bool {
        self.current[n] += 1;
        if self.current[n] == 10 {
            self.current[n] -= 10;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input);
    let mut split = input.split('-');
    let n1 = split.next().unwrap();
    let n2 = split.next().unwrap().trim();
    let mut n1_nums = [0; 6];
    n1.chars().enumerate().map(|(i, c)| (i, c as u8 & 15)).for_each(|(i, n)| n1_nums[i] = n);
    let mut n2_nums = [0; 6];
    n2.chars().enumerate().map(|(i, c)| (i, c as u8 & 15)).for_each(|(i, n)| n2_nums[i] = n);
    let mut password = Password::new(&n1_nums, &n2_nums);
    let mut p1_count = 0;
    let mut p2_count = 0;
    while password.next() {
        if password.is_valid_p1() {
            p1_count += 1;
        }
        if password.is_valid_p2() {
            p2_count += 1;
        }
    }
    println!("p1: {}\np2: {}", p1_count, p2_count);
}
