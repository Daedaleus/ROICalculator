use crate::model::application::Application;
use crate::model::roi::Roi;

pub struct DataSet {
    pub(crate) rois: Vec<Roi>,
    pub(crate) applications: Vec<Application>,
}

impl DataSet {
    pub fn add_application(&mut self, application: Application) -> Result<(), &'static str> {
        self.applications.push(application);
        Ok(())
    }

    pub fn do_applications(mut self) {
        for mut application in self.applications {
            for roi in &mut self.rois {
                roi.add_result(application.get_start(), application.get_end())
            }
            let mut mean_amplitude = 0.0;
            for roi in self.rois.clone() {
                mean_amplitude += roi.get_results().last().unwrap().get_amplitude().unwrap();
            }
            application.set_amplitude(mean_amplitude);
        }
    }
}
