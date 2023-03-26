use std::borrow::BorrowMut;

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

    // If applying a given delta, does this rock cover the specified
    // chamber coordinates?
    fn covers(&self, chamber_coord: ChamberCoord, delta: Delta) -> bool {
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
    num_dropped_rocks: i32,
}

impl Chamber {
    fn new(jets: &[u8]) -> Self {
        Chamber {
            jets: jets.to_vec(),
            jet_index: 0,
            shape_index: 0,
            height: 0,
            num_dropped_rocks: 0,
        }
    }

    fn next_jet(&mut self) -> Jet {
        let c = self.jets[self.jet_index];
        if c as char == '\n' {
            self.jet_index = 0;
            Jet::new(self.jets[0] as char)
        } else {
            self.jet_index += 1;
            Jet::new(c as char)
        }
    }

    fn next_rock(&mut self) -> Rock {
        const NUM_SHAPES: usize = 5;
        let next_shape = self.shape_index % NUM_SHAPES;
        self.shape_index += 1;
        Rock::new(next_shape, self.height + 3)
    }

    fn maybe_move_sideways(&self, rock: &mut Rock, jet: Jet) {
        rock.left_edge = (rock.left_edge + jet.dx).min(7 - rock.width).max(0);
    }

    fn maybe_drop(&self, rock: &mut Rock) -> bool {
        if rock.bottom_edge >= 1 {
            rock.bottom_edge -= 1;
            return true;
        } else {
            return false;
        }
    }

    fn print(&self, falling_rock: &Rock) {
        println!();

        for y in (-1..=self.height + 7).rev() {
            match y {
                -1 => println!("     +-------+"),
                y => {
                    print!("{:4} |", y);
                    for x in 0..7 {
                        let chamber_coord = (x, y);
                        if falling_rock.covers(chamber_coord, (0, 0)) {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                    println!("|");
                }
            }
        }
    }
}

pub fn solve() -> (i32, i64) {
    let jets = include_bytes!("../inputs/input17.txt");
    // let jets = "<<<<".as_bytes();
    let mut chamber = Chamber::new(jets);

    let mut rock = chamber.next_rock();
    println!("Rock begins falling:");
    chamber.print(&rock);

    loop {
        let jet = chamber.next_jet();
        println!("Next jet: {:?}", jet);
        println!("Falling rock: {:?}", rock);

        chamber.maybe_move_sideways(&mut rock, jet);

        if chamber.maybe_drop(&mut rock) {
            println!("Dropped!");
        } else {
            println!("Stopped!");
            chamber.print(&rock);
            break;
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn move_rock() {
        // let mut rock = Rock::new(vec![0b00111, 0b00100], 3, 2);
        // let expected = Rock::new(vec![0b000111, 0b000100], 3, 2);

        // rock.move_rock(1);
        // assert_eq!(expected, rock);
    }
}
