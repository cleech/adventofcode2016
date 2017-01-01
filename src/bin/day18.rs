static DATA: &'static str = include_str!("day18.txt");

fn next(x: &[bool]) -> Vec<bool> {
    let mut y = Vec::with_capacity(x.len());
    for i in 0..x.len() {
        let left = x.get(i - 1).cloned().unwrap_or(true);
        // let center = x.get(i).map(|b| *b).unwrap();
        let right = x.get(i + 1).cloned().unwrap_or(true);
        // y.push(match (left, center, right) {
        //     (false, false, true) => false,
        //     (true, false, false) => false,
        //     (false, true, true) => false,
        //     (true, true, false) => false,
        //     _ => true,
        // });
        y.push(left == right);
    }
    y
}

fn main() {
    let mut row = DATA.trim()
        .chars()
        .map(|c| match c {
            '.' => true,
            '^' => false,
            _ => panic!(),
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for _ in 0..400_000 {
        count += row.iter().filter(|&&b| b).count();
        row = next(&row);
    }
    println!("{}", count);
}
