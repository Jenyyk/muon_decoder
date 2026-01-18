use geo::{Area, ConvexHull, EuclideanLength};
use geo_types::{Coord, MultiPoint};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartType {
    ALPHA,
    BETA,
    GAMMA,
    MUON,
    UNKNOWN,
}
use std::cell::RefCell;

pub struct Particle {
    track: Vec<(usize, usize)>,
    total_energy_cache: RefCell<Option<f32>>,
    roundness_cache: RefCell<Option<f32>>,
    winding_cache: RefCell<Option<f32>>,
    part_type_cache: RefCell<Option<PartType>>,
}

impl Particle {
    pub fn new(track: Vec<(usize, usize)>) -> Self {
        Particle {
            track,
            total_energy_cache: RefCell::new(None),
            roundness_cache: RefCell::new(None),
            winding_cache: RefCell::new(None),
            part_type_cache: RefCell::new(None),
        }
    }

    pub fn get_track(&self) -> Vec<(usize, usize)> {
        self.track.clone()
    }
    pub fn size(&self) -> usize {
        self.track.len()
    }

    pub fn total_energy(&self, grid: &Vec<Vec<f32>>) -> f32 {
        if let Some(val) = *self.total_energy_cache.borrow() {
            return val;
        }

        let energy: f32 = self
            .track
            .iter()
            .map(|&(x, y)| grid[x as usize][y as usize])
            .sum();

        *self.total_energy_cache.borrow_mut() = Some(energy);
        energy
    }

    pub fn max_energy(&self, grid: &Vec<Vec<f32>>) -> f32 {
        self.track
            .iter()
            .map(|&(x, y)| grid[x as usize][y as usize])
            .fold(0.0, |acc, val| acc.max(val))
    }

    pub fn avg_energy(&self, grid: &Vec<Vec<f32>>) -> f32 {
        self.total_energy(grid) / self.size() as f32
    }

    pub fn roundness(&self) -> f32 {
        if let Some(val) = *self.roundness_cache.borrow() {
            return val;
        }

        let val = roundness(&self.track); // CALL YOUR HELPER HERE
        *self.roundness_cache.borrow_mut() = Some(val);
        val
    }

    pub fn winding(&self) -> f32 {
        if let Some(val) = *self.winding_cache.borrow() {
            return val;
        }

        let val = winding_of_path(&self.track).abs(); // CALL YOUR HELPER HERE
        *self.winding_cache.borrow_mut() = Some(val);
        val
    }

    pub fn particle_type(&self, grid: &Vec<Vec<f32>>) -> PartType {
        if let Some(pt) = *self.part_type_cache.borrow() {
            return pt;
        }

        let pt = match self.size() {
            0..4 => return PartType::GAMMA,
            4..50 => {
                if self.max_energy(grid) < 150.0 && self.avg_energy(grid) < 40.0 {
                    if self.winding() < 1.0 {
                        PartType::BETA
                    } else {
                        PartType::BETA
                    }
                } else if self.max_energy(grid) > 100.0 {
                    if self.roundness() > 0.4 {
                        PartType::ALPHA
                    } else {
                        PartType::UNKNOWN
                    }
                } else {
                    PartType::UNKNOWN
                }
            }
            50.. => {
                if self.max_energy(grid) < 100.0 && self.avg_energy(grid) < 40.0 {
                    if self.winding() > 1.0 {
                        PartType::BETA
                    } else {
                        PartType::MUON
                    }
                } else if self.max_energy(grid) < 100.0 {
                    PartType::UNKNOWN
                } else if self.roundness() > 0.4 {
                    PartType::ALPHA
                } else {
                    PartType::UNKNOWN
                }
            }
        };

        *self.part_type_cache.borrow_mut() = Some(pt);
        pt
    }
}

fn roundness(points: &[(usize, usize)]) -> f32 {
    let mp: MultiPoint<f64> = points
        .iter()
        .map(|&(x, y)| Coord {
            x: x as f64,
            y: y as f64,
        })
        .collect();

    let hull = mp.convex_hull();

    let area = hull.unsigned_area();
    let perimeter = hull.exterior().euclidean_length();

    (4.0 * PI * area / (perimeter * perimeter)) as f32
}

fn winding_of_path(points: &[(usize, usize)]) -> f32 {
    if points.len() < 3 {
        return 0.0;
    }

    let mut total = 0.0;

    for i in 1..points.len() - 1 {
        let (x0, y0) = points[i - 1];
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];

        let v1x = x1 as f64 - x0 as f64;
        let v1y = y1 as f64 - y0 as f64;
        let v2x = x2 as f64 - x1 as f64;
        let v2y = y2 as f64 - y1 as f64;

        let cross = v1x * v2y - v1y * v2x;
        let dot = v1x * v2x + v1y * v2y;

        let angle = cross.atan2(dot);
        total += angle;
    }

    (total / (2.0 * PI)) as f32
}
