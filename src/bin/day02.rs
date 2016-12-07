const DATA: &'static str = include_str!("day02.txt");

extern crate nalgebra;
use self::nalgebra::Vector2;
extern crate itertools;

static KEYPAD1: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

static KEYPAD2: [[char; 5]; 5] = [['-', '-', '1', '-', '-'],
                                  ['-', '2', '3', '4', '-'],
                                  ['5', '6', '7', '8', '9'],
                                  ['-', 'A', 'B', 'C', '-'],
                                  ['-', '-', 'D', '-', '-']];

fn to_direction(from: char) -> Vector2<i32> {
    match from {
        'U' => Vector2::new(0, -1),
        'D' => Vector2::new(0, 1),
        'L' => Vector2::new(-1, 0),
        'R' => Vector2::new(1, 0),
        _ => panic!("dont know what to do with {:?}", from),
    }
}

fn get_keycode(start: Vector2<i32>, cmd: &str) -> Vector2<i32> {
    cmd.chars()
        .map(to_direction)
        .scan(start, |state, d| {
            let next = *state + d;
            let key = KEYPAD1.get(next.y as usize).and_then(|cs| cs.get(next.x as usize));
            if key.is_some() {
                *state = next;
            }
            Some(*state)
        })
        .last()
        .unwrap()
}

fn get_keycode2(start: Vector2<i32>, cmd: &str) -> Vector2<i32> {
    cmd.chars()
        .map(to_direction)
        .scan(start, |state, d| {
            let next = *state + d;
            let key =
                KEYPAD2.get(next.y as usize).and_then(|cs| cs.get(next.x as usize)).cloned();
            if key.is_some() && key != Some('-') {
                *state = next;
            }
            Some(*state)
        })
        .last()
        .unwrap()
}

pub fn main() {
    let start = Vector2::new(1, 1); // '5'
    let code = DATA.lines()
        .scan(start, |state, line| {
            let pos = get_keycode(*state, line);
            *state = pos;
            Some(KEYPAD1[pos.y as usize][pos.x as usize])
        })
        .collect::<String>();
    println!("{:?}", code);

    let start = Vector2::new(2, 0); // '5'
    let code = DATA.lines()
        .scan(start, |state, line| {
            let pos = get_keycode2(*state, line);
            *state = pos;
            Some(KEYPAD2[pos.y as usize][pos.x as usize])
        })
        .collect::<String>();
    println!("{:?}", code);
}

#[cfg(test)]
mod test {
    #[test]
    fn examples() {}
}
