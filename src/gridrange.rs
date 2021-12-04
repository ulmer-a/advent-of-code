pub struct GridRange {
    p: (i64, i64),
    count_left: usize,
    step_x: i64,
    step_y: i64,
}

impl GridRange {
    pub fn new(p1: (usize, usize), p2: (usize, usize), allow_diagonal: bool) -> GridRange {
        let mut grid_range = GridRange {
            p: (p1.0 as i64, p1.1 as i64),
            count_left: Self::step_count(p1, p2),
            step_x: Self::step(p1.0, p2.0),
            step_y: Self::step(p1.1, p2.1),
        };
        if !allow_diagonal && grid_range.step_x != 0 && grid_range.step_y != 0 {
            // if no diagonals are allowed, don't do them
            grid_range.count_left = 0;
        }
        grid_range
    }

    fn step_count(p1: (usize, usize), p2: (usize, usize)) -> usize {
        let a1 = (p1.0 as i64 - p2.0 as i64).abs() as usize;
        let a2 = (p1.1 as i64 - p2.1 as i64).abs() as usize;
        std::cmp::max(a1, a2) + 1
    }

    fn step(p1: usize, p2: usize) -> i64 {
        if p1 == p2 {
            0
        } else if p1 < p2 {
            1
        } else {
            -1
        }
    }
}

impl Iterator for GridRange {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.count_left == 0 {
            return None;
        }

        let r = self.p;
        self.p.0 += self.step_x;
        self.p.1 += self.step_y;
        self.count_left -= 1;
        Some((r.0 as usize, r.1 as usize))
    }
}

#[test]
fn diagonal_test() {
    let mut r = GridRange::new((8, 9), (6, 7), true);
    assert_eq!(r.next(), Some((8, 9)));
    assert_eq!(r.next(), Some((7, 8)));
    assert_eq!(r.next(), Some((6, 7)));
    assert_eq!(r.next(), None);
}

#[test]
fn horiz_test() {
    let mut r = GridRange::new((8, 9), (10, 9), true);
    assert_eq!(r.next(), Some((8, 9)));
    assert_eq!(r.next(), Some((9, 9)));
    assert_eq!(r.next(), Some((10, 9)));
    assert_eq!(r.next(), None);
}

#[test]
fn vert_test() {
    let mut r = GridRange::new((8, 9), (8, 6), true);
    assert_eq!(r.next(), Some((8, 9)));
    assert_eq!(r.next(), Some((8, 8)));
    assert_eq!(r.next(), Some((8, 7)));
    assert_eq!(r.next(), Some((8, 6)));
    assert_eq!(r.next(), None);
}

#[test]
fn no_diagonal_test() {
    let mut r = GridRange::new((8, 9), (6, 7), false);
    assert_eq!(r.next(), None);
}
