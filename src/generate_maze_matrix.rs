use std::collections::HashSet;
use maze_generator::prelude::*;
use maze_generator::growing_tree::GrowingTreeGenerator;

pub struct MatrixMaze {
    pub(crate) wall_coords: HashSet<(usize, usize)>,    // Coordinates of all walls
    pub(crate) start: (usize, usize),                   // Coordinate of start
    pub(crate) goal: (usize, usize),                    // Coordinate of goal
}

/// Generate a maze.
///
/// # Arguments
/// * `matrix_width` - Width of the matrix, in number of LEDs
/// * `matrix_height` - Height of the matrix, in number of LEDs
///
///
pub fn generate_maze_matrix(matrix_width: usize, matrix_height: usize, seed: Option<[u8; 32]>) -> MatrixMaze {
    let mut out = MatrixMaze {
        wall_coords: HashSet::new(),
        start: (0, 0),
        goal: (0, 0),
    };

    // Figure out the maze dimensions
    let (maze_width, maze_height) = (matrix_width / 2, matrix_height / 2);

    // Generate the maze
    let mut generator = GrowingTreeGenerator::new(seed);
    let maze = generator.generate(maze_width as i32, maze_height as i32).unwrap();
    let maze_string = format!("{:?}", maze);

    // Parse the string representation of the maze
    for (y, row) in maze_string.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if (x >= matrix_width) || (y >=  matrix_height) {continue;}

            match c {
                'S' => {out.start = (x, y);},
                'G' => {out.goal = (x, y);},
                ' ' => {/* do nothing */}
                _ => {out.wall_coords.insert((x, y));}
            }
        }
    }

    out
}