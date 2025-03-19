use crate::model::application::Application;
use crate::model::data_set::DataSet;
use crate::model::roi::Roi;
use calamine::{open_workbook, Reader, Xlsx};
use eframe::egui;
use native_dialog::FileDialog;
use std::default::Default;
use std::error::Error;
use std::path::PathBuf;
use std::vec::Vec;

#[derive(Default, Clone)]
pub struct Input {
    pub start: String,
    pub end: String,
}

pub struct AppOptions {
    height: f32,
    width: f32,
}

impl Default for AppOptions {
    fn default() -> Self {
        AppOptions {
            height: 320.0,
            width: 320.0,
        }
    }
}

impl AppOptions {
    pub fn get_native_options(self) -> eframe::NativeOptions {
        let viewport = egui::ViewportBuilder::default().with_inner_size([self.width, self.height]);
        eframe::NativeOptions {
            viewport,
            ..Default::default()
        }
    }
}

pub struct RoiCalculator {
    file: PathBuf,
    application_count: usize,
    applications: Vec<Application>,
    inputs: Vec<Input>,
}

impl Default for RoiCalculator {
    fn default() -> RoiCalculator {
        RoiCalculator {
            file: "".into(),
            application_count: 1,
            applications: vec![],
            inputs: vec![Input::default()],
        }
    }
}

impl eframe::App for RoiCalculator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ROI Calculator");
            ui.label(self.file.to_str().unwrap().to_string());
            if ui.button("Datei laden").clicked() {
                let path = FileDialog::new().show_open_single_file();
                match path {
                    Ok(path) => match path {
                        None => {
                            todo!("What happens if no path given")
                        }
                        Some(path) => self.file = path,
                    },
                    Err(_) => {
                        todo!("What happens if an error occurs")
                    }
                }
            };
            if ui.button("Reset").clicked() {
                self.file = "".into();
                self.inputs.clear();
                self.application_count = 1;
                self.applications.clear();
            }
            ui.add(
                egui::Slider::new(&mut self.application_count, 1..=20).text("Application count"),
            );

            ui.separator();

            match self.inputs.len().cmp(&self.application_count) {
                std::cmp::Ordering::Less => {
                    self.inputs.resize(self.application_count, Input::default())
                }
                std::cmp::Ordering::Greater => self.inputs.truncate(self.application_count),
                std::cmp::Ordering::Equal => {}
            }

            for (index, input) in self.inputs.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let label_start = ui.label(format!("{} Start: ", index + 1));
                    ui.text_edit_singleline(&mut input.start)
                        .labelled_by(label_start.id);
                });
                ui.horizontal(|ui| {
                    let label_end = ui.label(format!("{} End: ", index + 1));
                    ui.text_edit_singleline(&mut input.end)
                        .labelled_by(label_end.id);
                });
            }

            ui.separator();

            if ui.button("Auswerten").clicked() {
                for input in self.inputs.iter() {
                    self.applications.push(input.into());
                }
                let final_path = self.file.clone();
                let rois = read_excel(final_path).unwrap();
                let applications = self.applications.clone();
                let data_set = DataSet { applications, rois };
                data_set.do_applications();
                todo!("Export der Daten")
            }
        });
    }
}

fn read_excel(path: PathBuf) -> Result<Vec<Roi>, Box<dyn Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path).unwrap();
    let sheet = workbook.worksheet_range("Input").unwrap();

    let column_count = sheet.start().unwrap_or((0, 0)).1;
    let mut rois = vec![];

    for col in 2..=column_count {
        rois.push(Roi::from_excel(col as usize, &sheet));
    }

    Ok(rois)
}
