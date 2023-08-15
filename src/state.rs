use std::collections::{HashMap, HashSet};
use priority_queue::PriorityQueue;
use crate::generate_maze_matrix::{generate_maze_matrix, MatrixMaze};

pub struct PluginState {
    pub maze: MatrixMaze,                                           // Information about the maze
    pub visited: HashSet<(usize, usize)>,                           // Set of visited coords
    pub queue: PriorityQueue<(usize, usize), u32>,                  // Queue for A*
    pub previous: HashMap<(usize, usize), Option<(usize, usize)>>,  // previous[p] = point used to initially traverse to p
}

impl Default for PluginState {
    fn default() -> Self {
        Self {
            maze: MatrixMaze {
                wall_coords: Default::default(),
                start: (0, 0),
                goal: (0, 0),
            },
            visited: HashSet::new(),
            queue: PriorityQueue::new(),
            previous: HashMap::new(),
        }
    }
}

impl PluginState {
    pub fn generate_maze(&mut self, matrix_width: usize, matrix_height: usize, seed: Option<[u8; 32]>) {
        self.maze = generate_maze_matrix(matrix_width, matrix_height, seed);
    }
}