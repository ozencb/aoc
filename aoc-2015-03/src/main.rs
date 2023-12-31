use std::{collections::HashSet, fs};

#[derive(Eq, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::hash::Hash for Coordinate {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        state.write_i32(self.x);
        state.finish();
    }
}

fn main() {
    let filename = "data.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong");
    let directions = contents.chars();

    let starting_coordinates = Coordinate { x: 0, y: 0 };

    let mut visited_coordinates: HashSet<Coordinate> = HashSet::from([starting_coordinates]);
    let mut santa_coordinate = starting_coordinates;
    let mut robot_coordinate = starting_coordinates;

    for (idx, direction) in directions.enumerate() {
        if idx % 2 == 0 {
            santa_coordinate = move_character(direction, santa_coordinate);
            visited_coordinates.insert(santa_coordinate);
        } else {
            robot_coordinate = move_character(direction, robot_coordinate);
            visited_coordinates.insert(robot_coordinate);
        }
    }

    println!("{:?}", visited_coordinates.into_iter().count())
}

fn move_character(direction: char, current_coordinate: Coordinate) -> Coordinate {
    match direction {
        '^' => Coordinate {
            x: current_coordinate.x,
            y: current_coordinate.y + 1,
        },
        '>' => Coordinate {
            x: current_coordinate.x + 1,
            y: current_coordinate.y,
        },
        'v' => Coordinate {
            x: current_coordinate.x,
            y: current_coordinate.y - 1,
        },
        '<' => Coordinate {
            x: current_coordinate.x - 1,
            y: current_coordinate.y,
        },
        _ => unreachable!(),
    }
}
