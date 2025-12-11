use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter};
use std::collections::VecDeque;

const N: usize = 2usize.pow(11) + 1;
const C: usize = 4;
const S: usize = 2usize.pow(30);

const COLORS: [[u8; 3]; 4] = [
    [  0,   0,   0],
    [255,   0,   0],
    [  0, 255,   0],
    [  0,   0, 255],
];

fn img(grid: &Vec<Vec<usize>>) -> std::io::Result<()> {
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
    let mut grid = vec![vec![0; N]; N];
    let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();

    grid[N/2][N/2] = S;
    to_check.push_back((N/2, N/2));

    while let Some((x, y)) = to_check.pop_front() {
        if grid[x][y] >= C {
            let to_add = grid[x][y] / C;
            grid[x][y] %= C;

            let neighbors: [(isize, isize); 4] = [
                ( 0,  1),
                ( 0, -1),
                ( 1,  0),
                (-1,  0),
            ];
            
            for (dx, dy) in neighbors.iter() {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                grid[nx][ny] += to_add;
                if ny == N/2 || ny == nx {
                    if (nx == N/2 && ny == nx) || (x != N/2 && y != N/2) {
                        grid[nx][ny] += to_add;
                    }
                }
                
                if grid[nx][ny] >= C && ny >= N/2 && ny <= nx {
                    to_check.push_back((nx, ny));
                }
            }
        }
    }

    let _ = img(&grid);
    Ok(())
}
