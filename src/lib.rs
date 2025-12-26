use rand;

type LinearSandBox<const SIZE: usize> = [Pile; SIZE];
type SandBox<const SIZE: usize> = [LinearSandBox<SIZE>; SIZE];

pub fn run<const SIZE: usize>() {
    let mut sim = Simulation {
        sand_box: [[Pile::new(); SIZE]; SIZE],
        iterations: 2000,
    };
    sim.simulate();
}

struct Simulation<const SIZE: usize> {
    sand_box: SandBox<SIZE>,
    iterations: u32,
}

impl<const SIZE: usize> Simulation<SIZE> {
    fn step(&mut self) {
        add_grain(&mut self.sand_box, (16,16));
        stabilize(&mut self.sand_box);
    }
    fn simulate(&mut self) {
        for _i in 0..self.iterations {
            write_sand_box_to_file(&self.sand_box).unwrap();
            self.step()
        }
    }
}

fn stabilize<const SIZE: usize>(sand_box: &mut SandBox<SIZE>) {
    loop {
        let current_state = is_stable(sand_box);
        match current_state {
            State::Stable => break,
            State::Unstable(to_topple) => topple::<SIZE>(sand_box, to_topple),
        }
    }
}

fn add_random_grain<const SIZE: usize>(sand_box: &mut SandBox<SIZE>) {
    let i = rand::random_range(0..SIZE);
    let j = rand::random_range(0..SIZE);
    add_grain(sand_box, (i, j));
}

fn add_grain<const SIZE: usize>(sand_box: &mut SandBox<SIZE>, to: (usize, usize)) {
    sand_box[to.0][to.1].add_grain()
}

fn topple<const SIZE: usize>(sand_box: &mut SandBox<SIZE>, to_topple: (usize, usize)) {
    let i = to_topple.0;
    let j = to_topple.1;

    // substract 4 grains from unstabillity
    sand_box[i][j].drop_grains();

    // add 1 to neighbours
    if i + 1 < SIZE {
        sand_box[i + 1][j].add_grain();
    }
    if i >= 1 {
        sand_box[i - 1][j].add_grain();
    }
    if j + 1 < SIZE {
        sand_box[i][j + 1].add_grain();
    }
    if j >= 1 {
        sand_box[i][j - 1].add_grain();
    }
}

fn is_stable<const SIZE: usize>(sand_box: &SandBox<SIZE>) -> State {
    for i in 0..SIZE {
        for j in 0..SIZE {
            if !sand_box[i][j].is_stable() {
                return State::Unstable((i, j));
            }
        }
    }
    State::Stable
}

#[derive(PartialEq, Debug)]
enum State {
    Stable,
    Unstable((usize, usize)),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pile {
    height: u8,
}

impl Pile {
    fn new() -> Pile {
        Pile { height: 0 }
    }
    fn is_stable(&self) -> bool {
        self.height < 4
    }
    fn add_grain(&mut self) {
        self.height = self.height + 1;
    }
    fn drop_grains(&mut self) {
        self.height = self.height - 4;
    }
}

fn write_sand_box_to_file<const SIZE: usize>(sand_box: &SandBox<SIZE>) -> Result<(), &'static str> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("sandbox.log")
        .map_err(|_| "cannot open or create output file")?;

    writeln!(file, "---- SAND BOX STATE ----").map_err(|_| "write error")?;

    for i in 0..SIZE {
        let mut line = String::with_capacity(SIZE);
        for j in 0..SIZE {
            let ch = char::from_digit(sand_box[i][j].height as u32, 10).unwrap();
            line.push(ch);
        }
        writeln!(file, "{}", line).map_err(|_| "write error")?;
    }
    Ok(())
}

// ------------------- TESTS ----------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_3BY3: SandBox<3> = [[Pile { height: 0 }; 3]; 3];

    const ONE_GRAIN_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 1 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
    ];

    const UNSTABLE_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 4 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
    ];

    const TOPPLED_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 0 }],
        [Pile { height: 1 }, Pile { height: 0 }, Pile { height: 1 }],
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 0 }],
    ];

    const BORDER_UNSTABLE_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 4 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
    ];

    const BORDER_TOPPLED_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 1 }],
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 1 }],
    ];

    const CORNER_UNSTABLE_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 4 }],
    ];

    const CORNER_TOPPLED_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 1 }],
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 0 }],
    ];

    const CASCADE_UNSTABLE_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 4 }, Pile { height: 3 }],
        [Pile { height: 0 }, Pile { height: 0 }, Pile { height: 0 }],
    ];

    const CASCADE_STABILIZED_3BY3: SandBox<3> = [
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 1 }],
        [Pile { height: 1 }, Pile { height: 1 }, Pile { height: 0 }],
        [Pile { height: 0 }, Pile { height: 1 }, Pile { height: 1 }],
    ];

    #[test]
    fn empty_pile() {
        let pile = Pile::new();
        assert!(pile.height == 0);
    }

    #[test]
    fn new_pile_plus_grain() {
        let mut pile = Pile::new();
        pile.add_grain();
        assert!(pile.height == 1);
    }

    #[test]
    fn state() {
        assert_eq!(is_stable(&EMPTY_3BY3), State::Stable);
        assert_eq!(is_stable(&UNSTABLE_3BY3), State::Unstable((1, 1)));
    }

    #[test]
    fn dropping_one_grain() {
        let mut sand_box = EMPTY_3BY3;
        add_grain(&mut sand_box, (1, 2));
        assert_eq!(sand_box, ONE_GRAIN_3BY3);
    }

    #[test]
    fn topple_3by3() {
        let mut sand_box = UNSTABLE_3BY3;
        topple(&mut sand_box, (1, 1));
        assert_eq!(sand_box, TOPPLED_3BY3, "CENTER TOPPLE");

        let mut sand_box = BORDER_UNSTABLE_3BY3;
        topple(&mut sand_box, (1, 2));
        assert_eq!(sand_box, BORDER_TOPPLED_3BY3, "BORDER TOPPLE");

        let mut sand_box = CORNER_UNSTABLE_3BY3;
        topple(&mut sand_box, (2, 2));
        assert_eq!(sand_box, CORNER_TOPPLED_3BY3, "CORNER TOPPLE");
    }

    #[test]
    fn stabilize_3by3() {
        let mut sand_box = UNSTABLE_3BY3;
        stabilize(&mut sand_box);
        assert_eq!(sand_box, TOPPLED_3BY3, "CENTER TOPPLE");

        let mut sand_box = BORDER_UNSTABLE_3BY3;
        stabilize(&mut sand_box);
        assert_eq!(sand_box, BORDER_TOPPLED_3BY3, "BORDER TOPPLE");

        let mut sand_box = CORNER_UNSTABLE_3BY3;
        stabilize(&mut sand_box);
        assert_eq!(sand_box, CORNER_TOPPLED_3BY3, "CORNER TOPPLE");

        let mut sand_box = CASCADE_UNSTABLE_3BY3;
        stabilize(&mut sand_box);
        assert_eq!(sand_box, CASCADE_STABILIZED_3BY3, "CORNER TOPPLE");
    }
}
