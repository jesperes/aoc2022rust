type ChamberCoord = (i32, i32);
type RockCoord = (i32, i32);
type Delta = (i32, i32);

#[derive(Copy, Clone, Debug)]
struct Jet {
    dx: i32,
}

impl Jet {
    fn new(c: char) -> Self {
        match c {
            '>' => Jet { dx: 1 },
            '<' => Jet { dx: -1 },
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
            tower: vec![0; 4096],
        }
    }

    fn is_chamber_coord_part_of_tower(&self, chamber_coord: &ChamberCoord) -> bool {
        assert!((0..7).contains(&chamber_coord.0));
        assert!(chamber_coord.1 >= 0);
        self.tower[chamber_coord.1 as usize] & (1 << chamber_coord.0) != 0
    }

    fn next_jet(&mut self) -> Jet {
        let mut c = self.jets[self.jet_index];
        if c as char == '\n' {
            self.jet_index = 0;
            c = self.jets[0];
        }
        self.jet_index += 1;
        return Jet::new(c as char);
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

pub fn solve() -> (i32, i64) {
    let jets = include_bytes!("../inputs/input17.txt");
    // let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n".as_bytes();
    let mut chamber = Chamber::new(jets);
    const DEBUG1: bool = false;
    const DEBUG: bool = false;

    for _ in 0..2022 {
        let mut rock = chamber.next_rock();
        if DEBUG1 {
            println!("A new rock begins falling:");
            chamber.print(Some(&rock));
        }

        loop {
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
                if DEBUG {
                    println!("Rock falls 1 unit, causing it to come to rest:");
                    chamber.print(None);
                }
                break;
            }
        }
    }

    println!("Height: {}", chamber.height);
    (0, 0)
}
