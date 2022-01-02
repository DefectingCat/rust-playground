pub struct AverageList {
    list: Vec<i32>,
    average: f64,
}

impl AverageList {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            average: 0.0,
        }
    }
}

impl AverageList {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update();
    }

    pub fn update(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
