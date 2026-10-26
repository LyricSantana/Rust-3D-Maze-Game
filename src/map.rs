use std::collections::VecDeque;

pub const SIZE: usize = 25;

struct SimpleRng {
    state: u64,
}

impl SimpleRng {
fn new() -> Self {
    let seed = macroquad::miniquad::date::now() as u64;
    Self { state: seed }
}

    fn next_range(&mut self, bounds: usize) -> usize {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state as usize) % bounds
    }
}


// pub fn find_endpoint(map: &[[u8; SIZE]; SIZE]) -> (usize, usize) {
//     for y in (1..SIZE - 1).rev() {
//         for x in (1..SIZE - 1).rev() {
//             if map[y][x] == 0 && (x,y) != (1,1) {
//                 return (x,y);
//             }
//         }
//     }
//     (1,1)
// }

// uses bfs (breadth first search) to make maze (thank you random youtube videos and forum posts from 5 years ago 🙏)
pub fn find_endpoint(map: &[[u8; SIZE]; SIZE]) -> (usize, usize) {
    let start = (1 as usize, 1 as usize);
    // usize MAX means wwe havent visited a cell yet
    let mut distances = [[usize::MAX; SIZE]; SIZE];
    let mut queue = VecDeque::new();

    distances[start.1][start.0] = 0;
    queue.push_back(start);

    //keep track of farthest cell so far
    let mut farthest = start;
    let mut farthest_distance = 0;
    let directions: [(isize, isize); 4] = [
        (1,0),
        (-1,0),
        (0,1),
        (0,-1),
    ];

    //continue until there are no more cells to explore
    while let Some((x,y)) = queue.pop_front() {
        let current_distance = distances[y][x];

        if current_distance > farthest_distance {
            farthest = (x,y);
            farthest_distance = current_distance;
        }

        for &(dx, dy) in &directions {
            let next_x = x as isize + dx;
            let next_y = y as isize + dy;

            if next_x < 0 || next_x >= SIZE as isize || next_y < 0 || next_y >= SIZE as isize {
                continue;
            }

            let nx = next_x as usize;
            let ny = next_y as usize;

            if map[nx][ny] == 1 {
                continue;
            }

            if distances[ny][nx] != usize::MAX {
                continue;
            }

            distances[ny][nx] = current_distance + 1;
            queue.push_back((nx, ny));
        }
    }
    farthest
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