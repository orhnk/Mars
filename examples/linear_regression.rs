use std::ops::AddAssign;

struct DataPoint {
    x: f64,
    y: f64,
}

struct Model {
    w: f64,
    b: f64,
}

impl Model {
    fn predict(&self, x: f64) -> f64 {
        leaky_relu(self.w * x + self.b)
    }

    fn train(&mut self, data: &[DataPoint], learning_rate: f64) {
        for datapoint in data {
            let prediction = self.predict(datapoint.x);
            let error = mean_squared_error(prediction, datapoint.y);
            let error_derivative = error;
            self.w -= error_derivative * datapoint.x * learning_rate;
            self.b -= error_derivative * learning_rate;
        }
    }
}

fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.01 * x
    }
}

fn mean_squared_error(prediction: f64, target: f64) -> f64 {
    0.5 * (prediction - target).powi(2)
}

fn main() {
    let mut model = Model { w: 0.0, b: 0.0 };

    let data = vec![
        DataPoint { x: 1.0, y: 1. },
        DataPoint { x: 2.0, y: 4.0 },
        DataPoint { x: 3.0, y: 9.0 },
        DataPoint { x: 4.0, y: 16.0 },
    ];

    let learning_rate = 0.1;

    model.train(&data, learning_rate);

    let final_prediction = model.predict(5.0);
    println!("Final prediction: {}", final_prediction);
}
