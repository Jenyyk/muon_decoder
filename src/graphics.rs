use crate::decoder::{PartType, Particle};
use eframe::egui::{self, Align, ColorImage, Layout, RichText};
use std::collections::HashMap;

const PADDING: f32 = 0.05;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Mode {
    Single,
    Combined,
}

impl Mode {
    fn toggle(&self) -> Self {
        match self {
            Mode::Single => Mode::Combined,
            Mode::Combined => Mode::Single,
        }
    }
}

pub struct MatrixApp {
    matrix: Vec<Vec<f32>>,
    tracks: Vec<Particle>,
    scale: usize,
    current_track: usize,
    image: ColorImage,
    needs_update: bool,
    current_mode: Mode,
}

impl MatrixApp {
    pub fn new(matrix: Vec<Vec<f32>>, tracks: Vec<Particle>, scale: usize) -> Self {
        let mut app = Self {
            matrix,
            tracks,
            scale,
            current_track: 0,
            image: ColorImage {
                size: [1, 1],
                pixels: vec![],
            },
            needs_update: true,
            current_mode: Mode::Combined,
        };
        app.update_image();
        app
    }

    /// Update the image for current track or combined tracks
    fn update_image(&mut self) {
        let size_x = self.matrix.len();
        let size_y = self.matrix[0].len();
        let img_x = size_x * self.scale;
        let img_y = size_y * self.scale;
        let mut pixels = vec![egui::Color32::BLACK; img_x * img_y];

        if self.tracks.is_empty() {
            self.image = ColorImage {
                size: [img_x, img_y],
                pixels,
            };
            return;
        }

        let tracks_to_draw: Vec<Vec<(usize, usize)>> = match self.current_mode {
            Mode::Single => vec![self.tracks[self.current_track].get_track()],
            Mode::Combined => self.tracks.iter().map(|p| p.get_track()).collect(),
        };

        for track_cells in tracks_to_draw {
            let color = egui::Color32::WHITE;
            for (x, y) in track_cells {
                for dx in 0..self.scale {
                    for dy in 0..self.scale {
                        let px = x * self.scale + dx;
                        let py = y * self.scale + dy;
                        if px < img_x && py < img_y {
                            pixels[px * img_y + py] = color;
                        }
                    }
                }
            }
        }

        self.image = ColorImage {
            size: [img_x, img_y],
            pixels,
        };
    }
}

impl eframe::App for MatrixApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        use egui::Key;

        // Track navigation
        if ctx.input(|i| i.key_pressed(Key::ArrowRight))
            && !self.tracks.is_empty()
            && self.current_mode == Mode::Single
        {
            self.current_track = (self.current_track + 1) % self.tracks.len();
            self.needs_update = true;
        }
        if ctx.input(|i| i.key_pressed(Key::ArrowLeft))
            && !self.tracks.is_empty()
            && self.current_mode == Mode::Single
        {
            if self.current_track == 0 {
                self.current_track = self.tracks.len() - 1;
            } else {
                self.current_track -= 1;
            }
            self.needs_update = true;
        }

        // Mode toggle
        if ctx.input(|i| i.key_pressed(Key::M)) {
            self.current_mode = self.current_mode.toggle();
            self.needs_update = true;
        }

        // Refresh image if needed
        if self.needs_update {
            self.update_image();
            self.needs_update = false;
        }

        egui::SidePanel::right("data").show(ctx, |ui| {
            let mut count: HashMap<PartType, usize> = HashMap::new();
            count.insert(PartType::ALPHA, 0);
            count.insert(PartType::BETA, 0);
            count.insert(PartType::GAMMA, 0);
            count.insert(PartType::MUON, 0);
            count.insert(PartType::UNKNOWN, 0);
            for particle in &self.tracks {
                *count
                    .get_mut(&particle.particle_type(&self.matrix))
                    .unwrap() += 1;
            }
            ui.vertical(|ui| {
                egui::Grid::new("Grid")
                    .num_columns(2)
                    .striped(true)
                    .min_col_width(150.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("alphas:").size(16.0).strong());
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(count.get(&PartType::ALPHA).unwrap().to_string());
                        });
                        ui.end_row();

                        ui.label("betas:");
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(count.get(&PartType::BETA).unwrap().to_string());
                        });
                        ui.end_row();

                        ui.label("gammas:");
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(count.get(&PartType::GAMMA).unwrap().to_string());
                        });
                        ui.end_row();

                        ui.label("muons:");
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(count.get(&PartType::MUON).unwrap().to_string());
                        });
                        ui.end_row();

                        ui.label("unknown:");
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(count.get(&PartType::UNKNOWN).unwrap().to_string());
                        });
                        ui.end_row();
                    });
            });
        });
        // --- Central image panel ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture =
                ui.ctx()
                    .load_texture("track_image", self.image.clone(), Default::default());
            ui.image(&texture);

            ui.horizontal(|ui| {
                if ui.button("Prev Track").clicked() {
                    if self.current_mode == Mode::Single {
                        if self.current_track == 0 {
                            self.current_track = self.tracks.len() - 1;
                        } else {
                            self.current_track -= 1;
                        }
                        self.update_image();
                    }
                }
                if ui.button("Next Track").clicked() {
                    if self.current_mode == Mode::Single {
                        self.current_track = (self.current_track + 1) % self.tracks.len();
                        self.update_image();
                    }
                }
                if ui.button("Switch Mode").clicked() {
                    self.current_mode = self.current_mode.toggle();
                    self.update_image();
                }
            });

            ui.label(format!(
                "Track {}/{}",
                self.current_track + 1,
                self.tracks.len()
            ));
            ui.label(match self.current_mode {
                Mode::Single => format!(
                    "Mode: Single Track, current particle type: {:?}",
                    self.tracks[self.current_track].particle_type(&self.matrix)
                ),
                Mode::Combined => "Mode: Combined Tracks".to_string(),
            });
        });
    }
}
