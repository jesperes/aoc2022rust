type ChamberCoord = (i32, i32);
type RockCoord = (i32, i32);
type Delta = (i32, i32);

#[derive(Copy, Clone)]
struct Jet {
    dx: i32,
}

impl Jet {
    fn new(c: char) -> Self {
        assert!(c != '\n');
        Jet {
            dx: if c == '<' { -1 } else { 1 },
        }
    }
}

struct Rock {
    coords: Vec<RockCoord>,
    left_edge: i32,
    bottom_edge: i32,
}

impl Rock {
    fn new(shape: usize, bottom_edge: i32) -> Self {
        let coords = match shape {
            0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            1 => vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
            2 => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            4 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            _ => panic!(),
        };

        Rock {
            coords,
            left_edge: 2,
            bottom_edge,
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
            Jet::new(c as char)
        }
    }

    fn next_rock(&mut self) -> Rock {
        const NUM_SHAPES: usize = 5;
        let next_shape = self.shape_index % NUM_SHAPES;
        self.shape_index += 1;
        Rock::new(next_shape, self.height + 3)
    }

    // fn maybe_move_sideways(&self, rock: &mut Rock, jet: Jet) {
    //     ()
    // }

    // fn maybe_drop(&self, rock: &mut Rock) -> bool {
    //     true
    // }

    fn print(&self, falling_rock: &Rock) {
        // println!("Height: {}", self.height);
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
    // let p1: i32;
    // let shapes = vec![
    //     (vec!["..####."], 4, 1),
    //     (vec!["...#...", "..###..", "...#..."], 3, 3),
    //     (vec!["....#..", "....#..", "..###.."], 3, 3),
    //     (vec!["..#....", "..#....", "..#....", "..#...."], 1, 4),
    //     (vec!["..##...", "..##..."], 2, 2),
    // ];
    // let wall_line = "#.......#";
    let mut chamber = Chamber::new(jets);
    let next_rock = chamber.next_rock();
    chamber.print(&next_rock);
    let next_rock = chamber.next_rock();
    chamber.print(&next_rock);
    let next_rock = chamber.next_rock();
    chamber.print(&next_rock);
    let next_rock = chamber.next_rock();
    chamber.print(&next_rock);
    let next_rock = chamber.next_rock();
    chamber.print(&next_rock);

    // loop {
    //     // if chamber.num_dropped_rocks == 2022 {
    //     //     p1 = chamber.height;
    //     // }

    //     //    let mut rock = chamber.next_rock();

    //     loop {
    //         //  let jet = chamber.next_jet();

    //         // chamber.maybe_move_sideways(&mut rock, jet);
    //         chamber.print();

    //         panic!();
    //         // if chamber.maybe_drop(&mut rock) {
    //         //     // stopped
    //         // } else {
    //         //     continue 'next_jet;
    //         // }
    //     }
    // }
    //
    // (p1, 0);
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
