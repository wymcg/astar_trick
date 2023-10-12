use crate::state::PluginState;
use std::ops::DerefMut;

use extism_pdk::*;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::util::euclidian_distance;

// Color constants
const WALL_COLOR: [u8; 4] = [255; 4];
const NONE_COLOR: [u8; 4] = [0; 4];
const START_COLOR: [u8; 4] = [0, 255, 0, 255];
const GOAL_COLOR: [u8; 4] = [0, 0, 255, 255];
const VISITED_COLOR: [u8; 4] = [255, 0, 0, 255];
const ENQUEUED_COLOR: [u8; 4] = [255, 0, 255, 255];
const PATH_COLOR: [u8; 4] = [0, 64, 0, 64];

// Scaling value for Euclidian distance
const DISTANCE_SCALE: f32 = 10_000.0;

lazy_static! {
    static ref STATE: Arc::<Mutex::<PluginState>> = Arc::new(Mutex::new(PluginState::default()));
}

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // Get the width and height from the config
    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();

    // Generate the maze
    let mut state = STATE.lock().unwrap();
    let state = state.deref_mut();
    state.generate_maze(width, height, None);

    // Set up the A* algorithm
    state.queue.push(state.maze.start, ((1.0 / euclidian_distance(state.maze.start, state.maze.goal)) * DISTANCE_SCALE) as u32);
    state.previous.insert(state.maze.start, None);

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8;4]>>>>> {
    // Get the width and height from the config
    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();

    // Grab our  plugin state
    let mut state = STATE.lock().unwrap();
    let state = state.deref_mut();

    // Make a colorless matrix
    let mut matrix_state: Vec<Vec<[u8; 4]>> = vec![vec![NONE_COLOR; width]; height];

    // Draw the walls
    for (x, y) in &state.maze.wall_coords {
        matrix_state[*y][*x] = WALL_COLOR;
    }

    // Draw visited sections
    for (x, y) in &state.visited {
        matrix_state[*y][*x] = VISITED_COLOR;
    }

    // Draw enqueued sections
    for ((x, y), _p) in &state.queue {
        matrix_state[*y][*x] = ENQUEUED_COLOR;
    }

    // Draw the start and the goal
    matrix_state[state.maze.start.1][state.maze.start.0] = START_COLOR;
    matrix_state[state.maze.goal.1][state.maze.goal.0] = GOAL_COLOR;

    if !state.visited.contains(&state.maze.goal) {
        // If the goal has not been found, do a step of the BFS

        // Pull the next coordinate from the queue
        match state.queue.pop() {
            None => {}
            Some(c) => {
                let c = c.0;

                // Mark the coordinate as visited
                state.visited.insert(c);

                // Add unvisited neighbors to the queue
                let candidates = [
                    (c.0 as i32 + 1, c.1 as i32),
                    (c.0 as i32 - 1, c.1 as i32),
                    (c.0 as i32, c.1 as i32 + 1),
                    (c.0 as i32, c.1 as i32 - 1),
                ];
                let unvisited_neighbors = candidates.iter()
                    .filter(|c| 0 <= c.0)
                    .filter(|c| c.0 < width as i32)
                    .filter(|c| 0 <= c.1)
                    .filter(|c| c.1 < height as i32)
                    .map(|c| (c.0 as usize, c.1 as usize))
                    .filter(|c| !state.maze.wall_coords.contains(c))
                    .filter(|c| !state.visited.contains(c));

                for new_c in unvisited_neighbors {
                    state.queue.push(new_c, ((1.0 / euclidian_distance(new_c, state.maze.goal)) * DISTANCE_SCALE) as u32);
                    state.previous.insert(new_c, Some(c));
                }
            }
        }
    } else {
        // If the goal has been found, paint the path from the goal
        let mut current = state.maze.goal;
        loop {
            let previous: (usize, usize) = match state.previous[&current] {
                None => {break;}
                Some(c) => c
            };

            if previous == state.maze.start {break;}

            matrix_state[previous.1][previous.0] = PATH_COLOR;
            current = previous;

        }
    }

    Ok(Json(Some(matrix_state)))
}
