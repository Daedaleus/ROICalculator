#[derive(Clone)]
pub struct RoiResult {
    peak: f64,
    baseline: f64,
    normalized_peak: Option<f64>,
    normalized_baseline: Option<f64>,
    amplitude: Option<f64>
}

impl Default for RoiResult {
    fn default() -> RoiResult {
        RoiResult {
            peak: 0.0,
            baseline: 0.0,
            normalized_peak: None,
            normalized_baseline: None,
            amplitude: None
        }
    }
}

impl RoiResult {
    pub fn new(peak: f64, baseline: f64) -> Result<RoiResult, String> {
        let normalized_peak = peak / baseline * 100.0;
        let normalized_baseline = baseline * 100.0;
        let amplitude = normalized_peak / normalized_baseline;
        Ok(
            RoiResult {
                peak,
                baseline,
                normalized_peak: Some(normalized_peak),
                normalized_baseline: Some(normalized_baseline),
                amplitude: Some(amplitude)
            }
        )
    }
    
    pub fn get_amplitude(&self) -> Option<f64> {
        self.amplitude
    }
    
}