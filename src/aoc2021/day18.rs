use std::str::Chars;

enum Pair {
    Regular(isize),
    NestedPair(Box<Pair>, Box<Pair>),
}

impl Pair {
    fn from_expr(source: &str) -> Option<Self> {
        let mut source_it = source.chars();
        Self::parse_pair_item(&mut source_it)
    }

    fn parse_pair_item(source: &mut Chars) -> Option<Pair> {
        let next_char = source.next()?;
        if next_char == '[' {
            let first = Box::new(Self::parse_pair_item(source)?);
            let next_char = source.next()?;
            assert_eq!(next_char, ',');
            let second = Box::new(Self::parse_pair_item(source)?);
            let next_char = source.next()?;
            assert_eq!(next_char, ']');
            Some(Pair::NestedPair(first, second))
        } else if let Some(n) = next_char.to_digit(10) {
            Some(Pair::Regular(n as isize))
        } else {
            None
        }
    }

    fn magnitude(&self) -> u64 {
        match self {
            Pair::Regular(n) => *n as u64,
            Pair::NestedPair(p1, p2) => 3 * p1.magnitude() + 2 * p2.magnitude(),
        }
    }

    // fn add(p1: Self, p2: &Self) -> Self {
    //     let p = Pair::NestedPair(
    //         Box::new(p1),
    //         Box::new(p2),
    //     );
    //     //p.reduce();
    //     p
    // }

    // fn reduce(&mut self) -> Self {
    //     self.check_for_explosions(4);
    // }

    // fn check_for_explosions(&self, levels: usize) -> Option<&Pair> {
    //     if levels == 0 {
    //         Some(self)
    //     } else {
    //         match self {
    //             Pair::Regular(_) => None,
    //             Pair::NestedPair(p1, p2) => {
    //                 p1.check_for_explosions(levels - 1).or(
    //                     p2.check_for_explosions(levels - 1)
    //                 )
    //             }
    //         }
    //     }
    // }

    // fn split(n: isize) -> Self {
    //     Pair::NestedPair(
    //         Box::new(Pair::Regular(n / 2)),
    //         Box::new(Pair::Regular(n - (n / 2))),
    //     )
    // }
}

pub fn main(input: String) -> (u64, u64) {
    let mut pairs = Vec::with_capacity(input.lines().count());
    for expr in input.lines() {
        pairs.push(Pair::from_expr(expr).unwrap());
    }

    (pairs[0].magnitude(), 0)
}

// #[test]
// fn test() {
//     let input = "target area: x=20..30, y=-10..-5";
//     let r = main(input.into());
//     assert_eq!(r.0, 0);
//     //assert_eq!(r.1, 0);
// }

#[test]
fn parse_and_mag_test() {
    assert_eq!(
        Pair::from_expr("[[1,2],[[3,4],5]]").unwrap().magnitude(),
        143
    );
    assert_eq!(
        Pair::from_expr("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
            .unwrap()
            .magnitude(),
        1384
    );
    assert_eq!(
        Pair::from_expr("[[[[1,1],[2,2]],[3,3]],[4,4]]")
            .unwrap()
            .magnitude(),
        445
    );
    assert_eq!(
        Pair::from_expr("[[[[3,0],[5,3]],[4,4]],[5,5]]")
            .unwrap()
            .magnitude(),
        791
    );
    assert_eq!(
        Pair::from_expr("[[[[5,0],[7,4]],[5,5]],[6,6]]")
            .unwrap()
            .magnitude(),
        1137
    );
    assert_eq!(
        Pair::from_expr("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            .unwrap()
            .magnitude(),
        3488
    );
}
