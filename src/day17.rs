use std::collections::HashMap;

type ChamberCoord = (i32, i32);
type RockCoord = (i32, i32);
type Delta = (i32, i32);

#[derive(Copy, Clone, Debug)]
struct Jet {
    index: usize,
    dx: i32,
}

impl Jet {
    fn new(index: usize, c: char) -> Self {
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
}

impl Rock {
    fn new(shape: usize, bottom_edge: i32) -> Self {
        let (coords, width) = match shape {
            0 => (vec![(0, 0), (1, 0), (2, 0), (3, 0)], 4),
            1 => (vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)], 3),
            2 => (vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 3),
            3 => (vec![(0, 0), (0, 1), (0, 2), (0, 3)], 1),
            4 => (vec![(0, 0), (0, 1), (1, 0), (1, 1)], 2),
            _ => panic!(),
        };

        Rock {
            coords,
            left_edge: 2,
            bottom_edge,
            width,
        }
    }

    fn rock_to_chamber_coords(&self, coord: RockCoord, delta: Delta) -> ChamberCoord {
        (
            coord.0 + self.left_edge + delta.0,
            coord.1 + self.bottom_edge + delta.1,
        )
    }
}

struct Chamber {
    jets: Vec<u8>,
    jet_index: usize,
    shape_index: usize,
    height: i32,
    tower: Vec<u8>,
}

impl Chamber {
    fn new(jets: &[u8]) -> Self {
        Chamber {
            jets: jets.to_vec(),
            jet_index: 0,
            shape_index: 0,
            height: 0,
            tower: vec![0; 16 * 1024],
        }
    }

    fn is_chamber_coord_part_of_tower(&self, chamber_coord: &ChamberCoord) -> bool {
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
        return Jet::new(index, c as char);
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
}

struct CycleTracker {
    states: HashMap<(i32, Vec<u8>), (u64, i32)>,
}

impl CycleTracker {
    fn new() -> Self {
        CycleTracker {
            states: HashMap::new(),
        }
    }

    fn add_state(&mut self, jet: &Jet, chamber: &Chamber, rock_num: u64) -> Option<(u64, i32)> {
        // Save the 15 top rows of the tower. This turns out to be the fewest
        // rows that we can save to correctly detect cycles.
        const NUM_ROWS: usize = 15;

        let mut top_rows: Vec<u8> = vec![0; NUM_ROWS];

        for i in 1..NUM_ROWS {
            let y = chamber.height as i64 - i as i64;
            if y < 0 {
                break;
            }
            top_rows.push(chamber.tower[y as usize]);
        }

        let state = (jet.index as i32, top_rows);

        if let Some((rock_num, height)) = self.states.get(&state) {
            return Some((*rock_num, *height));
        } else {
            self.states.insert(state, (rock_num, chamber.height as i32));
            return None;
        }
    }
}

pub fn solve() -> (i32, u64) {
    let jets = include_bytes!("../inputs/input17.txt");
    let mut chamber = Chamber::new(jets);
    let mut cycle_tracker = CycleTracker::new();

    // For p1, we just drop 2022 rocks
    let mut p1: i32 = 0;
    let p1_limit = 2022;

    // For p2, we drop 1 trillion rocks. We keep track of cycles
    // so that we do not need to actually drop the rocks which are part of
    // (full) cycles.
    let mut track_cycles = true;
    let mut p2_limit: u64 = 1_000_000_000_000;
    let mut cycle_height: u64 = 0;
    let mut num_cycles: u64 = 0;

    for rock_num in 0.. {
        if rock_num == p1_limit {
            p1 = chamber.height;
        } else if rock_num == p2_limit {
            let p2 = chamber.height as u64 + (num_cycles * cycle_height);
            return (p1, p2);
        }

        let mut rock = chamber.next_rock();

        loop {
            let jet = chamber.next_jet();

            chamber.maybe_move_sideways(&mut rock, jet);

            if !chamber.maybe_drop(&mut rock) {
                chamber.add_to_tower(rock);

                if track_cycles {
                    if let Some((start, height)) = cycle_tracker.add_state(&jet, &chamber, rock_num)
                    {
                        // Reduce the p2 limit by a whole number of cycles, and then calculate
                        // the total height contributed by the rocks which are part of the cycles.
                        cycle_height = (chamber.height - height) as u64;
                        let rocks_per_cycle = rock_num - start;
                        num_cycles = (p2_limit - start).div_floor(rocks_per_cycle) - 1;
                        p2_limit -= num_cycles * rocks_per_cycle;
                        track_cycles = false;
                    }
                }
                break;
            }
        }
    }
    panic!();
}
