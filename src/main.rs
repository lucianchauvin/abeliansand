use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter};
use std::collections::VecDeque;

const N: usize = 2usize.pow(12) + 1;
const C: usize = 4;
const S: usize = 2usize.pow(28);

const COLORS: [[u8; 3]; 4] = [
    [  0,   0,   0],
    [255,   0,   0],
    [  0, 255,   0],
    [  0,   0, 255],
];

fn img(grid: &[[usize; N]; N]) -> std::io::Result<()> {
    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);
    writer.write_all(format!("P6\n{} {}\n255\n", N, N).as_bytes())?;
    
    for row in grid {
        for &x in row {
            if x <= 3 {
                writer.write_all(&COLORS[x])?
            } else {
                writer.write_all(&COLORS[0])?
            }
        }
    }
    writer.flush()?; 
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut grid = Box::new([[0; N]; N]);
    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    let mut in_stack = [[false; N]; N];

    let m = N / 2;

    grid[m][m] = S;
    stack.push_back((m, m));
    in_stack[m][m] = true;

    while let Some((x, y)) = stack.pop_front() {
        in_stack[x][y] = false;
        if grid[x][y] >= C {
            let to_add = grid[x][y] / C;
            grid[x][y] %= C;

            let neighbors: [(isize, isize); 4] = [
                ( 1,  0),
                (-1,  0),
                ( 0,  1),
                ( 0, -1),
            ];
            
            for (dx, dy) in neighbors.iter() {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                grid[nx][ny] += to_add;
                if ny == m || ny == nx {
                    if (nx == m && ny == nx) || (x != m && y != m) {
                        grid[nx][ny] += to_add;
                    }
                }
                
                if grid[nx][ny] >= C && ny >= m && ny <= nx && !in_stack[nx][ny] {
                    stack.push_back((nx, ny));
                    in_stack[nx][ny] = true;
                }
            }
        }
    }


    for x in m..N {
        for y in m..=x {
            let val = grid[x][y];
            if val == 0 { continue; }

            let dx = x - m;
            let dy = y - m;

            let points = [
                (m + dx, m + dy), (m + dx, m - dy),
                (m - dx, m + dy), (m - dx, m - dy),
                (m + dy, m + dx), (m + dy, m - dx),
                (m - dy, m + dx), (m - dy, m - dx),
            ];

            for (px, py) in points {
                grid[px][py] = val;
            }
        }
    }

    img(&grid)
}
