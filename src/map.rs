use std::time::{SystemTime, UNIX_EPOCH};

pub const SIZE: usize = 25;

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new() -> Self {
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64;
        Self { state: seed }
    }

    fn next_range(&mut self, bounds: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state as usize) % bounds
    }
}

pub fn generate_maze() -> [[u8; SIZE]; SIZE] {
    let mut map = [[1 as u8; SIZE]; SIZE];
    let mut rng = SimpleRng::new();

    let start = (1 as usize, 1 as usize);
    map[start.1][start.0] = 0;

    let mut stack = vec![start];

    let directions: [(isize, isize); 4] = [
        (2,0),
        (-2,0),
        (0,2),
        (0,-2),
    ];

    while let Some(&(x, y)) = stack.last() {
        let mut neighbors = Vec::new();

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx > 0 && ny > 0 && nx < (SIZE - 1) as isize && ny < (SIZE - 1) as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if map[ny][nx] == 1 {
                    neighbors.push((nx, ny));
                }
            }
        }
        if neighbors.is_empty() {
            stack.pop();
        } else {
            let index = rng.next_range(neighbors.len());
            let (nx, ny) = neighbors[index];

            let wall_x = (x + nx) / 2;
            let wall_y = (y + ny) / 2;

            map[wall_y][wall_x] = 0;
            map[ny][nx] = 0;

            stack.push((nx, ny));
        }
    }
    map
}