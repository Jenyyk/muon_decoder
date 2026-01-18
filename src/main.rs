mod decoder;
mod graphics;
mod particle_extractor;

use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
const SIZE: usize = 256;

fn main() -> eframe::Result<()> {
    let grid: Vec<Vec<f32>> = vec![vec![0.0; SIZE]; SIZE];

    let tracks: Vec<decoder::Particle> = Vec::new();

    // graphics
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "256x256 Matrix Viewer",
        options,
        Box::new(move |_cc| Box::new(graphics::MatrixApp::new(grid, tracks, 2))),
    )
}

pub fn read_lines<P>(filename: P) -> Result<Vec<Vec<f32>>, std::io::Error>
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
