use std::collections::HashMap;

use num::integer::div_floor;

type ChamberCoord = (i32, i32);
type RockCoord = (i32, i32);
type Delta = (i32, i32);

#[derive(Copy, Clone, Debug)]
struct Jet {
    index: i32,
    dx: i32,
}

impl Jet {
    fn new(index: i32, c: char) -> Self {
        match c {
            '>' => Jet { index, dx: 1 },
            '<' => Jet { index, dx: -1 },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Rock {
    coords: Vec<RockCoord>,
    left_edge: i32,
    bottom_edge: i32,
    width: i32,
    height: i32,
}

impl Rock {
    fn new(shape: usize, bottom_edge: i32) -> Self {
        let (coords, width, height) = match shape {
            0 => (vec![(0, 0), (1, 0), (2, 0), (3, 0)], 4, 1),
            1 => (vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)], 3, 3),
            2 => (vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3, 3),
            3 => (vec![(0, 0), (0, 1), (0, 2), (0, 3)], 1, 4),
            4 => (vec![(0, 0), (0, 1), (1, 0), (1, 1)], 2, 2),
            _ => panic!(),
        };

        Rock {
            coords,
            left_edge: 2,
            bottom_edge,
            width,
            height,
        }
    }

    fn chamber_to_rock_coords(&self, coord: ChamberCoord) -> RockCoord {
        (coord.0 - self.left_edge, coord.1 - self.bottom_edge)
    }

    fn rock_to_chamber_coords(&self, coord: RockCoord, delta: Delta) -> ChamberCoord {
        (
            coord.0 + self.left_edge + delta.0,
            coord.1 + self.bottom_edge + delta.1,
        )
    }

    // If applying a given delta, does this rock cover the specified
    // chamber coordinates?
    fn covers(&self, chamber_coord: ChamberCoord) -> bool {
        let transformed_chamber_coord: RockCoord = self.chamber_to_rock_coords(chamber_coord);
        self.coords
            .iter()
            .any(|rock_coord| *rock_coord == transformed_chamber_coord)
    }
}

struct Chamber {
    jets: Vec<u8>,
    jet_index: usize,
    shape_index: usize,
    height: i32,
    // num_dropped_rocks: i32,
    tower: Vec<u8>,
}

impl Chamber {
    fn new(jets: &[u8]) -> Self {
        Chamber {
            jets: jets.to_vec(),
            jet_index: 0,
            shape_index: 0,
            height: 0,
            // num_dropped_rocks: 0,
            tower: vec![0; 16 * 1024],
        }
    }

    fn is_chamber_coord_part_of_tower(&self, chamber_coord: &ChamberCoord) -> bool {
        assert!((0..7).contains(&chamber_coord.0));
        assert!(chamber_coord.1 >= 0);
        self.tower[chamber_coord.1 as usize] & (1 << chamber_coord.0) != 0
    }

    fn next_jet(&mut self) -> Jet {
        let mut c = self.jets[self.jet_index];
        let mut index = self.jet_index;

        if c as char == '\n' {
            self.jet_index = 0;
            c = self.jets[0];
            index = 0;
        }

        self.jet_index += 1;
        return Jet::new(index as i32, c as char);
    }

    fn next_rock(&mut self) -> Rock {
        const NUM_SHAPES: usize = 5;
        let next_shape = self.shape_index % NUM_SHAPES;
        self.shape_index += 1;
        Rock::new(next_shape, self.height + 3)
    }

    fn maybe_move_sideways(&self, rock: &mut Rock, jet: Jet) -> bool {
        if self.is_valid_rock_position(rock, (jet.dx, 0)) {
            rock.left_edge = (rock.left_edge + jet.dx).min(7 - rock.width).max(0);
            return true;
        } else {
            return false;
        }
    }

    fn maybe_drop(&self, rock: &mut Rock) -> bool {
        if self.is_valid_rock_position(rock, (0, -1)) {
            rock.bottom_edge -= 1;
            return true;
        } else {
            return false;
        }
    }

    fn is_valid_rock_position(&self, rock: &Rock, delta: Delta) -> bool {
        for rock_coord in &rock.coords {
            let chamber_coord = rock.rock_to_chamber_coords(*rock_coord, delta);

            if chamber_coord.0 < 0
                || chamber_coord.0 >= 7
                || chamber_coord.1 < 0
                || self.is_chamber_coord_part_of_tower(&chamber_coord)
            {
                return false;
            }
        }

        true
    }

    // Rock is consumed here, as it becomes part of the tower
    fn add_to_tower(&mut self, rock: Rock) {
        for rock_coord in &rock.coords {
            let chamber_coord = rock.rock_to_chamber_coords(*rock_coord, (0, 0));
            self.tower[chamber_coord.1 as usize] |= 1 << chamber_coord.0;
            self.height = self.height.max(chamber_coord.1 + 1);
        }
    }

    fn print(&self, falling_rock: Option<&Rock>) {
        let top = if let Some(rock) = falling_rock {
            rock.bottom_edge + rock.height - 1
        } else {
            self.height - 1
        };

        // println!("tower: {:?}", self.tower);

        for y in (-1..=top).rev() {
            match y {
                -1 => println!("   +-------+"),
                y => {
                    print!("{:3}|", y);
                    for x in 0..7 {
                        let chamber_coord = (x, y);
                        if self.is_chamber_coord_part_of_tower(&chamber_coord) {
                            print!("#");
                        } else {
                            if let Some(rock) = falling_rock {
                                if rock.covers(chamber_coord) {
                                    print!("@");
                                } else {
                                    print!(".");
                                }
                            } else {
                                print!(".");
                            }
                        }
                    }
                    println!("|");
                }
            }
        }
        println!()
    }
}

struct CycleTracker {
    states: HashMap<Vec<i32>, (i64, i32)>,
}

impl CycleTracker {
    fn new() -> Self {
        CycleTracker {
            states: HashMap::new(),
        }
    }

    fn add_state(&mut self, jet: &Jet, chamber: &Chamber, rock_num: i64) -> Option<(i64, i32)> {
        let mut state: Vec<i32> = Vec::new();
        state.push(jet.index);
        for i in 1..20 {
            let y = chamber.height - i;
            if y < 0 {
                break;
            }
            state.push(chamber.tower[y as usize] as i32);
        }

        println!("State: {:?}", state);

        if let Some((rock_num, height)) = self.states.get(&state) {
            return Some((*rock_num, *height));
        } else {
            self.states.insert(state, (rock_num, chamber.height));
            return None;
        }
    }
}

pub fn solve() -> (i32, i64) {
    let jets = include_bytes!("../inputs/input17.txt");
    // let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n".as_bytes();
    let mut chamber = Chamber::new(jets);
    const DEBUG1: bool = false;
    const DEBUG: bool = false;
    let mut cycle_tracker = CycleTracker::new();

    let mut p1: i32 = 0;
    let mut p2: i64 = 0;
    let p1_limit = 2022;

    // When we discover a cycle, we will compute the length of the cycle, and reduce
    // the number of iterations we need to do.
    let mut p2_limit: i64 = 1_000_000_000_000;
    let mut rock_num: i64 = 0;

    let mut height_of_all_cycles: i64 = 0;

    'outer: loop {
        println!("Rock num: {rock_num}");

        if rock_num == p1_limit {
            p1 = chamber.height;
        }

        if rock_num == p2_limit {
            p2 = chamber.height as i64 + height_of_all_cycles;
            break 'outer;
        }

        let mut rock = chamber.next_rock();
        if DEBUG1 {
            println!("A new rock begins falling:");
            chamber.print(Some(&rock));
        }

        'inner: loop {
            let jet = chamber.next_jet();

            if chamber.maybe_move_sideways(&mut rock, jet) {
                if DEBUG {
                    println!(
                        "Jet of gas pushes rock {}:",
                        if jet.dx == 1 { "right" } else { "left" }
                    );
                }
            } else {
                if DEBUG {
                    println!(
                        "Jet of gas pushes rock {}, but nothing happens:",
                        if jet.dx == 1 { "right" } else { "left" }
                    )
                }
            }

            if DEBUG {
                chamber.print(Some(&rock));
            }

            if chamber.maybe_drop(&mut rock) {
                if DEBUG {
                    println!("Rock falls 1 unit:");
                    chamber.print(Some(&rock));
                }
            } else {
                chamber.add_to_tower(rock);

                if let Some((start, height)) = cycle_tracker.add_state(&jet, &chamber, rock_num) {
                    let rock_num_at_cycle_start = start as i64;
                    let height_at_cycle_start = height as i64;
                    let cycle_height = chamber.height as i64 - height_at_cycle_start;
                    let rocks_per_cycle = rock_num as i64 - rock_num_at_cycle_start;

                    println!("Found repeating state:");
                    println!(
                        "First occurrence after dropping rock number {} (height {})",
                        rock_num_at_cycle_start, height_at_cycle_start
                    );
                    println!(
                        "Second occurence after dropping rock number {} (height {})",
                        rock_num, chamber.height
                    );
                    println!(
                        "Cycle length: {}",
                        rock_num as i64 - rock_num_at_cycle_start
                    );

                    let p2_num_rocks_to_drop: i64 = 1_000_000_000_000;
                    let number_of_rocks_left_to_drop_when_cycles_start =
                        p2_num_rocks_to_drop - rock_num_at_cycle_start;
                    let num_cycles =
                        number_of_rocks_left_to_drop_when_cycles_start.div_floor(rocks_per_cycle);

                    let rocks_part_of_cycles = num_cycles * rocks_per_cycle;
                    height_of_all_cycles = cycle_height * num_cycles;

                    println!("Reducing p2_limit by {}", rocks_part_of_cycles);
                    p2_limit -= rocks_part_of_cycles;
                }

                if DEBUG {
                    println!("Rock falls 1 unit, causing it to come to rest:");
                    chamber.print(None);
                }

                break 'inner;
            }

            rock_num += 1;
        }
    }

    (p1, p2)
}
