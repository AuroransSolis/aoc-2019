use std::io::{stdin, Read};
use std::iter::Iterator;
use std::ops::Range;

const RANGES: [Range<usize>; 9] = [
    000000..089000, // [000000, 088999]
    111111..189000, // [111111, 188999]
    222222..289000, // [222222, 288999]
    333333..389000, // [333333, 388999]
    444444..489000, // [444444, 488999]
    555555..589000, // [555555, 588999]
    666666..689000, // [666666, 688999]
    777777..789000, // [777777, 788999]
    888888..890000, // [888888, 889999]
];

const STARTS: [[u8; 6]; 9] = [
    [0; 6], // 000000
    [1; 6], // 111111
    [2; 6], // 222222
    [3; 6], // 333333
    [4; 6], // 444444
    [5; 6], // 555555
    [6; 6], // 666666
    [7; 6], // 777777
    [8; 6], // 888888
    // No 999999 since that's the only
];

struct Password {
    s_range: usize,
    e_range: usize,
    s_val: usize,
    e_val: usize,
    key: [u8; 6],
}

impl Password {
    fn new(start: [u8; 6], stop: [u8; 6]) -> Self {
        let key = start;
        let s_val = key[0] as usize * 100_000 + key[1] as usize * 10_000
            + key[2] as usize * 1000 + key[3] as usize * 100 + key[4] as usize * 10
            + key[5] as usize;
        let s_range = RANGES.iter()
            .position(|range| range.contains(&s_val) || range.start >= s_val)
            .unwrap();
        let max = stop;
        let e_val = max[0] as usize * 100_000 + max[1] as usize * 10_000
            + max[2] as usize * 1000 + max[3] as usize * 100 + max[4] as usize * 10
            + max[5] as usize;
        let e_range = RANGES.iter()
            .rposition(|range| range.contains(&e_val) || range.end <= e_val)
            .unwrap();
        Password {
            s_range,
            e_range,
            s_val,
            e_val,
            key,
        }
    }

    fn solve(&mut self) -> (usize, usize) {
        let mut p1 = 0;
        let mut p2 = 0;
        // Handle cases where the start and end are in the same valid range of numbers
        if self.s_range == self.e_range {
            // Find the appropriate start value for the range. If the start range contains the start
            // value, then the range should start at the start value. Otherwise, the start of the
            // range is the next valid number after the start value, so go with that. And if that's
            // the case, set the key to the first valid key in that range.
            let start = if RANGES[self.s_range].contains(&self.s_val) {
                self.s_val
            } else {
                let new_key = STARTS[self.s_range];
                self.key = new_key;
                RANGES[self.s_range].start
            };
            // Find the appropriate end value for the range. If the end range contains the end
            // value, then the range should end at the end value. Otherwise, the end of the range is
            // the last valid number in the range before the end value, so go with that.
            let end = if RANGES[self.s_range].contains(&self.e_val) {
                self.e_val
            } else {
                RANGES[self.e_range].end
            };
            for _ in start..end {
                if self.is_valid_p1() {
                    p1 += 1;
                }
                if self.is_valid_p2() {
                    p2 += 1;
                }
                self.incr();
            }
        } else {
            // Find the appropriate start value for the range. If the start range contains the start
            // value, then the range should start at the start value. Otherwise, the start of the
            // range is the next valid number after the start value, so go with that. And if that's
            // the case, set the key to the first valid key in that range.
            let start_range = if RANGES[self.s_range].contains(&self.s_val) {
                self.s_val..RANGES[self.s_range].end
            } else {
                let new_key = STARTS[self.s_range];
                self.key = new_key;
                RANGES[self.s_range].clone()
            };
            for _ in start_range {
                if self.is_valid_p1() {
                    p1 += 1;
                }
                if self.is_valid_p2() {
                    p2 += 1;
                }
                self.incr();
            }
            // For each of the valid ranges between the input numbers, total up the valid keys for
            // p1 and p2
            for i in self.s_range + 1..self.e_range {
                self.key = STARTS[i];
                for _ in RANGES[i].clone() {
                    if self.is_valid_p1() {
                        p1 += 1;
                    }
                    if self.is_valid_p2() {
                        p2 += 1;
                    }
                    self.incr();
                }
            }
            let new_key = STARTS[self.e_range];
            self.key = new_key;
            // Find the appropriate end value for the range. If the end range contains the end
            // value, then the range should end at the end value. Otherwise, the end of the range is
            // the last valid number in the range before the end value, so go with that.
            let end_range = if RANGES[self.e_range].contains(&self.e_val) {
                RANGES[self.e_range].start..self.e_val
            } else {
                RANGES[self.e_range].clone()
            };
            for _ in end_range {
                if self.is_valid_p1() {
                    p1 += 1;
                }
                if self.is_valid_p2() {
                    p2 += 1;
                }
                self.incr();
            }
        }
        (p1, p2)
    }

    fn is_valid_p1(&self) -> bool {
        let mut double = false;
        for i in  0..5 {
            if self.key[i] == self.key[i + 1] {
                double = true;
            }
            // Return false if the value in the next digit decreases.
            if self.key[i] > self.key[i + 1] {
                return false;
            }
        }
        double
    }

    fn is_valid_p2(&self) -> bool {
        // Count the occurrences of each digit in `counts`.
        let mut counts = [0; 10];
        counts[self.key[5] as usize] += 1;
        for i in  0..5 {
            counts[self.key[i] as usize] += 1;
            // Return false if the value in the next digit decreases.
            if self.key[i] > self.key[i + 1] {
                return false;
            }
        }
        // If `counts` contains a 2, that means there was a standalone double, which is what p2
        // requires.
        counts.contains(&2)
    }

    // Increment the key value. Like regular addition, but I've implemented wrapping myself.
    fn incr(&mut self) {
        let mut ind = 5;
        while self.incr_n(ind) {
            ind -= 1;
        }
    }

    // Increment a digit in the key. Returns `true` if wrapping should occur, and `false` if not.
    fn incr_n(&mut self, n: usize) -> bool {
        self.key[n] += 1;
        if self.key[n] == 10 {
            self.key[n] -= 10;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut split = input.split('-');
    let n1 = split.next().unwrap();
    let n2 = split.next().unwrap().trim();
    let mut n1_nums = [0; 6];
    n1.chars().enumerate().map(|(i, c)| (i, c as u8 & 15)).for_each(|(i, n)| n1_nums[i] = n);
    let mut n2_nums = [0; 6];
    n2.chars().enumerate().map(|(i, c)| (i, c as u8 & 15)).for_each(|(i, n)| n2_nums[i] = n);
    let mut password = Password::new(n1_nums, n2_nums);
    let (mut p1, p2) = password.solve();
    if n2_nums == [9; 6] {
        p1 += 1;
    }
    println!("p1: {}\np2: {}", p1, p2);
}
