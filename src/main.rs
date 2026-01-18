mod decoder;
mod graphics;
mod particle_extractor;

use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

use particle_extractor::extract;

const SIZE: usize = 256;

fn main() -> eframe::Result<()> {
    let mut grid: Vec<Vec<f32>> = vec![vec![0.0; SIZE]; SIZE];
    let mut id_map: Vec<Vec<usize>> = vec![vec![0; SIZE]; SIZE];

    grid = match read_lines("./test.txt") {
        Ok(grid) => grid,
        Err(e) => panic!("{}", e),
    };

    let tracks: Vec<decoder::Particle> = extract(&grid, &mut id_map, 2)
        .iter()
        .map(|(_, track)| decoder::Particle::new(track.clone()))
        .collect();

    // graphics
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "256x256 Matrix Viewer",
        options,
        Box::new(move |_cc| Box::new(graphics::MatrixApp::new(grid, tracks, 2))),
    )
}

fn read_lines<P>(filename: P) -> Result<Vec<Vec<f32>>, std::io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut grid: Vec<Vec<f32>> = Vec::with_capacity(SIZE);

    for line_result in lines {
        let line = line_result?;
        let row: Vec<f32> = line
            .split_whitespace()
            .map(|val| {
                val.parse::<f32>()
                    .map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))
            })
            .collect::<Result<Vec<f32>, _>>()?;

        grid.push(row);
    }

    Ok(grid)
}
