#![windows_subsystem = "windows"]

use kalkulator::kalkulator::Kalkulator;
use egui::{IconData, ViewportBuilder};

const IMAGE_DATA: &'static [u8] = include_bytes!("../icons_symbols/icons8-calculator-48.png");

fn main() -> Result<(),eframe::Error> {

    let image = image::load_from_memory(IMAGE_DATA).unwrap().resize_exact(256,256, image::imageops::FilterType::Lanczos3);
    let icon = IconData {
        rgba: image.to_rgba8().to_vec(),
        height: 256,
        width: 256,
    };

    let viewport = ViewportBuilder::default()
        .with_icon(icon)
        .with_min_inner_size(egui::vec2(320.0, 470.0))
        .with_inner_size(egui::vec2(320.0, 470.0))
        .with_taskbar(true);
    
    let options = eframe::NativeOptions {
        centered: true,
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Kalkulator",
        options,
        Box::new(|cc| Box::new(Kalkulator::new(cc))),
    )?;
    Ok(())
}
