use crate::ui::application::Input;

#[derive(Clone)]
pub struct Application {
    pub start_frame: usize,
    pub end_frame: usize,
    mean_amplitude: f64,
}

impl Default for Application {
    fn default() -> Application {
        Application {
            start_frame: 1,
            end_frame: 1,
            mean_amplitude: 0.0,
        }
    }
}

impl From<&Input> for Application {
    fn from(input: &Input) -> Self {
        let start_frame = input.start.parse().unwrap_or(0);
        let end_frame = input.end.parse().unwrap_or(0);
        Application { start_frame, end_frame, ..Self::default() }
    }
}

impl Application {
    pub fn get_start(&self) -> usize {
        self.start_frame
    }

    pub fn get_end(&self) -> usize {
        self.end_frame
    }

    pub fn set_amplitude(&mut self, amplitude: f64) {
        self.mean_amplitude = amplitude;
    }

    pub fn set_start_frame(&mut self, start_frame: usize) {
        self.start_frame = start_frame;
    }

    pub fn set_end_frame(&mut self, end_frame: usize) {
        self.end_frame = end_frame;
    }
    
}