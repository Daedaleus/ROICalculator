mod model;
mod ui;

use crate::ui::application::{AppOptions, RoiCalculator};

fn main() -> eframe::Result {
    env_logger::init();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    eframe::run_native(
        &format!("ROI Calculator {}", VERSION),
        AppOptions::default().get_native_options(),
        Box::new(|_cc| Ok(Box::<RoiCalculator>::default())),
    )
}
