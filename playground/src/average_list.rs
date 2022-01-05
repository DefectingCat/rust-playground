pub struct AverageList {
    list: Vec<i32>,
    average: f64,
}

impl AverageList {
    pub fn new() -> Self {
        AverageList {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.average = self.update()
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.average = self.update();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update(&self) -> f64 {
        let sum: i32 = self.list.iter().sum();
        let result = sum as f64 / self.list.len() as f64;
        result
    }
}
