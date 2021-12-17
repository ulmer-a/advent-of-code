use itertools::Itertools;

type TargetRange = ((i32, i32), (i32, i32));

fn parse_target_window(s: String) -> Option<TargetRange> {
    let ranges = s.split(": x=").skip(1).next()?;
    ranges
        .split(", y=")
        .map(|range| -> (i32, i32) {
            range
                .split("..")
                .map(|n| n.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
}

struct Probe {
    velocity: (i32, i32),
    position: (i32, i32),
    target: TargetRange,
}

impl Probe {
    fn new(initial_vel: (i32, i32), target: TargetRange) -> Self {
        Probe {
            velocity: initial_vel,
            position: (0, 0),
            target,
        }
    }

    fn pos(&self) -> (i32, i32) {
        self.position
    }

    fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.velocity.0 -= if self.velocity.0 != 0 {
            self.velocity.0 / self.velocity.0.abs()
        } else {
            0
        };
        self.velocity.1 -= 1;
    }

    fn is_in_target_window(&self) -> bool {
        self.position.0 >= (self.target.0).0
            && self.position.0 <= (self.target.0).1
            && self.position.1 >= (self.target.1).0
            && self.position.1 <= (self.target.1).1
    }
}

fn reaches_target_window(vel: (i32, i32), target: TargetRange) -> bool {
    let mut probe = Probe::new(vel, target);
    let mut max_y = 0;
    for _ in 0..500 {
        probe.step();

        if probe.pos().1 > max_y {
            max_y = probe.pos().1;
        }

        if probe.is_in_target_window() {
            return true;
        }
    }
    false
}

pub fn main(input: String) -> (u64, u64) {
    let target = parse_target_window(input).unwrap();

    let y_min = (target.1).0;
    let y_max = y_min * (y_min + 1) / 2;

    let mut vel_count = 0;
    for v_x in 0..200 {
        for mut v_y in 0..400 {
            v_y -= 200;

            if reaches_target_window((v_x, v_y), target) {
                vel_count += 1;
            }
        }
    }

    (y_max as u64, vel_count)
}

#[test]
fn test() {
    let input = "target area: x=20..30, y=-10..-5";
    let r = main(input.into());
    assert_eq!(r.0, 45);
    assert_eq!(r.1, 112);
}
