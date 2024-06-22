#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::time::Instant;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Rend", options, Box::new(|_cc| Box::<MyApp>::default()))
}

struct MyApp {
    pushed: i64,
    v: bool,
    started: bool,
    start_time: Option<Instant>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            pushed: 0,
            v: false, // 前回vだったか
            started: false,
            start_time: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.started {
                ui.heading("Press space to start");
                if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                    if self.start_time.is_none() {
                        // 時間を初期化
                        self.start_time = Some(Instant::now());
                    }
                    self.started = true;
                }
            } else {
                ctx.request_repaint();
                if ctx.input(|i| i.key_pressed(egui::Key::V)) {
                    // V
                    if !self.v {
                        self.pushed += 1;
                        self.v = true;
                    }
                } else if ctx.input(|i| i.key_pressed(egui::Key::B)) {
                    // B
                    if self.v {
                        self.pushed += 1;
                        self.v = false;
                    }
                } else {
                    if self.v {
                        // (b: 黄色, v: 黒)
                        let v = egui::RichText::new("V").color(egui::Color32::WHITE);
                        let b = egui::RichText::new("B").color(egui::Color32::YELLOW);
                        ui.heading(v);
                        ui.heading(b);
                    } else {
                        // (b: 黒, v: 黄色)
                        let v = egui::RichText::new("V").color(egui::Color32::YELLOW);
                        let b = egui::RichText::new("B").color(egui::Color32::WHITE);
                        ui.heading(v);
                        ui.heading(b);
                    }
                }
                if let Some(start_time) = self.start_time {
                    let elapsed = start_time.elapsed();
                    let rate = format!("{:.2}", self.pushed as f64 / elapsed.as_secs_f64());
                    ui.heading(format!("pushes: {}, rate: {}", self.pushed, rate));
                }
                // レートを計算し出力
            }
        });
    }
}
