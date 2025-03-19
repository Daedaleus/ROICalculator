use calamine::{Data, DataType};
use crate::model::data_point::DataPoint;
use crate::model::results::RoiResult;

const OFFSET: usize = 3;

#[derive(Clone)]
pub struct Roi {
    data_points: Vec<DataPoint>,
    results: Vec<RoiResult>
}

impl Roi {

    pub fn from_excel(column_number: usize, data: &calamine::Range<Data>) -> Self {
        let mut data_points = Vec::new();

        for row in data.rows().skip(OFFSET) {
            if let (Some(x), Some(y)) = (row.get(0), row.get(column_number)) {
                if let (Some(x), Some(y)) = (x.get_float(), y.get_float()) {
                    data_points.push(DataPoint { time: x, value: y });
                }
            }
        }
        Roi {
            data_points,
            results: Vec::new()
        }
    }
    pub fn new(data_points: Vec<DataPoint>) -> Roi {
        Roi {
            data_points,
            results: Vec::new()
        }
    }
    
    fn get_max_inbetween(self, start: usize, end: usize) -> f64 {
        let mut max_value = 0.0;
        let start = start + OFFSET;
        let end = end + OFFSET;
        for i in start..end {
            if self.data_points[i].value() > max_value {
                max_value = self.data_points[i].value()
            }
        };
        max_value
    }

    fn calculate_baseline(self, row_number: usize) -> f64 {
        let mut baseline = 0.0;
        let start = row_number - 40;
        for i in start..row_number {
            baseline += self.data_points[i].value();
        }
        baseline / 40.0
    }

    pub fn add_result(&mut self, start: usize, end: usize) {
        let peak = self.clone().get_max_inbetween(start, end);
        let baseline = self.clone().calculate_baseline(start);
        self.results.push(
            RoiResult::new(peak, baseline).unwrap()
        )
    }

    pub fn get_results(&self) -> &Vec<RoiResult> {
        &self.results
    }


}