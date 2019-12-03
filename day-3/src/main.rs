use std::io::{stdin, BufRead};
use std::mem;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn is_vertical(&self) -> bool {
        match self {
            Direction::U | Direction::D => true,
            _ => false,
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Direction::R | Direction::L => true,
            _ => false,
        }
    }

    fn walk(&self, x: &mut i32, y: &mut i32, amt: i32) {
        match self {
            Direction::U => *y += amt,
            Direction::R => *x += amt,
            Direction::D => *y -= amt,
            Direction::L => *x -= amt,
        }
    }
}

impl From<&str> for Direction {
    fn from(other: &str) -> Self {
        match other {
            "U" => Direction::U,
            "R" => Direction::R,
            "D" => Direction::D,
            "L" => Direction::L,
            _ => panic!("Invalid direction! Not [URDL]."),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    x: i32,
    y: i32,
    d: Direction,
    amt: i32,
}

impl Move {
    fn new(x: i32, y: i32, d: Direction, amt: i32) -> Self {
        Self {
            x,
            y,
            d,
            amt
        }
    }

    fn intersect(&self, other: &Move) -> Option<(i32, i32)> {
        let mut m1 = *self;
        let mut m2 = *other;
        if self.d == other.d {
            if self.d.is_vertical() && self.x == other.x {
                // Guarantee that m1.y <= m2.y
                if m1.y > m2.y {
                    mem::swap(&mut m1, &mut m2);
                }
                if m1.d == Direction::U && m1.y + m1.amt >= m2.y {
                    if m2.y <= 0 && m1.y + m1.amt >= 0 {
                        Some((m1.x, 0))
                    } else {
                        Some((m1.x, (m1.y + m1.amt).abs().min(m2.y.abs())))
                    }
                } else {
                    None
                }
            } else if self.d.is_horizontal() && self.y == other.y {
                // Guarantee that m1.x <= m2.x
                if m1.x > m2.x {
                    mem::swap(&mut m1, &mut m2);
                }
                if m1.d == Direction::R && m1.x + m1.amt >= m2.x {
                    if m2.x <= 0 && m1.x + m1.amt >= 0 {
                        Some((0, m1.y))
                    } else {
                        Some(((m1.x + m1.amt).abs().min(m2.x.abs()), m1.y))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            if m1.x > m2.x {
                mem::swap(&mut m1, &mut m2);
            }
            // Possible orientations:
            //   m2
            // m1
            // and
            // m1
            //   m2
            if m2.y > m1.y { // orientation 1
                if m2.d == Direction::D && m1.d == Direction::R
                    && m2.y - m2.amt <= m1.y && m1.x + m1.amt >= m2.x
                {
                    Some((m2.x, m1.y))
                } else if m2.d == Direction::L && m1.d == Direction::U
                    && m2.x - m2.amt <= m1.x && m1.y + m1.amt >= m2.y
                {
                    Some((m1.x, m2.y))
                } else {
                    None
                }
            } else if m2.y < m1.y { // orientation 2
                if m2.d == Direction::U && m1.d == Direction::R
                    && m2.y + m2.amt >= m1.y && m1.x + m1.amt >= m2.x
                {
                    Some((m2.x, m1.y))
                } else if m2.d == Direction::L && m1.d == Direction::D
                    && m2.x - m2.amt <= m1.x && m1.y - m1.amt <= m2.y
                {
                    Some((m1.x, m2.y))
                } else {
                    None
                }
            } else if m1.x + m1.amt >= m2.x { // if m1.y == m2.y
                Some((m1.x, m1.y))
            } else {
                None
            }
        }
    }
}

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let mut stdin_lines = stdin_lock.lines();
    let l1 = get_points(stdin_lines.next().unwrap().unwrap());
    let l2 = get_points(stdin_lines.next().unwrap().unwrap());
    let mut abs_closest = (0, 0);
    let mut abs_closest_cost = std::i32::MAX;
    let mut signal_closest = (0, 0);
    let mut signal_closest_cost = std::i32::MAX;
    let mut l1_signal_cost = 0;
    let mut l2_signal_cost = 0;
    for m1 in l1.iter() {
        if l1_signal_cost >=  signal_closest_cost && m1.x.abs() + m1.y.abs() >= abs_closest_cost {
            continue;
        }
        for m2 in l2.iter() {
            if let Some((x, y)) = m1.intersect(m2) {
                if x | y == 0 {
                    continue;
                }
                if x.abs() + y.abs() < abs_closest_cost {
                    abs_closest_cost = x.abs() + y.abs();
                    abs_closest = (x, y);
                }
                if l1_signal_cost < signal_closest_cost && l2_signal_cost < signal_closest_cost {
                    let total = l1_signal_cost + l2_signal_cost + (m1.x - x).abs()
                        + (m1.y - y).abs() + (m2.x - x).abs() + (m2.y - y).abs();
                    if total < signal_closest_cost {
                        signal_closest_cost = total;
                        signal_closest = (x, y);
                    }
                }
            }
            l2_signal_cost += m2.amt;
        }
        l2_signal_cost = 0;
        l1_signal_cost += m1.amt;
    }
    println!("p1: {} {:?}", abs_closest_cost, abs_closest);
    println!("p2: {} {:?}", signal_closest_cost, signal_closest);
}

fn get_points(line: String) -> Vec<Move> {
    let mut moves = Vec::new();
    let move_strs = line.split(',').collect::<Vec<_>>();
    let mut x = 0;
    let mut y = 0;
    for m in move_strs {
        let (d, amt) = m.split_at(1);
        let amt = amt.parse::<i32>().unwrap();
        let d = d.into();
        let new_move = Move::new(x, y, d, amt);
        moves.push(new_move);
        d.walk(&mut x, &mut y, amt);
    }
    moves
}