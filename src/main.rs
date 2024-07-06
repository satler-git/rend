#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;
#[cfg(target_arch = "wasm32")]
use web_time::Instant;

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

#[cfg(target_arch = "wasm32")]
impl MyApp {
    /// Called once before the first frame.
    pub fn new() -> Self {
        Default::default()
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

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Rend", options, Box::new(|cc| {
        let style = egui::Style {
            visuals: egui::Visuals::dark(),
            ..Style::default()
        };
        cc.egui_ctx.set_style(style);
        Box::<MyApp>::default()))
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| {
                    let style = egui::Style {
                            visuals: egui::Visuals::dark(),
                            ..Style::default()
                    };
                    cc.egui_ctx.set_style(style);
                    Box::new(MyApp::new())
                }),
            )
            .await;

        // Remove the loading text and spinner:
        let loading_text = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("loading_text"));
        if let Some(loading_text) = loading_text {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
