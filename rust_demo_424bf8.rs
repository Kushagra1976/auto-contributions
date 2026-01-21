// Learning Objective:
// This tutorial demonstrates how to implement Conway's Game of Life in Rust
// using a struct to represent the game state and a 2D vector for the grid.
// We will focus on understanding how to manage a 2D grid, iterate over its cells,
// and apply simple rules to update the state.

// A struct to hold the entire Game of Life state.
// This encapsulates the grid and its dimensions.
struct GameOfLife {
    // A 2D vector (vector of vectors) to represent the game grid.
    // Each element will be a boolean: true for alive, false for dead.
    grid: Vec<Vec<bool>>,
    // The width of the grid.
    width: usize,
    // The height of the grid.
    height: usize,
}

impl GameOfLife {
    // A constructor function to create a new GameOfLife instance.
    // It takes the desired width and height of the grid.
    fn new(width: usize, height: usize) -> Self {
        // Initialize the grid with all cells set to dead (false).
        // `vec![vec![false; width]; height]` creates a `height` x `width`
        // grid filled with `false`.
        let grid = vec![vec![false; width]; height];
        // Return a new GameOfLife struct instance.
        GameOfLife { grid, width, height }
    }

    // A method to set a specific cell to alive.
    // `x` and `y` are the coordinates of the cell.
    fn set_alive(&mut self, x: usize, y: usize) {
        // We use `&mut self` because we are modifying the state of the struct.
        // Check if the coordinates are within the grid bounds to prevent panics.
        if x < self.width && y < self.height {
            // Set the cell at (x, y) to alive (true).
            self.grid[y][x] = true;
        }
    }

    // A method to count the number of live neighbors for a given cell.
    // This is a crucial part of the Game of Life logic.
    fn count_live_neighbors(&self, x: usize, y: usize) -> u8 {
        // Initialize a counter for live neighbors.
        let mut live_neighbors = 0;

        // Iterate over the 3x3 area around the current cell (including the cell itself).
        // `dx` and `dy` represent the offsets from the current cell's coordinates.
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Skip the current cell itself.
                if dx == 0 && dy == 0 {
                    continue;
                }

                // Calculate the neighbor's coordinates.
                // We use `checked_add` and `checked_sub` to safely handle potential
                // underflow/overflow when `x` or `y` are at the edge of the grid.
                // This returns `None` if an operation would result in an invalid index.
                let neighbor_x_opt = if dx < 0 {
                    x.checked_sub((-dx) as usize)
                } else {
                    x.checked_add(dx as usize)
                };
                let neighbor_y_opt = if dy < 0 {
                    y.checked_sub((-dy) as usize)
                } else {
                    y.checked_add(dy as usize)
                };

                // If the neighbor coordinates are valid (not `None`)...
                if let (Some(nx), Some(ny)) = (neighbor_x_opt, neighbor_y_opt) {
                    // ...and they are within the grid bounds...
                    if nx < self.width && ny < self.height {
                        // ...check if the neighbor cell is alive and increment the count.
                        if self.grid[ny][nx] {
                            live_neighbors += 1;
                        }
                    }
                }
            }
        }
        // Return the total count of live neighbors.
        live_neighbors
    }

    // A method to advance the game to the next generation.
    fn next_generation(&mut self) {
        // Create a new grid to store the state of the next generation.
        // We do this to avoid modifying the grid while we are still reading
        // from it to determine neighbor counts.
        let mut next_grid = vec![vec![false; self.width]; self.height];

        // Iterate over each cell in the current grid.
        for y in 0..self.height {
            for x in 0..self.width {
                // Count the live neighbors for the current cell.
                let live_neighbors = self.count_live_neighbors(x, y);

                // Apply the Game of Life rules:
                // 1. Any live cell with fewer than two live neighbors dies (underpopulation).
                // 2. Any live cell with two or three live neighbors lives on to the next generation.
                // 3. Any live cell with more than three live neighbors dies (overpopulation).
                // 4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction).

                let current_cell_is_alive = self.grid[y][x];

                // Rule 2 & 3: If the cell is alive...
                if current_cell_is_alive {
                    // ...it survives if it has 2 or 3 live neighbors.
                    if live_neighbors == 2 || live_neighbors == 3 {
                        next_grid[y][x] = true;
                    }
                    // Otherwise, it dies (underpopulation or overpopulation).
                } else {
                    // Rule 4: If the cell is dead and has exactly 3 live neighbors, it becomes alive.
                    if live_neighbors == 3 {
                        next_grid[y][x] = true;
                    }
                }
            }
        }
        // Update the current grid with the state of the next generation.
        self.grid = next_grid;
    }

    // A helper method to print the grid to the console.
    // This is useful for visualizing the game's progress.
    fn print(&self) {
        for row in &self.grid {
            for &cell in row {
                // Print '█' for alive cells and ' ' for dead cells.
                print!("{}", if cell { '█' } else { ' ' });
            }
            // Move to the next line after printing a row.
            println!();
        }
        // Add an extra newline for better separation between generations.
        println!();
    }
}

// Example Usage:
fn main() {
    // Create a new Game of Life instance with a 10x5 grid.
    let mut game = GameOfLife::new(10, 5);

    // Set up an initial pattern (a glider).
    // A glider is a simple pattern that moves diagonally across the grid.
    game.set_alive(1, 0);
    game.set_alive(2, 1);
    game.set_alive(0, 2);
    game.set_alive(1, 2);
    game.set_alive(2, 2);

    // Print the initial state.
    println!("Initial State:");
    game.print();

    // Run the game for a few generations.
    for i in 0..5 {
        println!("Generation {}:", i + 1);
        game.next_generation();
        game.print();
    }
}