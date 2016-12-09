#[macro_use]
extern crate nom;

static DATA: &'static str = include_str!("day09.txt");

named!(numeric_string<&str>,
    map_res!(
        nom::digit,
        std::str::from_utf8
    )
);

named!(usize_digit<usize>,
    map_res!(
        numeric_string,
        std::str::FromStr::from_str
    )
);

named!(rle<(usize, usize)>,
    delimited!(
        char!('('),
        separated_pair!(
            usize_digit,
            char!('x'),
            usize_digit
        ),
        char!(')')
    )
);

fn not_openp(c: u8) -> bool { c != b'(' }

named!(uncompress<usize>,
    alt!(
        map!(take_while1!(not_openp), |s: &[u8]| s.len())   |
        chain!(
            r:  rle ~
                take!(r.0),
            ||  r.0 * r.1
        )
    )
);

named!(uncompress_all<usize>,
    fold_many1!(uncompress, 0, |mut acc, l| { acc += l; acc })
);

named!(uncompress_r<usize>,
    alt!(
        map!(take_while1!(not_openp), |s: &[u8]| s.len())   |        
        map_res!(
            chain!(
                r:  rle ~
                s:  take!(r.0),
                || (s, r.1)
            ),
            |(s, r)| uncompress_all_r(s).map(|u| u * r).to_result()
        )
    )
);

named!(uncompress_all_r<usize>,
    fold_many1!(uncompress_r, 0, |mut acc, l| { acc += l; acc })
);

fn main () {
    let input = DATA.trim().as_bytes();
    println!("{:?}", uncompress_all(input));
    println!("{:?}", uncompress_all_r(input));
}
