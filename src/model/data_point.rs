#[derive(Clone)]
pub struct DataPoint {
    pub(crate) time: f64,
    pub(crate) value: f64,
}

impl DataPoint {
    pub fn value(&self) -> f64 {
        self.value
    }
}