mod model;
mod ui;

use crate::ui::application::{AppOptions, RoiCalculator};

fn main() -> eframe::Result {
    env_logger::init();

    eframe::run_native(
        "ROI Calculator",
        AppOptions::default().get_native_options(),
        Box::new(|_cc| Ok(Box::<RoiCalculator>::default())),
    )
}
